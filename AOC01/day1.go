package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	file, _ := os.Open("input")
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var count int
	var max [3]int
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			if i, min := minimum(max); min < count {
				max[i] = count
			}
			count = 0
			continue
		}
		num, _ := strconv.Atoi(line)
		count += num
	}
	fmt.Println(max[0] + max[1] + max[2])
}

func minimum(nums [3]int) (int, int) {
	min := nums[0]
	var mini int
	for i, num := range nums {
		if min > num {
			mini = i
			min = num
		}
	}
	return mini, min
}
