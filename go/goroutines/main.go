package main

import (
	"fmt"
	"sync"
	"time"
)

func main() {
	var wg sync.WaitGroup
	ch := make(chan string)

	wg.Add(1)
	go func() {
		defer wg.Done()
		for i := 1; i <= 3; i++ {
			ch <- fmt.Sprintf("ping %d", i)
			time.Sleep(100 * time.Millisecond)
		}
		close(ch)
	}()

	for msg := range ch {
		fmt.Println(msg)
	}

	wg.Wait()
}
