use bunnyfetch::*;

fn main() {
	// TODO: make this cleaner
	printr!("	    {}{}", Colors::Red, title());
	printlnr!("	   {} OS {}{}", Colors::Green, Colors::Reset, os().unwrap());
	printlnr!("   (\\ /)   {} Kernel {}{}", Colors::Yellow, Colors::Reset, kernel().unwrap());
	printlnr!("   ( . .)  {} DE {}{}", Colors::Blue, Colors::Reset, de().unwrap());
	printlnr!("   c({}\"{})({0}\"{1})", Colors::Red, Colors::Reset);
	print!("\n	   ");
	for pat in 0..=7 {
		print!("\x1b[4{}m  ", pat)
	}
	printlnr!()
}