// Package robotname deals with the serial number generation of robots
package robotname

import "fmt"

type Robot struct {
	name string
}

var id, seq uint32

// randomName generates (2^26) * 1000 = 67108864000 serial numbers
func newSerial() string {
	i := seq / 26
	j := seq % 26
	var a uint32 = 'A'
	name := fmt.Sprintf("%c%c%03d", a+i, a+j, id)
	id = id + 1
	if id >= 1000 {
		id = 0
		seq++
	}
	return name
}

// Name returns the unique serial number of a robot
func (r *Robot) Name() string {
	if r.name == "" {
		r.name = newSerial()
	}
	return r.name
}

// Reset resets the robot’s serial number
func (r *Robot) Reset() {
	r.name = newSerial()
}