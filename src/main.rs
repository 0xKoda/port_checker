use std::process::Command;
use std::str;

fn main() {
    let output = Command::new("lsof")
        .arg("-i")
        .arg("-P")
        .arg("-n")
        .output()
        .expect("Failed to execute lsof command");

    let output_str = str::from_utf8(&output.stdout).expect("Failed to convert output to string");

    println!("Ports in use and their associated programs:\n{}", output_str);
}
