package stars

import (
	_ "embed"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

//go:embed d02_input
var input02 string

// D02_1 solves https://adventofcode.com/2024/day/2
func D02_1() {
	lines := strings.Split(strings.TrimSpace(input02), "\n")
	answer, err := d02_1(lines)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed to find the answer! Err: %+v\n", err)
		return
	}
	fmt.Printf("Answer: %d\n", answer)
}

// d02_1 performs logic
func d02_1(lines []string) (int, error) {
	sum := 0
	for _, line := range lines {
		chars := strings.Split(line, " ")
		asc := false
		desc := false
		for ii, val := range chars {
			if ii <= 0 {
				continue
			}
			prev := chars[ii-1]
			a, err := strconv.ParseInt(prev, 10, 32)
			if err != nil {
				return 0, err
			}
			b, err := strconv.ParseInt(val, 10, 32)
			if err != nil {
				return 0, err
			}
			diff := b - a
			if ii == 1 {
				switch {
				case diff > 0:
					asc = true
				case diff < 0:
					desc = true
				}
			}
			if (diff < 0 && asc) || (diff > 0 && desc) {
				break
			}
			if diff > 3 || diff == 0 || diff < -3 {
				break
			}
			if ii == len(chars)-1 {
				sum += 1
			}
		}
	}
	return sum, nil
}

// D02_2 solves https://adventofcode.com/2024/day/2
func D02_2() {
	lines := strings.Split(strings.TrimSpace(input02), "\n")
	answer, err := d02_2(lines)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed to find the answer! Err: %+v\n", err)
		return
	}
	fmt.Printf("Answer: %d\n", answer)
}

// d02_2 performs logic
func d02_2(lines []string) (int, error) {
	sum := 0
	for _, line := range lines {
		chars := strings.Split(line, " ")
		add, err := d02_1([]string{line})
		if err != nil {
			return 0, err
		}
		if add > 0 {
			sum += add
			continue
		}
		for ii := range append([]string{}, chars...) {
			ch := slices.Clone(chars)
			snippet := append(ch[:ii], ch[ii+1:]...)
			add, err := d02_1([]string{strings.Join(snippet, " ")})
			if err != nil {
				return 0, err
			}
			if add > 0 {
				sum += add
				break
			}
		}
	}
	return sum, nil
}
