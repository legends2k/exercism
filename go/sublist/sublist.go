package sublist

type Relation string

func areEqual(a, b []int) bool {
	if len(a) != len(b) {
		return false
	}
	for i := 0; i < len(a); i++ {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}

// Brute-force approach since the strings.Contains approach involved memory
// allocation which makes it a lot slower.  stdlib implements the Rubin–Karp
// substring search algorithm whose average and best case running time is O(n+m)
// in space O(p), but its worst-case time is O(nm).  Wikipedia on brute-force:
// … works well in many practical cases, but can exhibit relatively long running
// times on certain examples, such as searching for a pattern string of 10,000
// "a"s followed by a single "b" in a search string of 10 million "a"s, in which
// case it exhibits its worst-case O(mn) time.
func index(needle, haystack []int) int {
	for i := 0; i < len(haystack)-len(needle)+1; i++ {
		if areEqual(needle, haystack[i:i+len(needle)]) {
			return i
		}
	}
	return -1
}

func Sublist(a, b []int) Relation {
	na, nb := len(a), len(b)
	if na == 0 && nb == 0 {
		return "equal"
	} else if na == 0 {
		return "sublist"
	} else if nb == 0 {
		return "superlist"
	}
	idx := -1
	if na >= nb {
		idx = index(b, a)
	} else {
		idx = index(a, b)
	}
	if idx < 0 {
		return "unequal"
	}
	if na == nb {
		return "equal"
	} else if na > nb {
		return "superlist"
	}
	return "sublist"
}