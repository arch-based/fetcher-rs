use std::fs;
use std::env;
use std::process::Command;
use colored::*;

fn main() {
    let mem_info = fs::read_to_string("/proc/meminfo").unwrap();
    let mut mem_total = 0;
    let mut mem_avail = 0;

    for line in mem_info.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "MemTotal:" => mem_total = parts[1].parse::<i32>().unwrap(),
            "MemAvailable:" => mem_avail = parts[1].parse::<i32>().unwrap(),
            _ => (),
        }
    }

    let mem_used = (mem_total - mem_avail) / 1024;
    let mem_total = mem_total / 1024;
    let memory = format!("{}MiB / {}MiB", mem_used, mem_total);

    let user = env::var("USER").unwrap();
    let hostname = fs::read_to_string("/etc/hostname").unwrap().trim().to_string();
    let shell = env::var("SHELL").unwrap();
    let shell = shell.split('/').last().unwrap();

    let output = Command::new("sh")
        .arg("-c")
        .arg("hostnamectl | grep 'Operating System' | awk '{print $3,$4}'")
        .output()
        .unwrap();
    let os = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let output = Command::new("sh")
        .arg("-c")
        .arg("uname -sr")
        .output()
        .unwrap();
    let kernel = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let output = Command::new("sh")
        .arg("-c")
        .arg("uptime -p | sed 's/up //'")
        .output()
        .unwrap();
    let uptime = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let output = Command::new("sh")
        .arg("-c")
        .arg("cat /sys/devices/virtual/dmi/id/product_name")
        .output()
        .unwrap();
    let name = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let output = Command::new("sh")
        .arg("-c")
        .arg("cat /sys/devices/virtual/dmi/id/product_version")
        .output()
        .unwrap();
    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let host = format!("{} {}", name, version);
    let output = Command::new("sh")
        .arg("-c")
        .arg("pacman -Qe | wc -l")
        .output()
        .unwrap();
    let pkgs = String::from_utf8_lossy(&output.stdout).trim().to_string();

    println!("                    \x1B[1m{}@\x1B[1m{}", user.cyan(), hostname.cyan());
    println!("{}","          \x1B[1m-----------------------------".cyan());
    println!("\x1B[1m{} \x1B[1m{}", " OS ~~~~~~~~~~>".cyan(), os);
    println!("\x1B[1m{} \x1B[1m{}", "󰌢 HOST: ~~~~~~~>".cyan(), host);
    println!("\x1B[1m{} \x1B[1m{}", "󰅐 UPTIME: ~~~~~>".cyan(), uptime);
    println!("\x1B[1m{} \x1B[1m{}", " KERNEL: ~~~~~>".cyan(), kernel);
    println!("\x1B[1m{} \x1B[1m{}", " SHELL: ~~~~~~>".cyan(), shell);
    println!("\x1B[1m{} \x1B[1m{}", " PACKAGES: ~~~>".cyan(), pkgs);
    println!("\x1B[1m{} \x1B[1m{}", "󰍛 MEMORY: ~~~~~>".cyan(), memory);    
}
