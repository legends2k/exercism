// Package reverse provides utilies for reversals
package reverse

// String reverses a given string
func String(s string) string {
	r := []rune(s)
	n := len(r)
	m := n / 2
	for i := 0; i < m; i++ {
		r[i], r[n-i-1] = r[n-i-1], r[i]
	}
	return string(r)
}
