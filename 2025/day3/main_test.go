package main

import "testing"

func Test(t *testing.T) {
	if part1() != 17554 {
		t.Errorf("Invalid part 1")
	}
	if part2() != 175053592950232 {
		t.Errorf("Invalid part 2")
	}
}
