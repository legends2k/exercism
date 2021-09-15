// Package twelve is to compose 'The Twelve Days of Christmas' song
package twelve

import (
	"fmt"
	"strings"
)

const testVersion = 1

const template = "On the %v day of Christmas my true love gave to me, %v."

var dayOrdinal = [...]string{
	"first",
	"second",
	"third",
	"fourth",
	"fifth",
	"sixth",
	"seventh",
	"eighth",
	"ninth",
	"tenth",
	"eleventh",
	"twelfth"}

var gift = [...]string{
	"twelve Drummers Drumming",
	"eleven Pipers Piping",
	"ten Lords-a-Leaping",
	"nine Ladies Dancing",
	"eight Maids-a-Milking",
	"seven Swans-a-Swimming",
	"six Geese-a-Laying",
	"five Gold Rings",
	"four Calling Birds",
	"three French Hens",
	"two Turtle Doves",
	"and a Partridge in a Pear Tree"}

// Verse returns one of the verses of the 'The Twelve Days of Christmas' song
func Verse(i int) string {
	var s string
	if i != 1 {
		n := len(gift) - i
		s = strings.Join(gift[n:], ", ")
	} else {
		s = "a Partridge in a Pear Tree"
	}
	return fmt.Sprintf(template, dayOrdinal[i-1], s)
}

// Song composes and returns the 'The Twelve Days of Christmas' song
func Song() string {
	// one empty string in the end for the required trailing newline
	r := [13]string{}
	for i := 1; i <= len(gift); i++ {
		r[i-1] = Verse(i)
	}
	return strings.Join(r[:], "\n")
}