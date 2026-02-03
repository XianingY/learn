package main

import "fmt"

func main() {
	nums := []int{1, 2, 3, 4, 5}
	fmt.Printf("len=%d, cap=%d\n", len(nums), cap(nums))

	nums = append(nums, 6)
	fmt.Printf("after append: len=%d, cap=%d\n", len(nums), cap(nums))

	sub := nums[1:4]
	fmt.Printf("sub-slice: %v (len=%d)\n", sub, len(sub))

	for i, v := range nums {
		fmt.Printf("%d: %d\n", i, v)
	}
}
