package bob

import (
	"io"
	"strings"
	"unicode"
)

// Hey is the solution to this exercise.
func Hey(remark string) string {
	hasLetters := false
	empty := true
	allCaps := true
	isQuestion := false
	r := strings.NewReader(remark)
	for {
		c, _, err := r.ReadRune()
		if err == io.EOF {
			break
		} else if err != nil {
			panic(err)
		}
		if unicode.IsSpace(c) {
			continue
		}
		empty = false
		isQuestion = false
		if unicode.IsLower(c) {
			allCaps = false
			hasLetters = true
		}
		if unicode.IsUpper(c) {
			allCaps = allCaps && true
			hasLetters = true
		}
		if c == '?' {
			isQuestion = true
		}
	}

	if empty {
		return "Fine. Be that way!"
	}

	switch {
	case hasLetters && isQuestion && allCaps:
		return "Calm down, I know what I'm doing!"
	case isQuestion:
		return "Sure."
	case hasLetters && allCaps:
		return "Whoa, chill out!"
	default:
		return "Whatever."
	}
}
