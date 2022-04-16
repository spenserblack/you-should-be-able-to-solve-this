// Solves x - 7 = 19 + x
package main

import (
	"fmt"
	"math"
)

// X is the value that matches x - 7 = 19 + x
var x = math.Inf(1)

func main() {
	fmt.Printf("x - 7 = 19 + x: %v\n", x - 7 == 19 + x)
}
