use bunnyfetch::*;

fn main() {
	// TODO: make this cleaner (posible use of a macro perhaps?)
	printr!("	    {}{}", Colors::Red, title());
	printlnr!("	   {} OS {}{}", Colors::Green, Colors::Reset, os().unwrap());
	printlnr!("   (\\ /)   {} Kernel {}{}", Colors::Yellow, Colors::Reset, kernel().unwrap());
	printlnr!("   ( . .)  {} DE {}{}", Colors::Blue, Colors::Reset, de().unwrap());
	printlnr!("   c({}\"{})({0}\"{1}) {} WM {1}{}", Colors::Red, Colors::Reset, Colors::Magenta, kernel().unwrap());
}