// +build windows

package cmd

import (
	"os/exec"
	"strings"
)

func OS() string {
	// wmic should be available on all recent windows versions
	out, _ := exec.Command("wmic", "os", "get", "caption").Output()
	winver := strings.TrimSpace(strings.Split(string(out), "\r\r\n")[1])

	return strings.TrimPrefix(winver, "Microsoft ")
}

func Kernel() string {
	out, _ := exec.Command("wmic", "os", "get", "Version").Output()
	kernelver := strings.TrimSpace(strings.Split(string(out), "\r\r\n")[1])

	return kernelver
}

func Shell() string {
	return "cmd.exe" // TODO
}

func WM() string {
	return "Metro" // TODO
}
