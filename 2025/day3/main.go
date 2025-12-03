package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	println("Part 1:", part1())
	println("Part 2:", part2())
}

func MakeNumeric(list []string) []int64 {
	res := make([]int64, 0)

	for _, el := range list {
		num, _ := strconv.ParseInt(el, 10, 8)
		res = append(res, num)
	}

	return res
}

func IndexOf(list []int64, candidate int64) int {
	for index, c := range list {
		if c == candidate {
			return index
		}
	}
	return -1
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

	for scanner.Scan() {
		line := scanner.Text()
		bank := MakeNumeric(strings.Split(line, ""))

	all:
		for i := int64(9); i >= 0; i-- {
			bankCopy := bank
			for firstIndex := IndexOf(bankCopy, i); firstIndex >= 0 && len(bankCopy) > 0; {
				bankCopyTwo := bankCopy[firstIndex+1:]
				for j := int64(9); j >= 0; j-- {
					for secondIndex := IndexOf(bankCopyTwo, j); secondIndex >= 0; {
						ans += (i * 10) + j
						break all
					}
				}
				bankCopy = bankCopy[firstIndex+1:]
			}
		}
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

	for scanner.Scan() {
		line := scanner.Text()
		bank := MakeNumeric(strings.Split(line, ""))

		batteries := 12
		bank_ans, _ := FindOptimal(bank, batteries)
		ans += bank_ans
	}

	return ans
}

func FindOptimal(bank []int64, batteries int) (int64, error) {
	for i := int64(9); i >= 0; i-- {
		for index := IndexOf(bank, i); index >= 0 && index <= len(bank)-batteries; {
			if batteries > 1 {
				rest, err := FindOptimal(bank[index+1:], batteries-1)
				if err != nil {
					continue
				}
				return i*int64(math.Pow10(batteries-1)) + rest, nil
			} else {
				return i, nil
			}
		}
	}

	return -1, fmt.Errorf("Could not find optimal")
}
