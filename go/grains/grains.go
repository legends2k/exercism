package grains

import (
	"errors"
	"reflect"
)

// Square returns the value of two raised to a given power.
func Square(i int) (uint64, error) {
	var err error = nil
	var n uint64
	if (i >= 1) && (i <= reflect.TypeOf(n).Bits()) {
		n = uint64(1) << uint(i-1)
	} else {
		err = errors.New("Power cannot be negative or beyond maximum int type's (uint64) bit width")
	}
	return n, err
}

// Total returns the total of the 2⁰ + 2¹ + … + 2⁶⁴
func Total() uint64 {
	var t uint64
	for i := 1; i <= 64; i++ {
		s, _ := Square(i)
		t = t + s
	}
	return t
}