package main

import (
	"testing"
)

func BenchmarkMoveVMComposite(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		// too unfair to create 10 composites here since this requires reloading and instantiating all modules 10 times as well
		MoveVMCreateComposite()
	}
}

func BenchmarkMoveVMRecFib14(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		MoveVMRecFib(14)
	}
}

func BenchmarkMoveVMRecFib1(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		MoveVMRecFib(1)
	}
}

func BenchmarkMoveVMImpFib14(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		MoveVMImpFib(14)
	}
}

func BenchmarkMoveVMImpFib1(b *testing.B) {
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