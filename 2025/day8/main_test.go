package main

import "testing"

func Test(t *testing.T) {
	if part1() != 112230 {
		t.Errorf("Invalid part 1")
	}
	if part2() != 2573952864 {
		t.Errorf("Invalid part 2")
	}
}
