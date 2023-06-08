package main

import (
	"bufio"
	"fmt"
	"os"
	"os/exec"
	"regexp"
	"strings"
)

// this tool takes two length os.Args.
const ArgsLen = 2

func main() {
	// TODO: implement this with reflection
	cmds := map[string]func() error{
		"download": download,
		"update":   update,
		"generate": generate,
		"dev":      dev,
		"lint":     lint,
		"fix":      fix,
		"test":     test,
	}

	if len(os.Args) != ArgsLen {
		fmt.Fprintln(os.Stderr, "Error: go run ./task <command>")
		os.Exit(1)
	}

	target := os.Args[1]
	exist := false
	for name, cmd := range cmds {
		if target == name {
			exist = true
			if err := cmd(); err != nil {
				fmt.Fprintf(os.Stderr, "An error has occurred while running task `%s`: %s\n", name, err.Error())
				os.Exit(1)
			}
			break
		}
	}

	if !exist {
		fmt.Fprintf(os.Stderr, "Error: go run ./task sub-command `%s` not found\n", target)
		os.Exit(1)
	}
}

func run(cmd string, opts ...string) error {
	fmt.Printf("$ %s %s\n", cmd, strings.Join(opts, " "))
	c := exec.Command(cmd, opts...)
	c.Stdout = os.Stdout
	c.Stderr = os.Stderr
	return c.Run()
}

func download() error {
	f, err := os.Open("tools.go")
	if err != nil {
		return err
	}
	defer f.Close()

	r := regexp.MustCompile(`^\s*_\s*"([^"]+)"`)
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		line := scanner.Text()
		found := r.FindStringSubmatch(line)
		if found != nil {
			packageName := found[1]
			if err := run("go", "install", packageName); err != nil {
				return err
			}
		}
	}

	if err := scanner.Err(); err != nil {
		return err
	}

	return run("go", "mod", "download")
}

func update() error {
	return run("go", "mod", "tidy")
}

func generate() error {
	return run("go", "generate", "./...")
}

func dev() error {
	return run("wails", "dev")
}

func lint() error {
	return run("golangci-lint", "run", "--timeout=1m", "-v", "./...")
}

func fix() error {
	return run("golangci-lint", "run", "--timeout=1m", "--fix", "./...")
}

func test() error {
	return run("go", "test", "./...")
}
