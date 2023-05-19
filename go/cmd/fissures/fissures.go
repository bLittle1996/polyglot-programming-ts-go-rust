package main

import (
	"fmt"
	"strconv"
	"strings"
)

func getInput() string {
	return `0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2`
}

type Point struct {
	x int
	y int
}

type Line struct {
	point1 *Point
	point2 *Point
}

func parsePoint(point string) (*Point, error) {
	parts := strings.Split(point, ",")
	x, err := strconv.Atoi(parts[0])

	if err != nil {
		return nil, err
	}

	y, err := strconv.Atoi(parts[1])

	if err != nil {
		return nil, err
	}

	return &Point{x: x, y: y}, nil
}

func parseLine(line string) (*Line, error) {
	parts := strings.Split(line, " -> ")

	p1, err := parsePoint(parts[0])

	if err != nil {
		return nil, err
	}

	p2, err := parsePoint(parts[1])

	if err != nil {
		return nil, err
	}

	return &Line{point1: p1, point2: p2}, nil
}

func isLineHorizontalOrVertical(line *Line) bool {
	return line.point1.x == line.point2.x || line.point1.y == line.point2.y
}

func lineToString(line *Line) string {
	return fmt.Sprintf("(%d, %d) -> (%d, %d)", line.point1.x, line.point1.y, line.point2.x, line.point2.y)
}

func main() {
	str_lines := strings.Split(getInput(), "\n")

	for _, l := range str_lines {
		line, err := parseLine(l)
		if err != nil {
			fmt.Println(fmt.Errorf("Unable to parse line %s: %v", l, err))
		}

		if isLineHorizontalOrVertical(line) {
			fmt.Printf("%s is hori or verti\n", lineToString(line))
		} else {
			fmt.Printf("%s is diagonal\n", lineToString(line))
		}
	}
}
