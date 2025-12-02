package main

import "testing"

func Test(t *testing.T) {
	if part1() != 0 {
		t.Errorf("Invalid part 1")
	}
	if part2() != 0 {
		t.Errorf("Invalid part 2")
	}
}
