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

func Sum(list []int64) int64 {
	ans := int64(0)

	for _, el := range list {
		ans += el
	}

	return ans
}

func ParseInt(text string) int64 {
	num, _ := strconv.ParseInt(text, 10, 64)
	return num
}

func part1() int64 {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	ans := int64(0)
	numbers := make([][]int64, 0)
	operators := make([]string, 0)

	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, " ")

		if parts[0] == "+" || parts[0] == "*" {
			for _, part := range parts {
				if part == "" {
					continue
				}
				operators = append(operators, part)
			}
			break
		}

		temp_numbers := make([]int64, 0)
		for _, part := range parts {
			if part == "" {
				continue
			}
			temp_numbers = append(temp_numbers, ParseInt(part))
		}

		numbers = append(numbers, temp_numbers)
	}

	for i, op := range operators {
		temp := int64(0)
		for j := range len(numbers) {
			num := numbers[j][i]

			if op == "+" {
				temp += num
			} else {
				temp = max(1, temp) * num
			}
		}
		ans += temp
	}

	return ans
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
	numbers := make([][]string, 0)
	operators := make([]string, 0)

	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, "")

		if parts[0] == "+" || parts[0] == "*" {
			for _, part := range parts {
				if part == "" {
					continue
				}
				operators = append(operators, part)
			}
			break
		}

		numbers = append(numbers, parts)
	}

	temp := make([]int64, 0)
	operator := "+"

	for i, op := range operators {
		// New calculation
		if op == "+" || op == "*" {
			temp2 := int64(0)
			for _, el := range temp {
				// use previous operator
				if operator == "+" {
					temp2 += el
				} else {
					temp2 = max(1, temp2) * el
				}
			}

			ans += temp2
			temp = make([]int64, 0)
			operator = op
		}

		temp3 := ""
		for j := 0; j < len(numbers); j++ {
			if numbers[j][i] == " " {
				continue
			}
			temp3 += numbers[j][i]
		}

		if temp3 != "" {
			temp = append(temp, ParseInt(temp3))
		}
	}

	// final
	temp2 := int64(0)
	for _, el := range temp {
		// use previous operator
		if operator == "+" {
			temp2 += el
		} else {
			temp2 = max(1, temp2) * el
		}
	}

	ans += temp2

	return ans
}
