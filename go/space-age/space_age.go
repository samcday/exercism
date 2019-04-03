package space

// Planet is a string. For some reason.
type Planet = string

const earthOrbitalPeriod = 31557600

var planetOrbitalPeriods = map[Planet]float64{
	"Earth":   earthOrbitalPeriod,
	"Mercury": earthOrbitalPeriod * 0.2408467,
	"Venus":   earthOrbitalPeriod * 0.61519726,
	"Mars":    earthOrbitalPeriod * 1.8808158,
	"Jupiter": earthOrbitalPeriod * 11.862615,
	"Saturn":  earthOrbitalPeriod * 29.447498,
	"Uranus":  earthOrbitalPeriod * 84.016846,
	"Neptune": earthOrbitalPeriod * 164.79132,
}

// Age calculates the age of someone on a particular celestial body.
func Age(seconds float64, planet Planet) float64 {
	return seconds / planetOrbitalPeriods[planet]
}
