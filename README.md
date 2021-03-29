<div align="center">
	<h1>Bunnyfetch</h1>
	<blockquote align="center">🐰 Tiny system info fetch utility.</blockquote>
	<p>
		<a href="https://github.com/Luvella/Bunnyfetch/blob/master/LICENSE">
			<img alt="GitHub license" src="https://img.shields.io/github/license/Luvella/Bunnyfetch?style=for-the-badge">
		</a>
		<a href="https://github.com/Luvella/Bunnyfetch/stargazers">
			<img alt="GitHub stars" src="https://img.shields.io/github/stars/Luvella/Bunnyfetch?style=for-the-badge">
		</a>
		<br>
		<a href="https://github.com/Luvella/Bunnyfetch/actions">
			<img alt="Windows Build Status" src="https://img.shields.io/github/workflow/status/Luvella/Bunnyfetch/Windows%20Build?style=flat-square&logo=github&label=Windows">
		</a>
		<a href="https://github.com/Luvella/Bunnyfetch/actions">
			<img alt="GNU/Linux Build Status" src="https://img.shields.io/github/workflow/status/Luvella/Bunnyfetch/Linux%20Build?style=flat-square&logo=github&label=GNU/Linux">
		</a>
		<a href="https://github.com/Luvella/Bunnyfetch/actions">
			<img alt="MacOS Build Status" src="https://img.shields.io/github/workflow/status/Luvella/Bunnyfetch/MacOS%20Build?style=flat-square&logo=github&label=MacOS">
		</a>
		<br>
		Bunnyfetch is a small and fast tool for getting info about your system.
		The idea is from <a href="https://github.com/elenapan/dotfiles/blob/master/bin/bunnyfetch">this here</a> and I decided to make it a multiplatform tool in Rust.
	</p>
</div>

# Table of Contents
- [Install](#install)
  - [Compile](#compiling)
- [License](#license)

# Install
Binaries are provided at the releases page [here](https://github.com/Luvella/Bunnyfetch/releases).
Alternatively, those on Arch Linux can compile using the [AUR package](https://aur.archlinux.org/packages/bunnyfetch-git/).

## Compiling
This project is made in [Rust](https://rust-lang.org/) so you will require the Rust toolchain installed. Steps for its installation are provided at that link.  
When installed run the following commands:  
```sh
git clone https://github.com/Luvella/Bunnyfetch
cd Bunnyfetch
cargo build --release
```  
The binary will be in the directory `./target/release`

# License
Bunnyfetch is licensed under the MIT license.  
[Read here](LICENSE) for more info.
