package cmd

import (
	"fmt"
	"strings"
	"os"
	"os/exec"
	"os/user"

	"github.com/spf13/cobra"
	"github.com/joho/godotenv"
)

var RootCmd = &cobra.Command{
	Use: "bunnyfetch",
	Short: "Tiny system info fetch utility.",
	Run: func(cmd *cobra.Command, args []string) {
		bunnyfetch()
	},
}
var r = "\033[0m"
var foot = "\033[31m\"" + r

var Bunny =
`            %s
            %s
   (\ /)    %s
   ( . .)   %s
   c(%s)(%s)  %s`

func bunnyfetch() {
	// /etc/os-release should always exist on Linux
	_ = godotenv.Load("/etc/os-release")
	fmt.Printf(
		Bunny,
		titleinf(),
		osinf(),
		kernelinf(),
		shellinf(),
		foot, foot,
		wminf())
	fmt.Printf("\n         ")
	for i := 0; i < 8; i++ {
		fmt.Printf("\033[4%dm   ", i)
	}
	fmt.Printf(r + "\n         ")
	for i := 0; i < 8; i++ {
		fmt.Printf("\033[10%dm   ", i)
	}
	fmt.Println(r + "\n")
}

func titleinf() string {
	return "\033[31m" + Title() + r
}

func osinf() string {
	// ansi colors of distro (name)? from /etc/os-release
	color := os.Getenv("ANSI_COLOR")
	if color == "" {
		color = "32"
	}

	return "\033[" + color + "mOS " + r + OS()
}

func kernelinf() string {
	return "\033[33mKernel " + r + Kernel()
}

func shellinf() string {
	return "\033[34mShell " + r + Shell()
}

func wminf() string {
	return "\033[35mWM " + r + WM()
}

func Title() string {
	curuser, err := user.Current()
	user := "Unknown"
	if err == nil {
		user = curuser.Username
	}

	hostname, err := os.Hostname()
	host := "Unknown"
	if err == nil {
		host = hostname
	}

	return user + "@" + host
}

func OS() string {
	return os.Getenv("NAME")
}

func Kernel() string {
	// /proc/version should always exist on linux
	procver, _ := os.ReadFile("/proc/version")

	// /proc/version has the same format with "Linux version <kern-version>" as the 3rd
	// word, `procver` is []byte so has to be converted 
	return strings.Split(string(procver), " ")[2]
}

func Shell() string {
	shellenv := strings.Split(os.Getenv("SHELL"), "/")
	return shellenv[len(shellenv) - 1]
}

func WM() string {
	out, _ := exec.Command("bash", "-c", `xprop -id $(xprop -root -notype | awk '$1=="_NET_SUPPORTING_WM_CHECK:"{print $5}') -notype -f _NET_WM_NAME 8t | grep "WM_NAME" | cut -f2 -d \"`).CombinedOutput()

	return string(out)
}
