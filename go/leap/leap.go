package leap

// IsLeapYear calculates if a given year is a leap year.
func IsLeapYear(year int) bool {
	// If a year is evenly divisible by 4 it is a leap year ...
	if year%4 == 0 {
		// ... unless the year is also divisible by 100, in which case it must
		// also be divisble by 400.
		if year%100 > 0 || year%400 == 0 {
			return true
		}
	}
	return false
}
