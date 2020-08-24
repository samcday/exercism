package acronym

import (
	"unicode"
)

// Abbreviate is the exercise solution
func Abbreviate(s string) string {
	acronym := ""
	firstLetter := true

	for _, r := range s {
		switch {
		case unicode.IsSpace(r) || r == '-':
			firstLetter = true
		case firstLetter && unicode.IsLetter(r):
			acronym += string(unicode.ToUpper(r))
			firstLetter = false
		}
	}

	return acronym
}
