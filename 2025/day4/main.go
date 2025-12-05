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

func Grid() [][]string {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	grid := make([][]string, 0)

	for scanner.Scan() {
		line := scanner.Text()
		grid = append(grid, strings.Split(line, ""))
	}

	return grid
}

func Neighbours(grid [][]string, x int, y int) int {
	ans := 0

	for i := -1; i <= 1; i++ {
		for j := -1; j <= 1; j++ {
			if (i == 0 && j == 0) || x+i < 0 || (x+i >= len(grid)) || y+j < 0 || (y+j >= len(grid)) {
				continue
			}

			if grid[y+j][x+i] == "@" {
				ans++
			}
		}
	}

	return ans
}

func Remove(grid [][]string) (int, [][]string) {
	ans := 0

	for y, line := range grid {
		for x, cell := range line {
			neighbours := Neighbours(grid, x, y)

			if cell == "@" && neighbours < 4 {
				grid[y][x] = "x"
				ans++
			}
		}
	}

	return ans, grid
}

func part1() int {
	grid := Grid()
	ans := 0

	for y, line := range grid {
		for x, cell := range line {
			neighbours := Neighbours(grid, x, y)

			if cell == "@" && neighbours < 4 {
				ans++
			}
		}
	}

	return ans
}

func part2() int {
	grid := Grid()
	ans := 0

	for true {
		removed, new_grid := Remove(grid)

		if removed == 0 {
			break
		}

		ans += removed
		grid = new_grid
	}

	return ans
}
