package paasio

import (
	"io"
	"sync/atomic"
)

type CountingReader struct {
	r io.Reader
	// Note: pack both byte and op count in one uint64 to use an atomic
	// instead of a mutex. In some sense, this is cheating since the byte
	// count itself can be as large as int64.
	cnt uint64
}

func (cr *CountingReader) Read(p []byte) (n int, err error) {
	n, err = cr.r.Read(p)
	comitted := false
	for !comitted {
		org_val := atomic.LoadUint64(&cr.cnt)
		val := org_val
		val = val + uint64(n)   // lower bits have byte count, simply add!
		val = val + 0x100000000 // add one to ops
		comitted = atomic.CompareAndSwapUint64(&cr.cnt, org_val, val)
	}
	return
}

func (cr *CountingReader) ReadCount() (n int64, nops int) {
	cnt := atomic.LoadUint64(&cr.cnt)
	n = int64(cnt & 0xFFFFFFFF)
	nops = int(cnt >> 32)
	return
}

func NewReadCounter(r io.Reader) ReadCounter {
	return &CountingReader{r, 0}
}

type CountingWriter struct {
	r   io.Writer
	cnt uint64
}

func (cr *CountingWriter) Write(p []byte) (n int, err error) {
	n, err = cr.r.Write(p)
	comitted := false
	for !comitted {
		org_val := atomic.LoadUint64(&cr.cnt)
		val := org_val
		val = val + uint64(n)   // lower bits have byte count, simply add!
		val = val + 0x100000000 // add one to ops
		comitted = atomic.CompareAndSwapUint64(&cr.cnt, org_val, val)
	}
	return
}

func (cr *CountingWriter) WriteCount() (n int64, nops int) {
	cnt := atomic.LoadUint64(&cr.cnt)
	n = int64(cnt & 0xFFFFFFFF)
	nops = int(cnt >> 32)
	return
}

func NewWriteCounter(r io.Writer) WriteCounter {
	return &CountingWriter{r, 0}
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
