package main

import "fmt"

func main() {
	// init
	arr := make([]int, 100)
	var tempArr []int
	temp := 0
	for i := 100; i > 0; i-- {
		arr[temp] = i
		temp = temp + 1
	}

	// filter prime
	for i, num := range arr {
		temp = num / 2
		for j := 2; j <= temp; j++ {
			if num%j == 0 {
				// not prime number
				tempArr = append(tempArr, arr[i])
				break
			}
		}
	}

	// output
	for _, num := range tempArr {
		if num%5 == 0 && num%3 == 0 {
			fmt.Print("FooBar ")
		} else if num%5 == 0 {
			fmt.Print("Bar ")
		} else if num%3 == 0 {
			fmt.Print("Foo ")
		} else {
			fmt.Printf("%d ", num)
		}
	}

	fmt.Println()
}
