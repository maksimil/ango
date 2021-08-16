package main

import (
	"fmt"
	"os"
	"os/exec"
	"time"
)

func run(name string, arg []string, env []string) {
	cmd := exec.Command(name, arg...)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Stdin = os.Stdin
	cmd.Env = env

	if err := cmd.Run(); err != nil {
		panic(err)
	}
}

func bench(object string) int64 {
	cwd, err := os.Getwd()

	if err != nil {
		panic(err)
	}

	ts1 := time.Now().UnixNano()
	run("target\\release\\ango.exe", []string{"add", object}, []string{fmt.Sprintf("ANGO_PATH=%s/.ango", cwd)})
	ts2 := time.Now().UnixNano()

	return ts2 - ts1
}

func main() {
	// build the exe
	run("cargo", []string{"build", "--release"}, nil)

	// run benches
	runs := 10
	objects := []string{"src", "target", "Cargo.toml", "Cargo.lock"}

	logs := make(map[string]float64)
	for _, obj := range objects {
		logs[obj] = 0
	}

	for i := 0; i < runs; i++ {
		// clear ango files
		run("sh", []string{"scripts/clear.sh"}, nil)

		for _, obj := range objects {
			ts := bench(obj)
			logs[obj] += float64(ts) / 1_000_000
		}
	}

	for _, obj := range objects {
		ts := logs[obj] / float64(runs)
		fmt.Printf("%s: %fms\n", obj, ts)
	}
}
