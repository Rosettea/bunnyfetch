use std::fmt;

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum Colors {
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,
	BrightBlack,
	BrightRed,
	BrightGreen,
	BrightYellow,
	BrightBlue,
	BrightMagenta,
	BrightCyan,
	Reset,
}

impl fmt::Display for Colors {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let code = match *self as u8 {
			0..=7 => format!("\x1b[3{}m", *self as u8),
			8..=14 => format!("\x1b[9{}m", *self as u8),
			15 => "\x1b[0m".to_string(),
			_ => "".to_string() // We should never reach this
		};
		write!(f, "{}", code)
	}
}
