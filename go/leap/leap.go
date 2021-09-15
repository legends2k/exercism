// Package leap contains leap year utilites.
package leap

const testVersion = 3

// IsLeapYear returns true if year is a leap year
func IsLeapYear(year int) bool {
	// https://news.ycombinator.com/item?id=21928681
	factor := year % 100
	if factor == 0 {
		factor = year / 100
	}
	return (factor % 4) == 0
}
