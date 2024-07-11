package main

import (
	"bytes"
	"testing"
)

// TestEncode calls Encode function with some uint64
// and return the encoded bytes for variable width integer.
func TestEncode(t *testing.T) {
	encodeTests := []struct {
		n        uint64
		expected []byte
	}{
		{150, []byte{0x96, 0x01}},
		{1, []byte{0x01}},
		{18446744073709551615, []byte{
			0xff, 0xff, 0xff, 0xff, 0xff,
			0xff, 0xff, 0xff, 0xff, 0x01}},
	}

	for _, v := range encodeTests {
		encodedValue := Encode(v.n)
		if bytes.Compare(encodedValue, v.expected) != 0 {
			t.Errorf("got %+v, want %+v", encodedValue, v.expected)
		}
	}
}

// TestDecode calls Decode function with some byte array
// and return the decoded uint64 for variable width integer.
func TestDecode(t *testing.T) {
	decodedTests := []struct {
		expected uint64
		value    []byte
	}{
		{150, []byte{0x96, 0x01}},
		{1, []byte{0x01}},
		{18446744073709551615, []byte{
			0xff, 0xff, 0xff, 0xff, 0xff,
			0xff, 0xff, 0xff, 0xff, 0x01}},
	}

	for _, v := range decodedTests {
		decodedValue := Decode(v.value)
		if decodedValue != v.expected {
			t.Errorf("got %+v, want %+v", decodedValue, v.expected)
		}
	}
}

// TestDecode calls Decode function with some byte array
// and return the decoded uint64 for variable width integer.
func TestDecodeAndEncodeRoundTrip(t *testing.T) {
	for n := range uint64(1 << 30) {
		if Decode(Encode(n)) != n {
			t.Errorf("Encode decode round trip fails")
		}
	}
}
