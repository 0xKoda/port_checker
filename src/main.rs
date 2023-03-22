use std::env;
use std::fs;
use std::io::{Write, BufRead, BufReader};
use std::process::Command;
use std::str;

/// Ensures the existence of the file containing programs of questionable intent.
///
/// If the file doesn't exist, it'll be created with a touch of finesse.
fn ensure_suspicious_programs_file_exists() {
    if !fs::metadata("suspicious_programs.txt").is_ok() {
        fs::File::create("suspicious_programs.txt").expect("Failed to create suspicious programs file");
    }
}

/// Adds a program to the list of suspicious programs, no questions asked.
///
/// # Parameters
///
/// * `program_name` - The name of the program that's raising eyebrows.
fn add_suspicious_program(program_name: &str) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("suspicious_programs.txt")
        .expect("Failed to open suspicious programs file");

    writeln!(file, "{}\n", program_name).expect("Failed to write to suspicious programs file");
}

/// Removes a program from the list of suspicious programs, as if it was never there.
///
/// # Parameters
///
/// * `program_name` - The name of the program that's been granted a pardon.
fn remove_suspicious_program(program_name: &str) {
    let file = fs::File::open("suspicious_programs.txt").expect("Failed to open suspicious programs file");
    let reader = BufReader::new(file);

    let updated_programs: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .filter(|line| line != program_name)
        .collect();

    fs::write("suspicious_programs.txt", updated_programs.join("\n")).expect("Failed to write updated suspicious programs file");
}

/// The main function, where the curtain rises and the show begins.
///
/// It ensures the file of questionable programs exists, processes command-line arguments, and
/// calls upon the ancient "lsof" spell to reveal the secrets of the running programs.
fn main() {
    ensure_suspicious_programs_file_exists();

    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        let action = &args[1];
        let program_name = &args[2];

        match action.as_str() {
            "add" => add_suspicious_program(program_name),
            "remove" => remove_suspicious_program(program_name),
            _ => println!("Invalid action. Use 'add' or 'remove'."),
        }
    }

    let suspicious_programs_file = "suspicious_programs.txt";
    let suspicious_programs_content = fs::read_to_string(suspicious_programs_file)
        .expect("Failed to read suspicious programs file");

    let suspicious_programs: Vec<&str> = suspicious_programs_content.lines().collect();

    let output = Command::new("lsof")
        .arg("-i")
        .arg("-P")
        .arg("-n")
        .output()
        .expect("Failed to execute lsof command");

    let output_str = str::from_utf8(&output.stdout).expect("Failed to convert output to string");

    println!("Ports in use and their associated programs:");

    let mut row_count = 0;
    for line in output_str.lines() {
        let row_color = if row_count % 2 == 0 {
            "\x1b[0;37m" // White
        } else {
            "\x1b[0;90m" // Gray
        };

        let program_name = line.split_whitespace().next().unwrap_or("");
        let warning_flag = if suspicious_programs.contains(&program_name) {
            "\x1b[0;31m [WARNING: Suspicious program]\x1b[0m" // Red
        } else {
            ""
        };

        println!("{}{}{}\x1b[0m", row_color, line, warning_flag);
        row_count += 1;
    }
}
