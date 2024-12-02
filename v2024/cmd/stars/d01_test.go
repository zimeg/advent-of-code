package stars

import (
	"strings"
	"testing"
)

func TestD01_1(t *testing.T) {
	provided := strings.Split(strings.TrimSpace(input01), "\n")
	tests := map[string]struct {
		input    []string
		expected int
	}{
		"provided test passes": {
			input: []string{
				"3   4",
				"4   3",
				"2   5",
				"1   3",
				"3   9",
				"3   3",
			},
			expected: 11,
		},
		"provided input passes": {
			input:    provided,
			expected: 2164381,
		},
	}
	for name, tt := range tests {
		t.Run(name, func(t *testing.T) {
			actual, err := d01_1(tt.input)
			if err != nil {
				t.Logf("Unexpected error! Err: %+v", err)
				t.FailNow()
			}
			if actual != tt.expected {
				t.Logf("Expected: %d\nActual: %d", tt.expected, actual)
				t.Fail()
			}
		})
	}
}

func TestD01_2(t *testing.T) {
	provided := strings.Split(strings.TrimSpace(input01), "\n")
	tests := map[string]struct {
		input    []string
		expected int
	}{
		"provided test passes": {
			input: []string{
				"3   4",
				"4   3",
				"2   5",
				"1   3",
				"3   9",
				"3   3",
			},
			expected: 31,
		},
		"provided input passes": {
			input:    provided,
			expected: 20719933,
		},
	}
	for name, tt := range tests {
		t.Run(name, func(t *testing.T) {
			actual, err := d01_2(tt.input)
			if err != nil {
				t.Logf("Unexpected error! Err: %+v", err)
				t.FailNow()
			}
			if actual != tt.expected {
				t.Logf("Expected: %d\nActual: %d", tt.expected, actual)
				t.Fail()
			}
		})
	}
}
