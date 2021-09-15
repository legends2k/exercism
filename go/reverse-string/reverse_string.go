// Package reverse provides utilies for reversals
package reverse

// String reverses a given string making two rune array copies
func String(s string) string {
	r := []rune(s)
	n := len(r)
	nr := make([]rune, n)
	for i := 0; i < n; i++ {
		nr[i] = r[n-i-1]
	}
	return string(nr)
}
