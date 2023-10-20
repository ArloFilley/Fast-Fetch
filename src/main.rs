use std::{fs, collections::HashMap, path::PathBuf};

use sysinfo::{System, SystemExt, DiskExt, CpuExt};

use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The Ascii Art File
    #[arg(long)]
    file: PathBuf,
}

fn main() {
    let args = Args::parse();
    
    let mut b = 0;

    let (w,_) = term_size::dimensions().unwrap();
    let mut system = System::new_all();
    system.refresh_all();

    let distro_logo = fs::read_to_string(args.file).expect("Couldn't read logo file");    

    let user = std::env::var("USER").unwrap_or("?".to_string());
    let host_name = system.host_name().unwrap_or("?".to_string());
    let os = system.long_os_version().unwrap_or("?".to_string());
    let kernel = system.kernel_version().unwrap_or("?".to_string());
    let uptime = to_time_pretty(system.uptime());
    let shell = std::env::var("SHELL").unwrap_or("?".to_string());
    // let de;
    // let wm;
    let cpu = system.global_cpu_info().brand();
    let used_ram = to_disk_units(system.used_memory(), DiskUnits::RamGB);
    let total_ram = to_disk_units(system.total_memory(), DiskUnits::RamGB);

    let used_disk = to_disk_units(system.disks()[0].total_space() - system.disks()[0].available_space(), DiskUnits::DiskGB);
    let total_disk = to_disk_units(system.disks()[0].total_space(), DiskUnits::DiskGB);

    let mut max = 0;
    let mid = distro_logo.lines().count() / 2;
    let mut map = HashMap::<usize, String>::new();

    map.insert(mid - 6, format!("󱩛 -> {user}@{host_name}"));
    map.insert(mid - 5, format!("󰋘 -> {os}"));
    map.insert(mid - 4, format!("󰌽 -> {kernel}"));
    map.insert(mid - 3, format!("󰖉 -> {uptime}"));
    map.insert(mid - 2, format!("󰞷 -> {shell}"));
    map.insert(mid - 1, format!("󰍹 -> Packages"));
    map.insert(mid + 0, format!("󰍹 -> 'DE'"));
    map.insert(mid + 1, format!("󰍹 -> 'WM'"));
    map.insert(mid + 2, format!("󰍹 -> 'Resolution'"));
    map.insert(mid + 3, format!("󰘚 -> {cpu}"));
    map.insert(mid + 4, format!("󰍹 -> GPU"));
    map.insert(mid + 5, format!("󰍛 -> {used_ram}/{total_ram}"));
    map.insert(mid + 6, format!("󰍛 -> {used_disk}/{total_disk}"));

    for (_, val) in map.iter() {
        if val.len() > max {
            max = val.len();
        }
    }

    for (i, line) in distro_logo.lines().enumerate() {
        if i == 0 {
            max += line.len();
            b = (w - max) / 2;
        }
        if map.contains_key(&i) {
            let mut g = format!("{line} {}",map.get(&i).unwrap());
            for _ in 0..b {
                g.insert(0, ' ');
            }
            println!("{g}");
        } else {
            let mut g = format!("{line}");
            for _ in 0..b {
                g.insert(0, ' ');
            }
            println!("{g}");
        }
    }

    // println!("{distro_logo}");
}

enum DiskUnits {
    DiskGB,
    RamGB
}

fn to_disk_units(base: u64, unit: DiskUnits) -> String {
    let base = base as f64;
    match unit {
        DiskUnits::DiskGB => format!("{:.2}GB", base / 1_000_000_000.0),
        DiskUnits::RamGB => format!("{:.2}GB", base / 1_073_741_824.0)
    }
}

fn to_time_pretty(seconds: u64) -> String {
    if seconds > 3600 {
        format!("{}h:{}m:{}s", seconds / 3600, seconds / 60 % 60, seconds % 60)
    } else if seconds > 60 {
        format!("{}m:{}s", seconds / 60, seconds % 60)
    } else {
        format!("{seconds}s")
    }
}