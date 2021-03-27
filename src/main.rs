use bunnyfetch::*;

fn main() {
    // TODO: make this cleaner
    print!("	    {}{}", Colors::Red, title());
    println!("	   {} OS {}{}", Colors::Green, Colors::Reset, os());
    println!(
        "   (\\ /)   {} Kernel {}{}",
        Colors::Yellow,
        Colors::Reset,
        kernel().trim_end()
    );
    println!("   ( . .)  {} DE {}{}", Colors::Blue, Colors::Reset, de());
    println!("   c({}\"{})({0}\"{1})", Colors::Red, Colors::Reset);
    print!("\n	   ");
    for pat in 0..=7 {
        print!("\x1b[4{}m  ", pat)
    }
    println!()
}
