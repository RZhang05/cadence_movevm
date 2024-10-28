package main

import (
	"testing"
)

func BenchmarkGoFib(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		fib(14)
	}
}

func fib(a int) int {
	if a < 2 {
		return a
	}

	return fib(a-1) + fib(a-2)
}

func BenchmarkRustFib(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		Fib(14)
	}
}

func BenchmarkCadenceFib(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		CadenceFib(14)
	}
}
