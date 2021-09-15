// Package luhn provides luhn algorithm implementaion
package luhn

import (
	"unicode"
)

// Valid validates a given SIN number
func Valid(s string) bool {
	r := []rune(s)
	n := len(r)
	digitCount := 0
	total := 0
	for i := n - 1; i >= 0; i-- {
		c := r[i]
		if unicode.IsDigit(c) {
			digitCount++
			digit := int(c - '0')
			if (digitCount % 2) == 0 {
				digit = digit * 2
				if digit > 9 {
					digit = digit - 9
				}
			}
			total = total + digit
		} else if !unicode.IsSpace(c) {
			return false
		}
	}
	return (digitCount >= 2) && (total%10) == 0
}