package acronym

import (
	"strings"
	"unicode"
)

// Abbreviate is the exercise solution
func Abbreviate(s string) string {
	var acronym strings.Builder
	firstLetter := true

	for _, r := range s {
		switch {
		case unicode.IsSpace(r) || r == '-':
			firstLetter = true
		case firstLetter && unicode.IsLetter(r):
			acronym.WriteRune(unicode.ToUpper(r))
			firstLetter = false
		}
	}

	return acronym.String()
}
