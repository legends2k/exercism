// Package gigasecond is a supplement to time.Duration
package gigasecond

// import path for the time package from the standard library
import "time"

const testVersion = 4

// 10^9 seconds = gigasecond, so 10^18 should do the trick
const Gigasecond = 1000000000 * time.Second

// AddGigasecond increments t by a gigasecond and returns the result
func AddGigasecond(t time.Time) time.Time {
	return t.Add(Gigasecond)
}