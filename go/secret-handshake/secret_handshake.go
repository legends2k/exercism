// Package secret is helps convert messages to secret codes
package secret

const testVersion = 2

// reverseSlice reverses a slice of strings
func reverseSlice(s *[]string) {
	var i, n int = 0, len(*s) - 1
	for ; i < n; i, n = i+1, n-1 {
		// fmt.Printf("\t%v\t%v\n", i, n)
		(*s)[i], (*s)[n] = (*s)[n], (*s)[i]
	}
}

// Handshake decodes code into handshake messages
func Handshake(code uint) []string {
	mask := uint(1)
	result := make([]string, 0, 4)
	msgs := [...]string{
		1:  "wink",
		2:  "double blink",
		4:  "close your eyes",
		8:  "jump"}
	const reverseMsg = 16
	// fmt.Printf("%b\n", code)
	for code != 0 {
		id := code & mask
		code &^= mask
		// fmt.Printf("%b\t%b\t%b\n", mask, code, id)
		if id == reverseMsg {
			reverseSlice(&result)
		} else if msgs[id] != "" {
			result = append(result, msgs[id])
		}
		mask <<= 1
		mask |= 1
	}
	return result
}