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
	White
}

impl fmt::Display for Colors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "\x1b[3{}m", *self as u8)
    }
}