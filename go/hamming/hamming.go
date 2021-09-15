// Package hamming provides utilites on performing hamming distance checks
package hamming

import (
	"errors"
	"io"
	"strings"
)

const testVersion = 6

// Distance returns the hamming distance between two DNA strands; when the
// lengths differ it errors out.
func Distance(a, b string) (int, error) {
	pa := strings.NewReader(a)
	d := 0
	err := io.EOF
	for _, cb := range b {
		var ca rune
		if ca, _, err = pa.ReadRune(); err != nil {
			return 0, errors.New("Sequence lengths differ")
		}
		if ca != cb {
			d++
		}
	}
	_, err = pa.ReadByte()
	if err != io.EOF {
		return 0, errors.New("Sequence lengths differ")
	}
	return d, nil
}