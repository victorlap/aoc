package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	println("Part 1:", part1())
	println("Part 2:", part2())
}

func part1() int64 {
	boxes := ParseBoxes()
	distances := make([]Distance, 0)
	c2b := make(map[int64]int64, len(boxes))

	for i, a := range boxes {
		c2b[int64(i)] = int64(i)
		for _, b := range boxes[i+1:] {
			dist := a.Distance(b)
			distances = append(distances, Distance{dist, a, b})
		}
	}

	sort.Slice(distances, func(i, j int) bool {
		return distances[i].dist < distances[j].dist
	})

	connections := 1000
	if len(boxes) == 20 {
		connections = 10
	}
	for _, distance := range distances[:connections] {
		from_circuit := c2b[distance.b.id]
		for i := range c2b {
			if c2b[i] == from_circuit {
				c2b[i] = c2b[distance.a.id]
			}
		}
	}

	circuits := make([]int64, 0)
	circuits_count := make(map[int64]int64, len(boxes))
	for _, el := range c2b {
		if circuits_count[el] == 0 {
			circuits = append(circuits, el)
		}
		circuits_count[el]++
	}

	sort.SliceStable(circuits, func(i, j int) bool {
		return circuits_count[circuits[i]] > circuits_count[circuits[j]]
	})

	return circuits_count[circuits[0]] * circuits_count[circuits[1]] * circuits_count[circuits[2]]
}

func part2() int64 {
	boxes := ParseBoxes()
	distances := make([]Distance, 0)
	c2b := make(map[int64]int64, len(boxes))

	for i, a := range boxes {
		c2b[int64(i)] = int64(i)
		for _, b := range boxes[i+1:] {
			dist := a.Distance(b)
			distances = append(distances, Distance{dist, a, b})
		}
	}

	sort.Slice(distances, func(i, j int) bool {
		return distances[i].dist < distances[j].dist
	})

	num_circuits := len(boxes)
	for _, distance := range distances {
		to_circuit := c2b[distance.a.id]
		from_circuit := c2b[distance.b.id]

		if to_circuit == from_circuit {
			continue
		}

		for i := range c2b {
			if c2b[i] == from_circuit {
				c2b[i] = c2b[distance.a.id]
			}
		}

		num_circuits--
		if num_circuits == 1 {
			return distance.a.x * distance.b.x
		}
	}

	panic("Oh 0h")
}

type Box struct {
	x  int64
	y  int64
	z  int64
	id int64
}

func (a Box) Distance(b Box) int64 {
	return Sqrt(Pow(a.x-b.x) + Pow(a.y-b.y) + Pow(a.z-b.z))
}

func (a Box) ToString() string {
	return fmt.Sprint(a.x, a.y, a.z)
}

type Distance struct {
	dist int64
	a    Box
	b    Box
}

func ParseInt(text string) int64 {
	num, _ := strconv.ParseInt(text, 10, 64)
	return num
}

func ParseBox(id int64, input string) Box {
	parts := strings.Split(input, ",")

	return Box{
		ParseInt(parts[0]),
		ParseInt(parts[1]),
		ParseInt(parts[2]),
		id,
	}
}

func ParseBoxes() []Box {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	boxes := make([]Box, 0)
	i := int64(0)

	for scanner.Scan() {
		line := scanner.Text()
		boxes = append(boxes, ParseBox(i, line))
		i++
	}

	return boxes
}

func Abs(in int64) int64 {
	return int64(math.Abs(float64(in)))
}
func Sqrt(in int64) int64 {
	return int64(math.Sqrt(float64(in)))
}
func Pow(in int64) int64 {
	return int64(math.Pow(float64(in), 2))
}
