package stars

import (
	_ "embed"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

//go:embed d01_input
var input01 string

// D01_1 solves https://adventofcode.com/2024/day/1
func D01_1() {
	lines := strings.Split(strings.TrimSpace(input01), "\n")
	answer, err := d01_1(lines)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed to find the answer! Err: %+v\n", err)
		return
	}
	fmt.Printf("Answer: %d\n", answer)
}

// d01_1 performs logic
func d01_1(lines []string) (int, error) {
	left := []int{}
	right := []int{}
	for _, line := range lines {
		values := strings.Split(line, "   ")
		a, err := strconv.ParseInt(values[0], 10, 32)
		if err != nil {
			return 0, err
		}
		b, err := strconv.ParseInt(values[1], 10, 32)
		if err != nil {
			return 0, err
		}
		left = append(left, int(a))
		right = append(right, int(b))
	}
	slices.Sort(left)
	slices.Sort(right)
	distance := 0
	for ii := range left {
		diff := left[ii] - right[ii]
		if diff < 0 {
			distance -= diff
		} else {
			distance += diff
		}
	}
	return distance, nil
}

// D01_2 solves https://adventofcode.com/2024/day/1
func D01_2() {
	lines := strings.Split(strings.TrimSpace(input01), "\n")
	answer, err := d01_2(lines)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed to find the answer! Err: %+v\n", err)
		return
	}
	fmt.Printf("Answer: %d\n", answer)
}

// d01_2 performs logic
func d01_2(lines []string) (int, error) {
	left := []int{}
	right := []int{}
	for _, line := range lines {
		values := strings.Split(line, "   ")
		a, err := strconv.ParseInt(values[0], 10, 32)
		if err != nil {
			return 0, err
		}
		b, err := strconv.ParseInt(values[1], 10, 32)
		if err != nil {
			return 0, err
		}
		left = append(left, int(a))
		right = append(right, int(b))
	}
	similarities := 0
	for _, aa := range left {
		count := 0
		for _, bb := range right {
			if aa == bb {
				count += 1
			}
		}
		similarities += aa * count
	}
	return similarities, nil
}
