package main

import (
	"bufio"
	"log"
	"os"
	"strings"
)

func main() {
	println("Part 1:", part1())
	println("Part 2:", part2())
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
	prev := make([]string, 0)

	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, "")

		if len(prev) == 0 {
			prev = parts
			continue
		}

		for i, cell := range parts {
			cell_is_empty := cell == "."
			cell_is_splitter := cell == "^"
			prev_is_beam := prev[i] == "|" || prev[i] == "S"

			if prev_is_beam && cell_is_empty {
				parts[i] = "|"
			}

			if prev_is_beam && cell_is_splitter {
				ans++
				if i-1 > 0 && parts[i-1] == "." {
					parts[i-1] = "|"
				}
				if i+1 < len(parts) && parts[i+1] == "." {
					parts[i+1] = "|"
				}
			}
		}

		prev = parts
	}

	return ans
}

func part2() int {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	prev := make([]string, 0)
	grid := make([]int, 0)

	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, "")
		new_grid := make([]int, len(parts))

		if len(prev) == 0 {
			prev = parts
			grid = new_grid
			continue
		}

		for i, cell := range parts {
			cell_is_splitter := cell == "^"
			prev_is_beam := prev[i] == "|" || prev[i] == "S"

			if prev_is_beam && cell != "^" {
				parts[i] = "|"
				new_grid[i] += max(1, grid[i])
			}

			if prev_is_beam && cell_is_splitter {
				if i-1 > 0 && parts[i-1] != "^" {
					parts[i-1] = "|"
					new_grid[i-1] += grid[i]
				}
				if i+1 < len(parts) && parts[i+1] != "^" {
					parts[i+1] = "|"
					new_grid[i+1] += grid[i]
				}
			}
		}

		prev = parts
		grid = new_grid
	}

	return Sum(grid) + 1
}

func Sum(list []int) int {
	ans := 0

	for _, el := range list {
		ans += el
	}

	return ans
}
