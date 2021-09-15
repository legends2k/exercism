// Package bob defines Bob
package bob

import (
	"strings"
	"unicode"
)

const testVersion = 3

// Hey is the way bob talks
func Hey(s string) string {
	yell := true
	alpha := false
	s = strings.Map(func(r rune) rune {
		if unicode.IsLetter(r) {
			alpha = true
		}
		if unicode.IsSpace(r) {
			return -1
		} else if r >= 'a' && r <= 'z' {
			yell = false
		}
		return r
	}, s)
	if s == "" {
		return "Fine. Be that way!"
	} else if alpha && yell {
		return "Whoa, chill out!"
	} else if strings.HasSuffix(s, "?") {
		return "Sure."
	}
	return "Whatever."
}
