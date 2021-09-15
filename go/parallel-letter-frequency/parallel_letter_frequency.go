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

// Wrap around sequential counter to communicate return value over a channel.
// Note the additional qualification for a write-only channel.
func count(s string, c chan<- FreqMap) {
	c <- Frequency(s)
}

// An alternative approach would be to
// 1. Slice string into NumCPU substrings each of size m runes
// 2. Make a buffered channel with NumCPU slots
// 3. Feed each slice to a goroutine and get a resulting map in a slot
// 3. Total them as each returns back to the main goroutine
// ConcurrentFrequency counts each letter’s frequency in a given string slice
func ConcurrentFrequency(inStr []string) FreqMap {
	n := len(inStr)
	c := make(chan FreqMap, n)
	for i := 0; i < n; i++ {
		go count(inStr[i], c)
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
