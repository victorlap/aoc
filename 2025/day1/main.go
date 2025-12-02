package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
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

	cur := int64(50)
	counter := 0

	for scanner.Scan() {
		value := []rune(scanner.Text())
		dir := value[0]
		rot, err := strconv.ParseInt(string(value[1:]), 10, 0)
		if err != nil {
			log.Fatal(err)
		}

		if dir == 'L' {
			cur = (cur - rot) % 100
		} else {
			cur = (cur + rot) % 100
		}

		if cur == 0 {
			counter++
		}
	}

	return counter
}

func part2() int64 {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	prev, cur := int64(50), int64(50)
	counter := int64(0)

	for scanner.Scan() {
		prev = cur
		value := []rune(scanner.Text())
		dir := value[0]
		rot, err := strconv.ParseInt(string(value[1:]), 10, 0)

		if err != nil {
			log.Fatal(err)
		}

		// Full rotations
		full := rot / 100
		counter += full

		rest := (rot + 100) % 100

		if dir == 'L' {
			cur -= rest
			if cur < 0 {
				cur += 100
				if prev != 0 {
					counter++
				}
			}
		} else {
			cur += rest
			if cur >= 100 {
				cur -= 100
				if cur != 0 {
					counter++
				}
			}
		}

		if cur == 0 {
			counter++
		}
	}

	return counter
}
