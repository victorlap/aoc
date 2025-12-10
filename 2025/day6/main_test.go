package main

import "testing"

func Test(t *testing.T) {
	if part1() != 5782351442566 {
		t.Errorf("Invalid part 1")
	}
	if part2() != 10194584711842 {
		t.Errorf("Invalid part 2")
	}
}
