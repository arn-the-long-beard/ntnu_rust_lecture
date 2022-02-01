package main

import (
	"flag"
	"fmt"
	"time"
)

func measure(start time.Time, name string) {
	elapsed := time.Since(start)
	fmt.Printf("%s took %s", name, elapsed)
	fmt.Println()
}

var maxCount = flag.Int("n", 100000, "how many")

func f(output, input chan int) {
	output <- 1 + <-input
}

func test() {
	fmt.Printf("Started, sending %d messages.", *maxCount)
	fmt.Println()
	flag.Parse()
	defer measure(time.Now(), fmt.Sprintf("Sending %d messages", *maxCount))
	finalOutput := make(chan int)
	var left, right chan int = nil, finalOutput
	for i := 0; i < *maxCount; i++ {
		left, right = right, make(chan int)
		go f(left, right)
	}
	right <- 0
	x := <-finalOutput
	fmt.Println(x)
}

func main() {
	test()
	test()
}
