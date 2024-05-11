use std::env;
use std::fs;
use std::process::Command;

// Function to execute shell commands
fn execute_command(command: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn main() {
    let mem_info = fs::read_to_string("/proc/meminfo").expect("Failed to read /proc/meminfo");
    let mut mem_total = 0;
    let mut mem_avail = 0;

    for line in mem_info.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "MemTotal:" => mem_total = parts[1].parse::<i32>().expect("Failed to parse MemTotal"),
            "MemAvailable:" => {
                mem_avail = parts[1]
                    .parse::<i32>()
                    .expect("Failed to parse MemAvailable")
            }
            _ => (),
        }
    }

    let mem_used = (mem_total - mem_avail) / 1024;
    let mem_total = mem_total / 1024;
    let memory = format!("{}MiB / {}MiB", mem_used, mem_total);

    let user = env::var("USER").expect("Failed to get USER environment variable");
    let hostname = fs::read_to_string("/etc/hostname")
        .expect("Failed to read /etc/hostname")
        .trim()
        .to_string();
    let shell = env::var("SHELL").expect("Failed to get SHELL environment variable");
    let shell = shell.split('/').last().unwrap();

    let os = execute_command("hostnamectl | grep 'Operating System' | awk '{print $3,$4}'");
    let kernel = execute_command("uname -sr");
    let uptime = execute_command("uptime -p | sed 's/up //'");

    let name = execute_command("cat /sys/devices/virtual/dmi/id/product_name");
    let version = execute_command("cat /sys/devices/virtual/dmi/id/product_version");

    let host = format!("{} {}", name, version);
    let pkgs = execute_command("pacman -Qe | wc -l");

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
