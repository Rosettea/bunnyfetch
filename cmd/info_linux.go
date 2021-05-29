// +build linux

package cmd

import (
	"os"
	"os/exec"
	"strings"
)

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
