// Package raindrops is funny package to convert numbers to strings
package raindrops

import (
	"strconv"
	"unicode"
)

const testVersion = 3

// digitalRoot returns the digital root for a (ASCII) number
func digitalRoot(num string) int {
	sum := 0
	for _, c := range num {
		if unicode.IsDigit(c) {
			// https://stackoverflow.com/q/21322173/183120
			sum += int(c - '0')
		}
	}
	return sum
}

// isDivisibleByThree returns true if i is cleanly divided by 3
func isDivisibleByThree(num string) bool {
	r := digitalRoot(num)
	return (r % 3) == 0
}

// isDivisibleByFive returns true if i is cleanly divided by 5
func isDivisibleByFive(num string) bool {
	n := len(num)
	return (num[n-1] == '0') || (num[n-1] == '5')
}

// isDivisibleBySeven returns true if i is cleanly divided by 7
func isDivisibleBySeven(i int) bool {
	return (i % 7) == 0
}

// Convert the given number to string based on divisibility by 3, 5 & 7
func Convert(i int) string {
	str := strconv.FormatInt(int64(i), 10)
	isDivisibleByThree(str)
	var s string
	if isDivisibleByThree(str) {
		s += "Pling"
	}
	if isDivisibleByFive(str) {
		s += "Plang"
	}
	if isDivisibleBySeven(i) {
		s += "Plong"
	}
	if s == "" {
		return str
	}
	return s
}
