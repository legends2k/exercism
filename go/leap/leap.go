// Package leap contains leap year utilites.
package leap

const testVersion = 3

// IsLeapYear returns true if year is a leap year
func IsLeapYear(year int) bool {
	return ((year % 4) == 0) && (((year % 100) != 0) || ((year % 400) == 0))
}
