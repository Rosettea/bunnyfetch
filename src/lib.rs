use std::env::var;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::Command;

pub fn username() -> String {
    // UNWRAP: Handled with or clause
    var("USER").unwrap_or(String::from("Unknown"))
}

// Use /etc/hostname to read hostname. $HOST does not appear to be set when called by rust
#[cfg(target_family = "unix")]
pub fn hostname<'a>() -> String {
    // UNWRAP: /etc/hostname will always exist and readable on unix machines
    let f = File::open("/etc/hostname").unwrap();
    let mut reader = BufReader::with_capacity(20, f);

    let mut line = String::with_capacity(20);
    // UNWRAP: /etc/hostname will always be UTF-8 encoded
    // https://doc.rust-lang.org/std/io/trait.BufRead.html#errors-2
    reader.read_line(&mut line).unwrap();
    line
}

#[cfg(target_family = "windows")]
pub fn hostname() -> Result<String, ()> {
    let output = Command::new("hostname").output();

    match output {
        Ok(output) => return Ok(String::from_utf8(output.stdout).unwrap()),
        Err(_) => return Err(()),
    }
}

#[cfg(target_family = "unix")]
pub fn os() -> String {
    // UNWRAP: /etc/os-release will always exist and readable on unix machines
    let f = File::open("/etc/os-release").unwrap();
    let mut reader = BufReader::with_capacity(50, f);

    let mut line = String::with_capacity(300);
    if let Err(e) = reader.read_to_string(&mut line) {
        panic!("failed to read /etc/os-release: {}", e)
    }

    let split: String = line
        .split('\n')
        .filter(|&x| x.contains("ID"))
        .take(1)
        .collect();

    // UNWRAP: /etc/os-release will always have KEY=VALUE pairs
    let string = split.split('=').nth(1).unwrap().replace("\"", "");

    // To uppercase the first letter
    let mut c = string.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

#[cfg(target_family = "windows")]
pub fn os() -> Result<String, ()> {
    let output = Command::new("wmic")
        .args(&["os", "get", "Caption"])
        .output();

    match output {
        Ok(output_) => {
            let output = String::from_utf8(output_.stdout).unwrap();
            let pat: Vec<&str> = output.split_terminator("\r\r\n").collect();
            let os = pat[1];
            return Ok(os.trim().to_string().split_off(10));
        }
        Err(_) => return Err(()),
    }
}

#[cfg(target_family = "unix")]
pub fn kernel() -> String {
    // UNWRAP: /proc/version will always exist and readable on unix machines
    let f = File::open("/proc/version").unwrap();
    let mut reader = BufReader::with_capacity(20, f);

    let mut line = String::with_capacity(175);
    // UNWRAP: /proc/version will always be UTF-8 encoded
    // https://doc.rust-lang.org/std/io/trait.BufRead.html#errors-2
    reader.read_line(&mut line).unwrap();
    // UNWRAP: /proc/version has the same format with "Linux version <kern-version>" as the 3rd
    // word
    line.split(" ").nth(2).unwrap().to_string()
}

#[cfg(target_family = "windows")]
pub fn kernel() -> Result<String, ()> {
    let output = Command::new("wmic")
        .args(&["os", "get", "Version"])
        .output();

    match output {
        Ok(output_) => {
            let output = String::from_utf8(output_.stdout).unwrap();
            let pat: Vec<&str> = output.split_terminator("\r\r\n").collect();
            let os = pat[1];
            return Ok(os.trim().to_string());
        }
        Err(_) => return Err(()),
    }
}

#[cfg(target_family = "unix")]
pub fn de() -> String {
    // UNWRAP: handled safely with or clause
    var("XDG_SESSION_DESKTOP").unwrap_or(String::from("Unknown"))
}

#[cfg(target_family = "windows")]
pub fn de() -> Result<String, ()> {
    let os = os().unwrap();
    let pat: Vec<&str> = os.split_terminator(" ").collect();
    if pat[1].trim() == "7" {
        Ok("Aero".to_string())
    } else {
        Ok("Metro".to_string())
    }
}
