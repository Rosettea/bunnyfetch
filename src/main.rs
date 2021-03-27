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
    print!("   c({}\"{})({0}\"{1})", Colors::Red, Colors::Reset);
    print!("  ");
    for pat in 0..=7 {
        print!("\x1B[10{}m  ", pat)
    }
    print!("{}\n            ", Colors::Reset);
    for pat in 0..=7 {
        print!("\x1B[4{}m  ", pat);
    }
    println!()
}
