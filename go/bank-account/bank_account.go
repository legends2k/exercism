// Package account provides accounting facilities
package account

import "sync/atomic"

type Account struct {
	amt     int64
	defunct uint32
}

// Open opens a new account with an initial balance and returns it
// The initial balance should be non-negative
func Open(basic int64) *Account {
	if basic < 0 {
		return nil
	}
	return &Account{amt: basic}
}

// Balance returns the current balance in the account
func (a Account) Balance() (balance int64, ok bool) {
	if a.defunct == 1 {
		return 0, false
	}
	return a.amt, true
}

// Close permenantly closes an account
func (a *Account) Close() (payout int64, ok bool) {
	conclusive := false
	for !conclusive {
		if a.defunct == 1 {
			return 0, false
		}
		conclusive = atomic.CompareAndSwapUint32(&a.defunct, 0, 1)
	}
	payout = a.amt
	a.amt = 0
	ok = true
	return
}

// Deposit deopsits or withdraws from an account; a -ve amount means withdrawal
func (a *Account) Deposit(amt int64) (newBal int64, ok bool) {
	if a.defunct == 1 {
		return 0, false
	}

	// deposits are safe; just add them atomically
	if amt >= 0 {
		return atomic.AddInt64(&a.amt, amt), true
	}

	// withdrawal; make sure decision is based on fresh value
	conclusive := false
	var bal int64
	for !conclusive {
		bal = a.amt // work on a copy, not the hot original
		if -amt > bal {
			return a.amt, false
		}
		conclusive = atomic.CompareAndSwapInt64(&a.amt, bal, amt+bal)
	}
	return (bal + amt), true
}