package main

import (
	"fmt"
	"os"
	"os/exec"
	"strconv"
	"time"
)

func main() {
	ch := make(chan string)
	go func(ch chan string) {
		// disable input buffering
		exec.Command("stty", "-F", "/dev/tty", "cbreak", "min", "1").Run()
		// do not display entered characters on the screen
		exec.Command("stty", "-F", "/dev/tty", "-echo").Run()
		var b []byte = make([]byte, 1)
		for {
			os.Stdin.Read(b)
			ch <- string(b)
		}
	}(ch)

	for {
		select {
		case stdin, _ := <-ch:
			n, err := strconv.Atoi(stdin)
			if err != nil {
				fmt.Println("\x07")
			} else {
				for range n {
					n, err := os.Stdout.Write([]byte("\x07"))
					if err != nil {
						fmt.Println("Error while beeping", err)
					}
					fmt.Printf("%d bytes writted\n", n)
					time.Sleep(time.Millisecond * 100)
				}
			}
		}
		time.Sleep(time.Millisecond * 100)
	}
}
