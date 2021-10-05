// +build darwin

package cmd

import (
	"os"
	"os/exec"
	"strings"
)

func OS() string {
	out, err := exec.Command("sw_vers", "-productName").CombinedOutput()
	if err != nil {
		return "Unknown"
	}
	return strings.TrimSuffix(string(out), "\n")
}

func Kernel() string {
	out, err := exec.Command("uname", "-r").CombinedOutput()
	if err != nil {
		return "Unknown"
	}
	return strings.TrimSuffix(string(out), "\n")
}

func Shell() string {
	shellenv := strings.Split(os.Getenv("SHELL"), "/")
	return shellenv[len(shellenv)-1]
}

func WM() string {
	// AFAIK there are no other window manager than Quartz on Darwin
	return "Quartz"
}
