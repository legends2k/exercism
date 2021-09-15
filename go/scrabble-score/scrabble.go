package scrabble

import "strings"

func Score(word string) int {
	word = strings.ToLower(word)
	score := [26]int{
		1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3,
		1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10,
	}
	total := 0
	for _, ch := range word {
		total += score[ch-'a']
	}
	return total
}