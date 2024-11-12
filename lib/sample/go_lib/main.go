package main

import (
	"C"

	"github.com/onflow/cadence/common"
)

//export DoSomething
func DoSomething(str string) interface{} {
	return C.CString("hello")
}

//export GetMember
func GetMember(v interface{}) interface{} {
	var str = "failed"
	switch v:= v.(type) {
	// v is expected to be a compositeValue
	case *CompositeValue:
			str = v.QualifiedIdentifier
	}
	return C.CString(str)
}

//export CreateComposite
func CreateComposite(
	moveLoc string,
	moveKind uint,
	moveQualifiedIdentifier string,
	//fields []interpreter.CompositeField,
	moveAddress string,
) interface{} {
	// TODO: derive these fields from params
	var runtime = NewMoveRuntime()
	var location = NewAddressLocationFromHex(moveAddress, moveQualifiedIdentifier)
	var kind common.CompositeKind =  common.CompositeKind(moveKind)
	var address common.Address = common.ZeroAddress

	return NewCompositeValue(
		runtime,
		location,
		moveQualifiedIdentifier,
		kind,
		//fields
		address,
	)
}

func main() {}
