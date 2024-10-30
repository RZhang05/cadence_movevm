package main

import (
	"testing"
)

func BenchmarkMoveVMRecFib(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		MoveVMRecFib(14)
	}
}

func BenchmarkMoveVMFibRecEmpty(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		MoveVMRecFib(1)
	}
}

func BenchmarkMoveVMImpFib(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		MoveVMImpFib(14)
	}
}

func BenchmarkMoveVMFibImpEmpty(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		MoveVMImpFib(1)
	}
}

func BenchmarkMoveVMLoadOnly(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		MoveVMLoad()
	}
}
