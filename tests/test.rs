#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::str;

    // Helper function to execute the lsof command and return the output as a string
    fn execute_lsof() -> String {
        let output = Command::new("lsof")
            .arg("-i")
            .arg("-P")
            .arg("-n")
            .output()
            .expect("Failed to execute lsof command");

        str::from_utf8(&output.stdout).expect("Failed to convert output to string").to_string()
    }

    #[test]
    fn test_lsof_execution() {
        let output = execute_lsof();
        assert!(!output.is_empty(), "The output of the lsof command should not be empty");
    }

    #[test]
    fn test_lsof_output_format() {
        let output = execute_lsof();
        let lines: Vec<&str> = output.lines().collect();

        // Check if the output has at least one line (excluding the header)
        assert!(lines.len() > 1, "The output should have at least one line (excluding the header)");

        // Check if the header line contains the expected columns
        let header = lines[0];
        assert!(
            header.contains("COMMAND") && header.contains("PID") && header.contains("USER") && header.contains("FD") && header.contains("TYPE") && header.contains("DEVICE") && header.contains("SIZE/OFF") && header.contains("NODE") && header.contains("NAME"),
            "The header line should contain the expected columns"
        );
    }
}
