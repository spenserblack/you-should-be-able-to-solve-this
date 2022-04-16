package main

import "testing"

// Asserts that x - 7 = 19 + x
func TestXMinus7Equals19PlusX(t *testing.T) {
	if x - 7 != 19 + x {
		t.Fatalf("x - 7 does not equal 19 + x")
	}
}
