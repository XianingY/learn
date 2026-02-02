package main

import "fmt"

func main() {
	a := complex(2.1, -1.2)
	b := complex(11.1, 22.2)
	result := a + b

	fmt.Printf("%.1f + %.1fi\n", real(result), imag(result))
}
