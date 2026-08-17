package utils

import (
	"context"
	"sync"
)

var closedPromiseChan = make(chan struct{})

func init() {
	close(closedPromiseChan)
}

type Promise[T any] struct {
	mu     sync.Mutex
	done   chan struct{}
	Result T
	Err    error
}

func NewPromise[T any]() *Promise[T] {
	return &Promise[T]{
		done: make(chan struct{}),
	}
}

func GoPromise[T any](f func() (T, error)) *Promise[T] {
	p := NewPromise[T]()
	go func() { p.Resolve(f()) }()
	return p
}

func NewResolvedPromise[T any](result T, err error) *Promise[T] {
	return &Promise[T]{
		Result: result,
		Err:    err,
		done:   closedPromiseChan,
	}
}

func (p *Promise[T]) Resolve(result T, err error) {
	p.mu.Lock()
	defer p.mu.Unlock()

	if p.done == closedPromiseChan {
		return
	}

	p.Result = result
	p.Err = err

	if p.done != nil {
		close(p.done)
	}
	p.done = closedPromiseChan
}

func (p *Promise[T]) Done() <-chan struct{} {
	p.mu.Lock()
	defer p.mu.Unlock()
	if p.done == nil {
		p.done = make(chan struct{})
	}
	return p.done
}

func (p *Promise[T]) Resolved() bool {
	p.mu.Lock()
	defer p.mu.Unlock()
	return p.done == closedPromiseChan
}

func AwaitPromise[T any](ctx context.Context, p *Promise[T]) (T, error) {
	select {
	case <-ctx.Done():
		var v T
		return v, ctx.Err()
	case <-p.Done():
		return p.Result, p.Err
	}
}
