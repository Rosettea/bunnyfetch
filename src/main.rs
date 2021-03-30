pub mod colors;
mod lib;

use colors::Colors;
use lib::*;

fn main() {
    // TODO: make this cleaner
    print!("	    {}{}@{}", Colors::Red, username(), hostname());
    println!("	   {} OS {}{}", Colors::Green, Colors::Reset, os());
    println!(
        "   (\\ /)   {} Kernel {}{}",
        Colors::Yellow,
        Colors::Reset,
        kernel().trim_end()
    );
    println!(
        "   ( . .)  {} Shell {}{}",
        Colors::Blue,
        Colors::Reset,
        shell()
    );
    print!(
        "   c({}\"{})({0}\"{1}) {} WM {}{}",
        Colors::Red,
        Colors::Reset,
        Colors::Red,
        Colors::Reset,
        de()
    );
    print!("{}\n            ", Colors::Reset);
    for pat in 0..=7 {
        print!("\x1B[10{}m   ", pat)
    }
    print!("{}\n            ", Colors::Reset);
    for pat in 0..=7 {
        print!("\x1B[4{}m   ", pat);
    }
    println!()
}
