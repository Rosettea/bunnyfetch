// +build linux

package cmd

import (
	"os"
	"strings"

	"github.com/jezek/xgb"
	"github.com/jezek/xgb/xproto"
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
	X, err := xgb.NewConn()
	if err != nil {
		return "Unknown"
	}
	defer X.Close()

	setup := xproto.Setup(X)
	root := setup.DefaultScreen(X).Root

	// get a "window" for the window manager (makes sense huh)
	aname := "_NET_SUPPORTING_WM_CHECK"
	activeAtom, err := xproto.InternAtom(X, true, uint16(len(aname)), aname).Reply()
	if err != nil {
		return "Unknown"
	}

	// get the atom for the actual window manager name
	aname = "_NET_WM_NAME"
	nameAtom, err := xproto.InternAtom(X, true, uint16(len(aname)), aname).Reply()
	if err != nil {
		return "Unknown"
	}

	// and its value
	reply, err := xproto.GetProperty(X, false, root, activeAtom.Atom,
		xproto.GetPropertyTypeAny, 0, (1 << 32) - 1).Reply()
	if err != nil {
		return "Unknown"
	}
	windowID := xproto.Window(xgb.Get32(reply.Value))

	reply, err = xproto.GetProperty(X, false, windowID, nameAtom.Atom,
	xproto.GetPropertyTypeAny, 0, (1<<32)-1).Reply()
	if err != nil {
		return "Unknown"
	}

	return string(reply.Value)
}

