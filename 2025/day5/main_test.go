package main

import "testing"

func Test(t *testing.T) {
	if part1() != 798 {
		t.Errorf("Invalid part 1")
	}
	if part2() != 366181852921027 {
		t.Errorf("Invalid part 2")
	}
}
