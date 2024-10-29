package main

import (
	"testing"
)

func BenchmarkMoveVMFib(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		MoveVMFib(14)
	}
}

func BenchmarkMoveVMFibEmpty(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		MoveVMFib(1)
	}
}

func BenchmarkMoveVMLoadOnly(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		MoveVMLoad()
	}
}
