package paasio

import (
	"io"
	"sync/atomic"
)

type CountingReader struct {
	r     io.Reader
	n     int64
	nops  int64
	ready uint32 // atomic flag used for synchronisation
}

func (cr *CountingReader) Read(p []byte) (n int, err error) {
	n, err = cr.r.Read(p)
	// spin lock
	for !atomic.CompareAndSwapUint32(&cr.ready, 0, 1) {
	}
	cr.n = cr.n + int64(n)
	cr.nops = cr.nops + 1
	atomic.StoreUint32(&cr.ready, 0)
	return
}

// Important criterion here is that both n and nops should agree
// i.e. say 20 50-byte reads should give (1000, 20) as result.
// This rules out making two atomic integer addition, as the
// operations themselves are atomic but not together
func (cr *CountingReader) ReadCount() (n int64, nops int) {
	for !atomic.CompareAndSwapUint32(&cr.ready, 0, 1) {
	}
	n = cr.n
	nops = int(cr.nops)
	atomic.StoreUint32(&cr.ready, 0)
	return
}

func NewReadCounter(r io.Reader) ReadCounter {
	return &CountingReader{r, 0, 0, 0}
}

type CountingWriter struct {
	w     io.Writer
	n     int64
	nops  int64
	ready uint32
}

func (cw *CountingWriter) Write(p []byte) (n int, err error) {
	n, err = cw.w.Write(p)
	for !atomic.CompareAndSwapUint32(&cw.ready, 0, 1) {
	}
	cw.n = cw.n + int64(n)
	cw.nops = cw.nops + 1
	atomic.StoreUint32(&cw.ready, 0)
	return
}

func (cw *CountingWriter) WriteCount() (n int64, nops int) {
	for !atomic.CompareAndSwapUint32(&cw.ready, 0, 1) {
	}
	n = cw.n
	nops = int(cw.nops)
	atomic.StoreUint32(&cw.ready, 0)
	return
}

func NewWriteCounter(r io.Writer) WriteCounter {
	return &CountingWriter{r, 0, 0, 0}
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
