package main

import (
	"fmt"
	"time"
)

func main() {
	ticker := time.NewTicker(time.Second)

	for {
		select {
		case t := <-ticker.C:
			fmt.Println("current time:", t.Format("2006-01-02 15:04:05"))
		}
	}
}

