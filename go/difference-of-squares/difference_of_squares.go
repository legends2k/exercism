// Package diffsquares provides natural number difference of squares
package diffsquares

const testVersion = 1

// SumOfSquares returns square of the sum of first n numbers
func SumOfSquares(n int) int {
	return (n * (n + 1) * (2*n + 1)) / 6
}

// SquareOfSums returns the sum of square of first n numbers
func SquareOfSums(n int) int {
	num := (n * (n + 1)) / 2
	return (num * num)
}

// Difference returns the difference betweem the two squares
func Difference(n int) int {
	return SquareOfSums(n) - SumOfSquares(n)
}