package cmd

import (
	"os"
	"os/user"
)

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


