package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	println("Part 1:", part1())
	println("Part 2:", part2())
}

func part1() int64 {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)
	scanner.Scan()
	input := scanner.Text()
	parts := strings.Split(input, ",")

	counter := int64(0)

	for _, part := range parts {
		parts2 := strings.Split(part, "-")
		from, _ := strconv.ParseInt(string(parts2[0]), 10, 0)
		to, _ := strconv.ParseInt(string(parts2[1]), 10, 0)

		for i := from; i <= to; i++ {
			num := fmt.Sprint(i)

			if len(num)%2 != 0 {
				continue
			}

			left := num[len(num)/2:]
			right := num[:len(num)/2]
			if left == right {
				// println(num)
				counter += i
			}

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
	scanner.Scan()
	input := scanner.Text()
	parts := strings.Split(input, ",")

	counter := int64(0)

	for _, part := range parts {
		parts2 := strings.Split(part, "-")
		from, _ := strconv.ParseInt(string(parts2[0]), 10, 0)
		to, _ := strconv.ParseInt(string(parts2[1]), 10, 0)

		for i := from; i <= to; i++ {
			num_orig := fmt.Sprint(i)

		inner:
			for j := 1; j <= len(num_orig)/2; j++ {
				if len(num_orig)%j != 0 {
					continue
				}
				num := num_orig

				for len(num) > j {
					if num[:j] != num[j:2*j] {
						continue inner
					}
					num = num[j:]
				}

				counter += i
				break inner
			}
		}
	}

	return counter
}
