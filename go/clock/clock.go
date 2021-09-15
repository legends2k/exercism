// Package clock provides a simple clock that deals with just hours and minutes.
package clock

import (
	"fmt"
	"math"
)

const testVersion = 4

type Clock struct {
	h int
	m int
}

// New Clock instance
func New(hour, minute int) Clock {
	hour += int(math.Floor(float64(minute) / 60.0))
	hour = kmod(hour, 24)
	minute = kmod(minute, 60)
	return Clock{hour, minute}
}

// kmod retuns the default mod behaviour in languages like Lua a.k.a Knuth mod
// The default mod of Go is C’s mod, which is undesired here.
// 43 % 24 = (19 + 24) % 24 = 19
// -91 % 24 = (-19 + 24) % 24 = 5
//
// if branching, then it’d be
// if value < 0 {
//     return (value % max) + max
// } else {
//     return (value % max)
// }
// this is a branchless version
func kmod(value int, max int) int {
	return ((value % max) + max) % max
}

// String returns the time as a string
func (c Clock) String() string {
	return fmt.Sprintf("%02d:%02d", c.h, c.m)
}

// Add updates the clock’s current by minutes
func (c Clock) Add(minute int) Clock {
	return New(c.h, c.m+minute)
}