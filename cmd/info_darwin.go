//go:build darwin
// +build darwin

package cmd

import (
	"bytes"
	"os"
	"os/exec"
	"strings"
)

func OS() string {
	name, err := exec.Command("sw_vers", "-productName").CombinedOutput()
	if err != nil {
		return "Unknown"
	}

	name = bytes.TrimSuffix(name, []byte{'\n'})

	version, err := exec.Command("sw_vers", "-productVersion").CombinedOutput()
	if err != nil {
		return string(name)
	}

	version = bytes.TrimSuffix(version, []byte{'\n'})
	return string(name) + " " + string(version)
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
