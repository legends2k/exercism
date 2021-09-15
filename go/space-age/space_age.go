// Package space provides the Planet type with facilities like seconds
package space

const secsInAYear = 24 * 60 * 60 * 365.25

type Planet string

var planetDuration map[Planet](float64)

func init() {
	planetDuration = map[Planet](float64){
		"Mercury": 0.2408467 * secsInAYear,
		"Venus":   0.61519726 * secsInAYear,
		"Earth":   1.0 * secsInAYear,
		"Mars":    1.8808158 * secsInAYear,
		"Jupiter": 11.862615 * secsInAYear,
		"Saturn":  29.447498 * secsInAYear,
		"Uranus":  84.016846 * secsInAYear,
		"Neptune": 164.79132 * secsInAYear,
	}
}

// Seconds returns the orbital period of a planet in Earth seconds
func (p Planet) Seconds() float64 {
	return planetDuration[p]
}

func Age(sec float64, p Planet) float64 {
	return sec / p.Seconds()
}