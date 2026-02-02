package main

import "fmt"

func main() {
	x := 5
	y := &x

	fmt.Println("x =", x)
	fmt.Println("*y =", *y)
	fmt.Println("x == *y:", x == *y)
}
