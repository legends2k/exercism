// Package raindrops is funny package to convert numbers to strings
package raindrops

import (
	"strconv"
)

const testVersion = 3

// isDivisibleByThree returns true if i is cleanly divided by 3
func isDivisibleByThree(i int) bool {
	return (i % 3) == 0
}

// isDivisibleByFive returns true if i is cleanly divided by 5
func isDivisibleByFive(i int) bool {
	return (i % 5) == 0
}

// isDivisibleBySeven returns true if i is cleanly divided by 7
func isDivisibleBySeven(i int) bool {
	return (i % 7) == 0
}

// Convert the given number to string based on divisibility by 3, 5 & 7
func Convert(i int) string {
	var s string
	if isDivisibleByThree(i) {
		s += "Pling"
	}
	if isDivisibleByFive(i) {
		s += "Plang"
	}
	if isDivisibleBySeven(i) {
		s += "Plong"
	}
	if s == "" {
		return strconv.FormatInt(int64(i), 10)
	}
	return s
}
