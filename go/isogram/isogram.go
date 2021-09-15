// Package isogram provides utilties related to isograms
package isogram

import (
	"strings"
	"unicode"
)

// IsIsogram checks whether a given string is an isogram
func IsIsogram(s string) bool {
	occurances := [26]int{}
	for _, ch := range strings.ToLower(s) {
		if unicode.IsLetter(ch) {
			idx := ch - 'a'
			occurances[idx]++
			if occurances[idx] > 1 {
				return false
			}
		}
	}
	return true
}