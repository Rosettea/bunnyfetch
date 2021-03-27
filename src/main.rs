use bunnyfetch::*;

fn main() {
    // TODO: make this cleaner
    printr!("	    {}{}", Colors::Red, title());
    printlnr!("	   {} OS {}{}", Colors::Green, Colors::Reset, os());
    printlnr!(
        "   (\\ /)   {} Kernel {}{}",
        Colors::Yellow,
        Colors::Reset,
        kernel().trim_end()
    );
    printlnr!("   ( . .)  {} DE {}{}", Colors::Blue, Colors::Reset, de());
    printlnr!("   c({}\"{})({0}\"{1})", Colors::Red, Colors::Reset);
    print!("\n	   ");
    for pat in 0..=7 {
        print!("\x1b[4{}m  ", pat)
    }
    printlnr!()
}
