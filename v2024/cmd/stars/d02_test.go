package stars

import (
	"strings"
	"testing"
)

func TestD02_1(t *testing.T) {
	provided := strings.Split(strings.TrimSpace(input02), "\n")
	tests := map[string]struct {
		input    []string
		expected int
	}{
		"provided test passes": {
			input: []string{
				"7 6 4 2 1",
				"1 2 7 8 9",
				"9 7 6 2 1",
				"1 3 2 4 5",
				"8 6 4 4 1",
				"1 3 6 7 9",
			},
			expected: 2,
		},
		"provided input passes": {
			input:    provided,
			expected: 549,
		},
	}
	for name, tt := range tests {
		t.Run(name, func(t *testing.T) {
			actual, err := d02_1(tt.input)
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
