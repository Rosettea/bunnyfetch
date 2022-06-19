// +build linux

package cmd

import (
	"os"
	"os/exec"
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

// $XDG_SESSION_DESKTOP

func WM() string {
	if waylandDisplay := os.Getenv("WAYLAND_DISPLAY"); os.Getenv("XDG_SESSION_TYPE") == "wayland" || waylandDisplay != "" {
		// if we're here WAYLAND_DESKTOP wont be empty
		sockPath := os.Getenv("XDG_RUNTIME_DIR") + "/" + waylandDisplay
		_, err := os.Stat(sockPath)
		if err != nil {
			return "Unknown"
		}

		// TODO: not shell out to fuser
		// doing its job is half hell and i dont really want to deal with it
		out, err := exec.Command("fuser", sockPath).CombinedOutput()
		if err != nil {
			return "Unknown"
		}

		procID := strings.TrimSpace(strings.Split(string(out), ":")[1])
		bin, err := os.Readlink("/proc/" + procID + "/exe")
		// apparently may not exist?
		if err != nil {
			return "Unknown"
		}

		splitBin := strings.Split(bin, "/")
		return splitBin[len(splitBin) - 1]
	}
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

