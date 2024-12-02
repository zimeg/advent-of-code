package main

import (
	"fmt"
	"os"

	"github.com/zimeg/advent-of-code/v2024/cmd/stars"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintf(os.Stderr, "No date provided! Example: 'aoc 1.1'\n")
		return
	}
	switch os.Args[1] {
	case "1.1":
		stars.D01_1()
	case "1.2":
		stars.D01_2()
	}
}
