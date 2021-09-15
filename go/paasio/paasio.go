package paasio

import (
	"io"
	"sync/atomic"
)

type CountingReader struct {
	r    io.Reader
	n    int64
	nops int64
}

func (cr *CountingReader) Read(p []byte) (n int, err error) {
	n, err = cr.r.Read(p)
	atomic.AddInt64(&cr.n, int64(n))
	atomic.AddInt64(&cr.nops, 1)
	return
}

func (cr *CountingReader) ReadCount() (n int64, nops int) {
	return cr.n, int(cr.nops)
}

func NewReadCounter(r io.Reader) ReadCounter {
	return &CountingReader{r, 0, 0}
}

type CountingWriter struct {
	r    io.Writer
	n    int64
	nops int64
}

func (cr *CountingWriter) Write(p []byte) (n int, err error) {
	n, err = cr.r.Write(p)
	atomic.AddInt64(&cr.n, int64(n))
	atomic.AddInt64(&cr.nops, 1)
	return
}

func (cr *CountingWriter) WriteCount() (n int64, nops int) {
	return cr.n, int(cr.nops)
}

func NewWriteCounter(r io.Writer) WriteCounter {
	return &CountingWriter{r, 0, 0}
}

type CountingRW struct {
	cr ReadCounter
	cw WriteCounter
}

func (crw *CountingRW) Read(p []byte) (n int, err error) {
	return crw.cr.Read(p)
}

func (crw *CountingRW) Write(p []byte) (n int, err error) {
	return crw.cw.Write(p)
}

func (crw *CountingRW) ReadCount() (n int64, nops int) {
	return crw.cr.ReadCount()
}

func (crw *CountingRW) WriteCount() (n int64, nops int) {
	return crw.cw.WriteCount()
}

func NewReadWriteCounter(rw io.ReadWriter) ReadWriteCounter {
	return &CountingRW{NewReadCounter(rw), NewWriteCounter(rw)}
}
