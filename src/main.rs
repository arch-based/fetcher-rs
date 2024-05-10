use std::env;
use std::fs;
use std::process::Command;

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
    let hostname = fs::read_to_string("/etc/hostname")
        .unwrap()
        .trim()
        .to_string();
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

    let cyan = format!("{}", "\x1b[36m");
    let bold = format!("{}", "\x1b[1m");
    let reset = format!("{}", "\x1b[0m");

    // The output

    println!(
        "{}{}{}{}{}@{}{}{}{}",
        "                    ", cyan, bold, user, reset, cyan, bold, hostname, reset
    );
    println!(
        "{}{}{}{}{}",
        "          ", cyan, bold, "-----------------------------", reset
    );
    println!(
        "{}{}{}{} {}{}{}",
        cyan, bold, " OS ~~~~~~~~~~>", reset, bold, os, reset
    );
    println!(
        "{}{}{}{} {}{}{}",
        cyan, bold, "󰌢 HOST: ~~~~~~~>", reset, bold, host, reset
    );
    println!(
        "{}{}{}{} {}{}{}",
        cyan, bold, "󰅐 UPTIME: ~~~~~>", reset, bold, uptime, reset
    );
    println!(
        "{}{}{}{} {}{}{}",
        cyan, bold, " KERNEL: ~~~~~>", reset, bold, kernel, reset
    );
    println!(
        "{}{}{}{} {}{}{}",
        cyan, bold, " SHELL: ~~~~~~>", reset, bold, shell, reset
    );
    println!(
        "{}{}{}{} {}{}{}",
        cyan, bold, " PACKAGES: ~~~>", reset, bold, pkgs, reset
    );
    println!(
        "{}{}{}{} {}{}{}",
        cyan, bold, "󰍛 MEMORY: ~~~~~>", reset, bold, memory, reset
    );
    println!()
}
