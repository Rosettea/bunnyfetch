<div align="center">
	<h1>Bunnyfetch</h1>
	<blockquote align="center">🐰 Tiny system info fetch utility.</blockquote>
	<p>
		<a href="https://github.com/Rosettea/Bunnyfetch/blob/master/LICENSE">
			<img alt="GitHub license" src="https://img.shields.io/github/license/Rosettea/Bunnyfetch?style=for-the-badge">
		</a>
		<a href="https://github.com/Rosettea/Bunnyfetch/stargazers">
			<img alt="GitHub stars" src="https://img.shields.io/github/stars/Rosettea/Bunnyfetch?style=for-the-badge">
		</a>
		<br>
<!--		<a href="https://github.com/Rosettea/Bunnyfetch/actions">
			<img alt="Windows Build Status" src="https://img.shields.io/github/workflow/status/Rosettea/Bunnyfetch/Windows%20Build?style=flat-square&logo=github&label=Windows">
		</a>
		<a href="https://github.com/Rosettea/Bunnyfetch/actions">
			<img alt="GNU/Linux Build Status" src="https://img.shields.io/github/workflow/status/Rosettea/Bunnyfetch/Linux%20Build?style=flat-square&logo=github&label=GNU/Linux">
		</a>
		<a href="https://github.com/Rosettea/Bunnyfetch/actions">
			<img alt="MacOS Build Status" src="https://img.shields.io/github/workflow/status/Rosettea/Bunnyfetch/MacOS%20Build?style=flat-square&logo=github&label=MacOS">
		</a>
		<br>-->
		Bunnyfetch is a small and fast tool for getting info about your system.
		The idea is from <a href="https://github.com/elenapan/dotfiles/blob/master/bin/bunnyfetch">this here</a> and I decided to make it crossplatform in Go.
	</p><br>
	<img src="https://safe.kashima.moe/n75w100wuuks.png">
</div>

# Install
Binaries are provided at the releases page [here](https://github.com/Rosettea/Bunnyfetch/releases).
Or, you can just run `go get -u github.com/Rosettea/bunnyfetch`

## Manual Compile
```sh
git clone https://github.com/Rosettea/Bunnyfetch
cd Bunnyfetch
go get -d ./...
go build -ldflags "-w -s" # ldflags make the binary smaller
```  

# License
Bunnyfetch is licensed under the MIT license.  
[Read here](LICENSE) for more info.
