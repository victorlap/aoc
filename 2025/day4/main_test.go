package main

import "testing"

func Test(t *testing.T) {
	ans := part1()
	if ans != 1351 {
		t.Errorf("Invalid part 1, got %d", ans)
	}

	ans = part2()
	if ans != 8345 {
		t.Errorf("Invalid part 2, got %d", ans)
	}
}
