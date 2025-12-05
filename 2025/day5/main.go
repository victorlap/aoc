package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	println("Part 1:", part1())
	println("Part 2:", part2())
}

func ParseInt(text string) int64 {
	num, _ := strconv.ParseInt(text, 10, 64)
	return num
}

func ParseRange(text string) []int64 {
	parts := strings.Split(text, "-")

	range_ := make([]int64, 0)

	range_ = append(range_, ParseInt(parts[0]))
	range_ = append(range_, ParseInt(parts[1]))

	return range_
}

func part1() int {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	ans := 0
	is_range := true
	ranges := make([][]int64, 0)

	for scanner.Scan() {
		line := scanner.Text()

		if line == "" {
			is_range = false
		}

		if is_range {
			ranges = append(ranges, ParseRange(line))
		} else {
			num := ParseInt(line)

			for _, range_ := range ranges {
				if num >= range_[0] && num <= range_[1] {
					ans++
					break
				}
			}
		}
	}

	return ans
}

func Remove(ranges [][]int64) (int64, [][]int64) {
	new_ranges := make([][]int64, 0)
	removed := int64(0)

	for i := len(ranges) - 1; i >= 0; i-- {
		range1 := ranges[i]
		found := false

		for j := range i {
			range2 := ranges[j]

			if range1[0] == range2[0] && range1[1] == range2[1] {
				removed++
				found = true
				break
			}

			if (range1[0] >= range2[0] && range1[0] <= range2[1]) ||
				(range1[1] >= range2[0] && range1[1] <= range2[1]) {
				new_range := make([]int64, 0)
				new_range = append(new_range, min(range1[0], range2[0]))
				new_range = append(new_range, max(range1[1], range2[1]))

				new_ranges = append(new_ranges, new_range)

				removed++
				found = true
				break
			}
		}

		if !found {
			new_ranges = append(new_ranges, range1)
		}
	}

	return removed, new_ranges
}

func part2() int64 {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	ans := int64(0)
	ranges := make([][]int64, 0)

	for scanner.Scan() {
		line := scanner.Text()

		if line == "" {
			break
		}

		ranges = append(ranges, ParseRange(line))
	}

	for true {
		removed, new_ranges := Remove(ranges)

		if removed == 0 {
			break
		}

		ranges = new_ranges
	}

	for _, range_ := range ranges {
		ans += range_[1] - range_[0] + 1
	}

	return ans
}
