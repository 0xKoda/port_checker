# Suspicious Programs Detector

A Rust-based command-line tool that detects suspicious programs running on your system that are trying to communicate through ports on your pc.

Returns a list of running programs, the port, protocol etc. and allows you to identify and add suspicious programs to a list, so that you can easily identify suspecious network activity on your mac.

## Features

- Maintains a list of suspicious programs in a text file
- Adds and removes programs from the list
- Detects running suspicious programs using the `lsof` command

## Usage

1. Clone the repository and navigate to the project directory.

2. Build the project using Cargo:

```sh
cargo build --release
```

Run the program with the following command:
```sh
cargo run add program_name
```

### Adding a suspicious program

To add a program to the list of suspicious programs, use the following command:
```sh
cargo run add program_name
```
Replace `program_name` with the actual name of the program you want to add to the list.

### Removing a suspicious program

To remove a program from the list of suspicious programs, use the following command:
```sh
cargo run remove program_name
```
Replace `program_name` with the actual name of the program you want to remove from the list.
