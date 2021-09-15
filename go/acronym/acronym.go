// Package acronym has utilities to deal with TLAs
package acronym

import (
	"strings"
	"unicode"
)

const testVersion = 3

// Abbreviate abbriviates given string into acronyms
func Abbreviate(phrase string) string {
	acro := ""
	catch := true
	for _, c := range phrase {
		if !unicode.IsLetter(c) {
			catch = true
		} else if catch {
			acro += strings.ToUpper(string(c))
			catch = false
		}
	}
	return acro
}