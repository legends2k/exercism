package bob

import (
	"strings"
)

const testVersion = 3

func Hey(s string) string {
	s = strings.Trim(s, " \t\n\r")
	if s == "" {
		return "Fine. Be that way!"
	} else if s == strings.ToUpper(s) && strings.ContainsAny(s, "ABCDEFGHIJKLMNOPQRSTUVWXYZ") {
		return "Whoa, chill out!"
	} else if strings.HasSuffix(s, "?") {
		return "Sure."
	}
	return "Whatever."
}
