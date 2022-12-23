package main

import (
	"fmt"
	"os"
	"strings"
	"unicode"
	"strconv"
)

func main() {

	input, err := os.ReadFile("../5.txt")
	if err != nil {
		fmt.Printf("Could not read the file due to this %s error \n", err)
	}
	line := string(input)
	
}