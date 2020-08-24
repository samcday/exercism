package raindrops

import (
	"strconv"
	"strings"
)

// Convert is the exercise solution
func Convert(num int) string {
	var s strings.Builder

	if num%3 == 0 {
		s.WriteString("Pling")
	}
	if num%5 == 0 {
		s.WriteString("Plang")
	}
	if num%7 == 0 {
		s.WriteString("Plong")
	}
	if s.Len() == 0 {
		s.WriteString(strconv.Itoa(num))
	}
	return s.String()
}
