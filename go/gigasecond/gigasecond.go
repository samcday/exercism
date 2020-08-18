package gigasecond

import (
	"time"
)

const gigasecond = 1000000000 * time.Second

// AddGigasecond returns the given time extended a giga-second (10^9 seconds) into the future.
func AddGigasecond(t time.Time) time.Time {
	return t.Add(gigasecond)
}
