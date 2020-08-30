package reverse

import (
	"unicode/utf8"
)

func Reverse(s string) string {
	new := make([]rune, utf8.RuneCountInString(s))
	i := len(new) - 1
	for _, r := range s {
		new[i] = r
		i--
	}
	return string(new)
}
