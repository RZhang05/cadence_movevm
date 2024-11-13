package main

import (
	"testing"
)

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

func BenchmarkMoveVMCreateStruct(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		MoveVMCreateStruct()
	}
}

func BenchmarkMoveVMCallGoExtern(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		MoveVMEmptyExtern()
	}
}
