package isogram

import (
	"unicode"
)

func IsIsogram(s string) bool {
	seen := make(map[rune]struct{}, len(s))
	for _, r := range s {
		if !unicode.IsLetter(r) {
			continue
		}
		r = unicode.ToLower(r)
		if _, exists := seen[r]; exists {
			return false
		}
		seen[r] = struct{}{}
	}
	return true
}
