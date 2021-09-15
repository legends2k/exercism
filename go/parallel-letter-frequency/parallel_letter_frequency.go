// Package letter provides utilities around letters in a string
package letter

type FreqMap map[rune]int

// Frequency calculates the letter occurannces in a given string
func Frequency(s string) FreqMap {
	m := FreqMap{}
	for _, r := range s {
		m[r]++
	}
	return m
}

// An alternative approach would be to slice string into NumCPU substrings each
// of size m runes.
// ConcurrentFrequency counts each letter’s frequency in a given string slice
func ConcurrentFrequency(inStr []string) FreqMap {
	n := len(inStr)
	c := make(chan FreqMap, n)
	for i := 0; i < n; i++ {
		// Note the additional qualification for a write-only channel.
		go func(s string, c chan<- FreqMap) {
			c <- Frequency(s)
		}(inStr[i], c)
	}
	collated := FreqMap{}
	for i := 0; i < n; i++ {
		miniMap := <-c
		for k, v := range miniMap {
			collated[k] += v
		}
	}
	return collated
}
