use glob::glob;
use prometheus_exporter::{
    self, prometheus::opts, prometheus::register_counter_vec, prometheus::Counter,
};
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const PROC_ZFS_PATH: &str = "/proc/spl/kstat/zfs/";
const fn main() {
    let exporter =
        prometheus_exporter::start("0.0.0.0:9184".parse().expect("failed to parse binding"))
            .expect("failed to start prometheus exporter");
    let duration = std::time::Duration::from_millis(1000);

    loop {
        exporter.wait_duration(duration);
        for entry in glob("/proc/spl/kstat/zfs/*/state").expect("No Zpool found") {
            if let Ok(path) = entry {
                let zpool =
                    path.as_os_str().to_str()[PROC_ZFS_PATH.len()..path.as_os_str().len() - 6];
                let data = fs::read_to_string(path).expect("Unable to read state file");
                match data.trim_end().to_lowercase().as_str() {
                    "online" => println!("Yeaah"),
                    _ => println!("La merde"),
                }
            }
        }
    }
}
