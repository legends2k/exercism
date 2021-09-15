package sublist

import (
	"fmt"
	"strings"
)

type Relation string
type intSlice []int

// String returns a string representation of []int
func (l intSlice) String() string {
	// using a value recevier instead of a pointer receiver since slice copies
	// are not deep copies:
	// https://blog.golang.org/go-slices-usage-and-internals
	var b strings.Builder
	for _, e := range l {
		// without the delimiter false negatives pass e.g. [1,23] == [12,3]
		fmt.Fprintf(&b, "%d,", e)
	}
	return b.String()
}

func Sublist(a, b intSlice) Relation {
	na, nb := len(a), len(b)
	sa, sb := a.String(), b.String()
	// use stdlib’s substring search implementation like KMP that’s way more
	// tested, robust and perhaps optimized than reinventing the wheel
	substring := false
	if na >= nb {
		substring = strings.Contains(sa, sb)
	} else {
		substring = strings.Contains(sb, sa)
	}
	if substring {
		if na == nb {
			return "equal"
		} else if na > nb {
			return "superlist"
		}
		return "sublist"
	}
	return "unequal"
}
