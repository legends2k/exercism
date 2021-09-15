// Package pangram provides utilities to test for pangrams
package pangram

import (
	"unicode"
)

const testVersion = 1

// IsPangram returns true when the input is a pangram
func IsPangram(s string) bool {
	f := uint32(0x3ffffff) // one bit for each alphabet
	for _, c := range s {
		if unicode.IsLetter(c) {
			// fmt.Printf("%c\t", c)
			c = unicode.ToLower(c) - 'a'
			mask := uint32(^(1 << uint(c)))
			// fmt.Printf("%v\t%b\n", c, mask)
			f = f & mask
		}
	}
	// fmt.Printf("%b\n", f)
	return f == 0
}
