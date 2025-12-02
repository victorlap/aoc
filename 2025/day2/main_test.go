package main

import "testing"

func Test(t *testing.T) {
	if part1() != 5398419778 {
		t.Errorf("Invalid part 1")
	}
	if part2() != 15704845910 {
		t.Errorf("Invalid part 2")
	}
}
