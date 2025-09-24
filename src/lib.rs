use std::fs;
use chrono::{Local, NaiveDateTime, TimeZone};
use fs_extra::{copy_items, dir};

pub mod config;

use std::path::Path;
use config::structs::BackuperConfig;

pub fn ensure_destination(dst: &Path, silent: bool) {
    if !silent {println!("Destination is: {}", dst.display());}

    match dst.exists() {
        true => if !silent {println!("Destination exists.")},
        false => {
            if !silent {println!("Destination does not exist, creating.")};
            std::fs::create_dir_all(dst).unwrap();
        }
    }
}

pub fn do_backup(config: &BackuperConfig) -> Result<(), String> {
    let now = Local::now();
    let ctime = now.format(get_time_mark_format()).to_string();
    println!("[BEGIN] Performing backup (current time {}).", &ctime);

    for (cat, paths) in &config.backup_sources {
        println!("Backing up: {}.", cat);

        // ensure destination exists
        let dst = Path::new(&config.destination).join(format!("Back-up {}", &ctime)).join(cat);
        ensure_destination(&dst, true);

        // backup
        let options = dir::CopyOptions::new();
        for path in paths {
            match copy_items(&[path], &dst, &options) {
                Err(err) => {
                    // skip non existent files/dirs
                    println!("Error copying: {}", err);
                },
                _ => ()
            }
        }
    }

    println!("[END] Backup completed.");
    Ok(())
}

fn get_time_mark_format() -> &'static str {
    "%d.%m.%Y %H-%M-%S"
}

pub fn cleanup_old_backups(config: &BackuperConfig) -> std::io::Result<()> {
    let now = Local::now();
    println!("Cleaning up old backups.");

    for entry in fs::read_dir(&config.destination)? {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Skipping entry: {}", e);
                continue;
            }
        };

        // skip non-UTF 8 names etc
        let name = match entry.file_name().into_string() {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Skipping non-UTF-8 name");
                continue;
            }
        };

        // strip "Back-up " prefix
        let Some(ts) = name.strip_prefix("Back-up ") else { continue };

        // try-parse the time mark
        match NaiveDateTime::parse_from_str(ts, get_time_mark_format()) {
            Ok(naive) => {
                // Interpret as local time
                match Local.from_local_datetime(&naive) {
                    chrono::LocalResult::Single(dt) => {
                        println!("Parsed time mark OK: {}", dt);

                        if dt.date_naive() < now.checked_sub_days(chrono::Days::new(config.keep_days)).unwrap().date_naive() {
                            println!("Deleting old backup: {}", name);
                            fs::remove_dir_all(entry.path())?;
                        }
                    },
                    _ => ()
                }
            }
            Err(err) => {
                eprintln!("Couldn't parse '{}' with '{}': {}", ts, get_time_mark_format(), err);
            }
        }
    }

    Ok(())
}