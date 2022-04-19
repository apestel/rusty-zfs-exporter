use glob::glob;
use prometheus_exporter::prometheus::{register_counter, register_gauge};
use prometheus_exporter::{
    self, prometheus::opts, prometheus::register_counter_vec, prometheus::Counter,
};
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let exporter =
        prometheus_exporter::start("0.0.0.0:9184".parse().expect("failed to parse binding"))
            .expect("failed to start prometheus exporter");
    let duration = std::time::Duration::from_millis(1000);
    let reg = register_gauge!("zfs_zpool_state", "ZFS zpool state").unwrap();

    loop {
        exporter.wait_duration(duration);
        for entry in glob("/proc/spl/kstat/zfs/*/state").expect("No Zpool found") {
            if let Ok(path) = entry {
                let s_path: Vec<&str> = path.to_str().unwrap().split('/').collect();
                println!("Pool name: {}", s_path[s_path.len() - 2]);
                let data = fs::read_to_string(path).expect("Unable to read state file");
                match data.trim_end().to_lowercase().as_str() {
                    "online" => reg.set(1.0),
                    _ => reg.set(0.0),
                }
            }
        }
    }
}
