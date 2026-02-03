package main

import "fmt"

type DivideError struct {
	Divident int
}

func (e DivideError) Error() string {
	return fmt.Sprintf("cannot divide %d by zero", e.Divident)
}

func divide(a, b int) (int, error) {
	if b == 0 {
		return 0, DivideError{Divident: a}
	}
	return a / b, nil
}

func main() {
	result, err := divide(10, 0)
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	fmt.Println("Result:", result)
}
