package main

import "testing"

func Test(t *testing.T) {
	if part1() != 1154 {
		t.Errorf("Invalid part 1")
	}
	if part2() != 6819 {
		t.Errorf("Invalid part 2")
	}
}
