# GameOfLife in Go

Set-up and execution
 - 
This folder contains working code for Conway's game of life in Go language.

To build and run the game of life on your machine, follow the below steps:

 - Firstly, you need to install binaries from [here](https://golang.org/dl/)
 - Post that, you can run the code using `go run gameoflife.go`
 - You should be able to enter your desired inputs for height, width, number of live cells to start with, and number of steps/iterations
 - The code would print the board after every iteration!

Alternatively, if you want to make life simpler like most of us, please use the online playground [here](https://repl.it/languages/go)

Few resources to debug the Go code:
 - 
 
A commonly used tool to debug is Delve. You can find more about it [here](https://github.com/go-delve/delve). 

But since, it is a bit too much to be done in 30 minutes, we suggest you to use `fmt.Print()` statements to debug. You can find more information [here](https://golang.org/pkg/fmt/).
 - Example: `fmt.Printf("%d\n", i)` to print value of integer `i`
