// Package accumulate stores accumulation methods
package accumulate

const testVersion = 1

// Accumulate maps elements to their image via the function f
func Accumulate(elem []string, f func(string) string) []string {
	image := make([]string, len(elem))
	for i, s := range elem {
		image[i] = f(s)
	}
	return image
}