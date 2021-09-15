// Package matrix gives a matrix type and related utilities
package matrix

import (
	"bufio"
	"errors"
	"fmt"
	"strconv"
	"strings"
)

type Matrix [][]int

// String is Matrix’s Stringer implementation
func (m *Matrix) String() string {
	b := strings.Builder{}
	r := len(*m)
	c := len((*m)[0])
	for i := 0; i < r; i++ {
		for j := 0; j < c; j++ {
			fmt.Fprintf(&b, "%v ", (*m)[i][j])
		}
		fmt.Fprintf(&b, "\n")
	}
	return b.String()
}

// New returns a new n x m Matrix based on the input
func New(d string) (Matrix, error) {
	// since the tests expect to error out on leading and trailing new lines, we
	//check this since bufio.Scanner discards trailing empty lines
	if strings.HasSuffix(d, "\n") {
		return nil, errors.New("Invalid data")
	}
	s := bufio.NewScanner(strings.NewReader(d))
	m := make([][]int, 0) // make an empty matrix with 0 rows
	cols := -1
	for s.Scan() {
		line := s.Text()
		// use strings.Fields, not strings.Split since the latter will not club
		// up multiple separators as one unit and also not ignore leading and
		// trailing separators
		// https://medium.com/@mlowicki/strings-fieldsfunc-vs-strings-split-96c667912f78
		f := strings.Fields(line)
		if (len(f) != cols) && (cols != -1) {
			return nil, errors.New("Invalid data")
		} else if cols == -1 {
			cols = len(f)
		}
		e := make([]int, 0, cols)
		for _, tok := range f {
			i, err := strconv.Atoi(tok)
			if err != nil {
				return nil, err
			}
			e = append(e, i)
		}
		m = append(m, e)
	}
	return m, nil
}

// Set sets the value of an element given by (row, col)
func (m *Matrix) Set(row, col, val int) bool {
	if row < 0 || row >= len(*m) || col < 0 || col >= len((*m)[0]) {
		return false
	}
	(*m)[row][col] = val
	return true
}

// Cols returns a new copy of all the columns
func (m *Matrix) Cols() [][]int {
	r := len((*m))
	c := len((*m)[0])
	out := make([][]int, 0, c)
	for i := 0; i < c; i++ {
		col := make([]int, 0, r)
		for j := 0; j < r; j++ {
			col = append(col, (*m)[j][i])
		}
		out = append(out, col)
	}
	return out
}

// Rows returns a new copy of all the rows
func (m *Matrix) Rows() [][]int {
	r := len(*m)
	c := len((*m)[0])
	out := make([][]int, 0, r)
	for i := 0; i < r; i++ {
		s := make([]int, c)
		// use copy to do the deep copy since both are column-wise
		copy(s, (*m)[i])
		out = append(out, s)
	}
	return out
}