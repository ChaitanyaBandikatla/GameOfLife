package main

import (
	"fmt"
	"math/rand"
)

type Cell struct {
	currentState bool
	nextState    bool
}

func createCell(state bool) Cell {
	c := Cell{currentState: state, nextState: false}
	return c
}

type Board struct {
	boardWidth    int
	boardHeight   int
	boardContents [][]Cell
}

func createBoard(width int, height int, countOfLiveCells int) Board {
	b := make([][]Cell, height)
	for i := range b {
		b[i] = make([]Cell, width)
	}

	for i := 0; i < height; i++ {
		for j := 0; j < width; j++ {
			b[i][j] = createCell(false)
		}
	}

	board := Board{boardWidth: width, boardHeight: height, boardContents: b}
	assignLives(board, countOfLiveCells)
	return board
}

func assignLives(board Board, countOfLiveCells int) {
	height := len(board.boardContents)
	width := len(board.boardContents[0])
	for i := 0; i < countOfLiveCells; i++ {
		row := rand.Intn(height)
		col := rand.Intn(width)
		board.boardContents[row][col].currentState = true
	}
}

func generateNextStep(board Board) {
	height := len(board.boardContents)
	width := len(board.boardContents[0])
	for i := 0; i < height; i++ {
		for j := 0; j < width; j++ {
			assignNextState(board, i, j)
		}
	}

	copyNextStateToCurrentState(board)
}

func assignNextState(board Board, row int, column int) {
	live := 0
	height := len(board.boardContents)
	width := len(board.boardContents[0])
	for i := -1; i <= 1; i++ {
		newRow := row + i
		if newRow > -1 && newRow < height {
			for j := -1; j <= 1; j++ {
				newColumn := column + j
				if newColumn > -1 && newColumn < width {
					if !(j == 0 && i == 0) && board.boardContents[newRow][newColumn].currentState {
						live++
					}
				}
			}
		}
	}
	if board.boardContents[row][column].currentState {
		if live < 2 || live > 3 {
			board.boardContents[row][column].nextState = false
		}
	} else if !board.boardContents[row][column].currentState {
		if live == 3 {
			board.boardContents[row][column].nextState = true
		}
	}
}

func copyNextStateToCurrentState(board Board) {
	height := len(board.boardContents)
	width := len(board.boardContents[0])
	for i := 0; i < height; i++ {
		for j := 0; j < width; j++ {
			board.boardContents[i][j].currentState = board.boardContents[i][j].nextState
		}
	}
}

func printBoard(board Board) {
	fmt.Println("------------------------------------")
	height := len(board.boardContents)
	width := len(board.boardContents[0])
	for i := 0; i < height; i++ {
		for j := 0; j < width; j++ {
			if board.boardContents[i][j].currentState {
				fmt.Print("o")
			} else {
				fmt.Print(" ")
			}
		}
		fmt.Println("")
	}

	fmt.Println("------------------------------------")
}

func main() {
	fmt.Println("Welcome!\n")

	fmt.Print("Please enter the number of rows in the board: ")
	var numRows int
	fmt.Scanf("%d", &numRows)
	fmt.Print("\nPlease enter the number of columns in the board: ")
	var numCols int
	fmt.Scanf("%d", &numCols)
	fmt.Print("\nPlease enter the number of live cells to start with: ")
	var numLiveCells int
	fmt.Scanf("%d", &numLiveCells)
	fmt.Print("\nPlease enter the number of iterations: ")
	var numiterations int
	fmt.Scanf("%d", &numiterations)

	fmt.Println("\nInitial Configuration")
	board := createBoard(numCols, numRows, numLiveCells)
	printBoard(board)

	for i := 0; i < numiterations; i++ {
		fmt.Printf("After iteration %d \n", i+1)
		generateNextStep(board)
		printBoard(board)
	}

	fmt.Println("Thank you! See you soon!!!")
}
