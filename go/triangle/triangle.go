// Package triangle provides triangle-processing utilities
package triangle

import (
	"math"
)

const testVersion = 3

const (
	NaT = 0
	Equ = 1
	Iso = 2
	Sca = 3
)

type Kind uint8

func isFinite(f float64) bool {
	return !(math.IsNaN(f) || math.IsInf(f, 0))
}

func IsValid(a, b, c float64) bool {
	return isFinite(a) && isFinite(b) && isFinite(c) &&
		(a > 0) && (b > 0) && (c > 0) &&
		((a + b) >= c) && ((b + c) >= a) && ((c + a) >= b)
}

// KindFromSides returns the class of the triangle based on its side lengths
func KindFromSides(a, b, c float64) Kind {
	if !IsValid(a, b, c) {
		return NaT
	}
	if (a == b) && (b == c) {
		return Equ
	}
	if (a == b) || (b == c) || (c == a) {
		return Iso
	}
	return Sca
}