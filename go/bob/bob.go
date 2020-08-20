package bob

import (
	"regexp"
	"strings"
)

// Hey is the solution to this exercise.
func Hey(remark string) string {
	remark = strings.TrimSpace(remark)
	if remark == "" {
		return "Fine. Be that way!"
	}

	hasAlpha, err := regexp.Match("[a-zA-Z]", []byte(remark))
	if err != nil {
		panic(err)
	}

	allCaps := hasAlpha && strings.ToUpper(remark) == remark
	isQuestion := strings.HasSuffix(remark, "?")

	switch {
	case isQuestion && allCaps:
		return "Calm down, I know what I'm doing!"
	case isQuestion:
		return "Sure."
	case allCaps:
		return "Whoa, chill out!"
	default:
		return "Whatever."
	}
}
