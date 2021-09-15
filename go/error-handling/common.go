// Package erratum is to learn about error, panic and recovery
package erratum

import (
	"io"
)

var testVersion = 2

// These are the support types and interface definitions used in the
// implementation if your Use function. See the test suite file at
// for information on the expected implementation.
//
// Because this is part of the package "erratum", if your solution file
// is also declared in the package you will automatically have access to
// these definitions (you do not have to re-declare them).

// TransientError is an error that may occur while opening a resource via
// ResourceOpener.
type TransientError struct {
	err error
}

func (e TransientError) Error() string {
	return e.err.Error()
}

// FrobError is a possible error from doing some frobbing, your implementation
// will require calling your Resource's Defrob(string) method.
// When this error occurs, the FrobError's defrobTag string will contain the
// string you must pass into Defrob.
type FrobError struct {
	defrobTag string
	inner     error
}

func (e FrobError) Error() string {
	return e.inner.Error()
}

type Resource interface {

	// Resource is using composition to inherit the requirements of the io.Closer
	// interface. What this means is that a Resource implementation will be
	// expected to have a .Close() method too.
	io.Closer

	// Frob does something with the input string.
	// Because this is an incredibly badly designed system if there is an error
	// it panics.
	//
	// The panicked error may be a FrobError in which case Defrob should be
	// called with the .defrobTag string property of the error.
	Frob(string)

	Defrob(string)
}

// ResourceOpener is a function that creates a resource.
//
// It may return a wrapped error of type TransientError. In this case the resource
// is temporarily unavailable and the caller should retry soon.
type ResourceOpener func() (Resource, error)

func Use(o ResourceOpener, input string) (err error) {
	var res Resource
	for {
		res, err = o()
		if err != nil {
			if _, ok := err.(TransientError); ok {
				continue
			}
			return err
		}
		break
	}
	defer res.Close()
	defer func() {
		if r := recover(); r != nil {
			err = r.(error)
			fe, ok := err.(FrobError)
			if ok {
				res.Defrob(fe.defrobTag)
			}
		}
	}()
	res.Frob(input)
	return err
}
