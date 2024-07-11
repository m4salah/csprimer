package main

import (
	"fmt"
	"io"
	"os"
	"regexp"
	"slices"
)

// Reverse returns its argument string reversed rune-wise left to right.
func Reverse(s string) string {
	r := []rune(s)
	for i, j := 0, len(r)-1; i < len(r)/2; i, j = i+1, j-1 {
		r[i], r[j] = r[j], r[i]
	}
	return string(r)
}

func reverseArr(numbers []string) []string {
	for i, j := 0, len(numbers)-1; i < j; i, j = i+1, j-1 {
		numbers[i], numbers[j] = numbers[j], numbers[i]
	}
	return numbers
}

func rchunks(s string) []string {
	result := []string{}
	s = Reverse(s)

	fmt.Println(s)
	for i := 0; i < len(s); i = i + 7 {
		if len(s) < i+7 {
			result = append(result, s[i:])
		} else {
			newStr := Reverse(s[i : i+7])
			result = append(result, newStr)
		}
	}

	result = reverseArr(result)
	fmt.Println(result)
	return result
}

func Decode(varn []byte) uint64 {
	var out uint64 = 0
	// reverse the byte array in plan
	slices.Reverse(varn)

	for _, b := range varn {
		out <<= 7
		out |= uint64(b & 0x7f) // get the lower 7 bites and add it to the accumlator
	}
	return out
}

func Encode(n uint64) []byte {
	out := []byte{}
	for n > 0 {
		part := n & 0x7F
		n >>= 7
		if n > 0 {
			part |= (1 << 7)
		}
		out = append(out, byte(part))
	}
	return out
}

func hToD(c string) uint8 {
	switch c {
	case "0":
		return 0
	case "1":
		return 1
	case "2":
		return 2
	case "3":
		return 3
	case "4":
		return 4
	case "5":
		return 5
	case "6":
		return 6
	case "7":
		return 7
	case "8":
		return 8
	case "9":
		return 9
	case "a":
		return 10
	case "A":
		return 10
	case "b":
		return 11
	case "B":
		return 11
	case "c":
		return 12
	case "C":
		return 12
	case "d":
		return 13
	case "D":
		return 13
	case "e":
		return 14
	case "E":
		return 14
	case "f":
		return 15
	case "F":
		return 15
	}
	return 0
}

func hhToD(hh string) uint8 {
	var result uint8 = 0
	slices.Reverse([]byte(hh))
	for i, b := range hh {
		result |= hToD(string(b)) << 4 * uint8(i)
	}return
	return result
}

func main() {
	stdin, err := io.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	re := regexp.MustCompile("#[0-9a-fA-F]{3,8}")
	newcss := re.ReplaceAllFunc(stdin, func(b []byte) []byte {
		word := string(b)[1:]
		for i := 0; i < len(string(b))-1; i += 2 {
			hh := word[i : i+2]
			fmt.Println(hhToD(hh))
		}
		return []byte("rgb()" + "bar")
	})
	fmt.Println(string(newcss))
}
