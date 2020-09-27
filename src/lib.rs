use std::fmt;
use std::env::var;
use std::process::Command;

pub mod colors;
pub use crate::colors::Colors;

#[derive(Debug)]
pub struct Title {
	username: String,
	hostname: String
}

pub fn username() -> String {
	var("USERNAME").unwrap()
}

#[cfg(target_family = "unix")]
pub fn hostname() -> Result<String, ()> {
	Ok(var("HOSTNAME").unwrap())
}

#[cfg(target_family = "windows")]
pub fn hostname() -> Result<String, ()> {
	let output = Command::new("hostname")
		.output();

	match output {
		Ok(output) => return Ok(String::from_utf8(output.stdout).unwrap()),
		Err(_) => return Err(()),
	}
}

pub fn title() -> Title {
	let username = username();
	let hostname = hostname().unwrap_or("Unknown".to_string());

	Title {
		username,
		hostname
	}
}

#[cfg(target_family = "unix")]
pub fn os() -> Result<String, ()> {
	let output = Command::new("lsb_release")
		.arg("-sd")
		.output();

	match output {
		Ok(output) => return Ok(String::from_utf8(output.stdout).unwrap()),
		Err(_) => return Err(()),
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
			return Ok(os.trim().to_string().split_off(10))
		},
		Err(_) => return Err(()),
	}
}

#[cfg(target_family = "unix")]
pub fn kernel() -> Result<String, ()> {
	let output = Command::new("uname")
		.arg("-r")
		.output();

	match output {
		Ok(output) => return Ok(String::from_utf8(output.stdout).unwrap()),
		Err(_) => return Err(()),
	}
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
			return Ok(os.trim().to_string())
		},
		Err(_) => return Err(()),
	}
}

#[cfg(target_family = "unix")]
pub fn de() -> Result<String, ()> {
	"Unknown".to_string()
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

impl fmt::Display for Title {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}@{}", self.username, self.hostname)
	}
}

#[macro_export]
macro_rules! printr {
	() => (print!("\x1b[0m"));
	($($arg:tt)*) => ({
		print!("{}\x1b[0m", format_args!($($arg)*));
	})
}

#[macro_export]
macro_rules! printlnr {
	() => (println!("\x1b[0m"));
	($($arg:tt)*) => ({
		println!("{}\x1b[0m", format_args!($($arg)*));
	})
}