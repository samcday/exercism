package scrabble

import "unicode"

var letterScores map[rune]int

func init() {
	letterScores = make(map[rune]int)

	for _, r := range []rune{'a', 'e', 'i', 'o', 'u', 'l', 'n', 'r', 's', 't'} {
		letterScores[r] = 1
	}
	letterScores['d'] = 2
	letterScores['g'] = 2
	for _, r := range []rune{'b', 'c', 'm', 'p'} {
		letterScores[r] = 3
	}
	for _, r := range []rune{'f', 'h', 'v', 'w', 'y'} {
		letterScores[r] = 4
	}
	letterScores['k'] = 5
	letterScores['j'] = 8
	letterScores['x'] = 8
	letterScores['q'] = 10
	letterScores['z'] = 10
}

// Score is the exercise solution.
func Score(word string) int {
	score := 0
	for _, r := range word {
		score += letterScores[unicode.ToLower(r)]
	}
	return score
}
