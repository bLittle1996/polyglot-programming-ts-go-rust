package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"
)

type Coordinate struct {
    x int
    y int
}

func parseLine(line string) Coordinate {
    parts := strings.Split(line, " ")
    direction := parts[0]
    amount, error := strconv.Atoi(parts[1])

    if error != nil {
        log.Println(fmt.Errorf("Unable to parse amount to int: %s", line))
        amount = 0
    }

    switch direction {
    case "up": return Coordinate { x: 0, y: amount * -1 }
    case "down": return Coordinate { x: 0, y: amount }
    case "forward": return Coordinate{  x: amount, y: 0 }
    case "backward": return Coordinate{ x: amount * -1, y: 0 }
    default: return Coordinate{ x: 0, y: 0 } 
    }
}

func main() {
	lines := strings.Split(getInput(), "\n")
    finalCoordinate := Coordinate {}

    for _, line := range lines {
        coordinate := parseLine(line) 
        finalCoordinate.x += coordinate.x
        finalCoordinate.y += coordinate.y
    }

    answer := finalCoordinate.x * finalCoordinate.y

    log.Printf("Coordinate: %d, %d\n", finalCoordinate.x, finalCoordinate.y)
    log.Printf("Answer: %d", answer)
}

func getInput() string {
	return `forward 5
down 5
forward 8
up 3
down 8
forward 2`
}
