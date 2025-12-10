package main

import "testing"

func Test(t *testing.T) {
	if part1() != 1499 {
		t.Errorf("Invalid part 1")
	}
	if part2() != 24743903847942 {
		t.Errorf("Invalid part 2")
	}
}
