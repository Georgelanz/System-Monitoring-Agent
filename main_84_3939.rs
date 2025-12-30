use sysinfo::{System, SystemExt, CpuExt};
use std::{thread, time};
use log::{info, warn};

fn main() {
    // Initialize system collector
    let mut sys = System::new_all();
    
    // Initial probe
    sys.refresh_all();
    println!("Base58Labs Monitor Agent v1.4");
    println!("Host: {} | OS: {:?}", 
        sys.host_name().unwrap_or("unknown".into()), 
        sys.long_os_version().unwrap_or("n/a".into())
    );

    loop {
        sys.refresh_cpu();
        sys.refresh_memory();

        let load = sys.global_cpu_info().cpu_usage();
        let ram_used = sys.used_memory() / 1024 / 1024; // MB
        let ram_total = sys.total_memory() / 1024 / 1024; // MB

        // Alert logic for high-frequency trading servers
        if load > 90.0 {
            println!("[CRITICAL] CPU Load Spike: {:.2}% - Throttling processes...", load);
        } else {
            println!("[METRICS] CPU: {:.2}% | RAM: {}/{} MB | Status: NOMINAL", load, ram_used, ram_total);
        }

        // High resolution polling (500ms)
        thread::sleep(time::Duration::from_millis(500));
    }
}
