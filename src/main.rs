use std::env;
use clokwerk::{Scheduler, TimeUnits};
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running backuper v{}", env!("CARGO_PKG_VERSION"));

    // Initialize config and ensure destination exists
    let config = backuper::config::init()?;
    backuper::ensure_destination(&config.destination, false);

    // Make scheduler
    let mut scheduler = Scheduler::new();
    let interval_minutes = config.interval_minutes;

    // First run backup?
    // println!("Running initial backup.");
    // backuper::do_backup(&config);
    // backuper::cleanup_old_backups(&config);

    println!("Starting backup scheduler (every {} minutes).", interval_minutes);
    scheduler.every((interval_minutes).minutes()).run(move || {
        println!("Running backup");

        // do the backup
        let _ = backuper::do_backup(&config);

        // remove old backups
        let _ = backuper::cleanup_old_backups(&config);
    });

    loop {
        // Run pending jobs
        scheduler.run_pending();

        // go to sleep
        thread::sleep(std::time::Duration::from_secs(60));
    }
}