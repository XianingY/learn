package main

import "fmt"

func main() {
	scores := map[string]int{
		"Ada":   98,
		"Ken":   91,
		"Linus": 88,
	}

	scores["Ada"] = 99

	total := 0
	for name, score := range scores {
		fmt.Printf("%s: %d\n", name, score)
		total += score
	}

	count := len(scores)
	avg := float64(total) / float64(count)
	fmt.Printf("students: %d\n", count)
	fmt.Printf("average: %.1f\n", avg)

	if v, ok := scores["Ken"]; ok {
		fmt.Println("Ken's score:", v)
	}
}
