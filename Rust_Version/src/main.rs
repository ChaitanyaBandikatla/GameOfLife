#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::io;
use rand::Rng;

struct Cell {
    currentState: bool,
    nextState: bool,
}

struct Board {
    rows: usize,
    cols: usize,
    content: Vec<Vec<Cell>>,
}

impl Cell {
    pub fn new () -> Cell {
        Cell{
            currentState: false,
            nextState: false,
        }
    }

    
    fn to_string(&self) -> String {
        if self.currentState {
            return "0".to_string();
        } else {
            return "_".to_string();
        }
    }
}

impl Board {
    pub fn new(numRows: usize, numCols: usize, numLiveCells: u32) -> Board {
        Board{
            rows: numRows,
            cols: numCols,
            content: {
                let mut totalVector = Vec::new();
                for _r in 0..numRows {
                    let mut rowVector = Vec::new();
                    for _c in 0..numCols {
                        rowVector.push(Cell::new());
                    }
                    totalVector.push(rowVector);
                }
                
                let mut rng = rand::thread_rng();
                for r in 0..numLiveCells {
                    let rowGenerated = rng.gen_range(0, numRows);
                    let colGenerated = rng.gen_range(0, numCols);

                    totalVector[rowGenerated][colGenerated].currentState = true; 
                }
                totalVector
            },
        }
    }
}

fn getInput() -> u32 {
    let mut inputText = String::new();
    io::stdin().read_line(&mut inputText).expect("failed to read from stdin");
    let trimmed = inputText.trim().parse::<u32>().unwrap();
    return trimmed;
}

fn displayBoard(board: &Board, numRows: usize, numCols: usize) {
    println!("-------------------------");
    for r in 0..numRows {
        for c in 0..numCols {
            print!("{}", board.content[r][c].to_string());
        }
        println!();
    }
    println!("-------------------------Generation done");
}

fn countNeighbourPopulation(board: &Board, row: usize, col: usize, numRows: usize, numCols: usize) -> i32 {
    let mut neighborPopulation = 0;
    let mut r = if row > 0 {row - 1} else {0};
    let mut c = if col > 0 {col - 1} else {0};
    while r <= row + 1 {
        while c <= col + 1 {
            if (r < numRows && c < numCols) && (r != row || c != col) {
                neighborPopulation += if board.content[r][c].currentState {1} else {0};
            }
            c = c + 1;
        }
        r = r + 1;
    }

    return neighborPopulation;
}

fn assignNextState(board: &mut Board, row: usize, col: usize, numRows: usize, numCols: usize) {
    let neighborPopulation = countNeighbourPopulation(board, row, col, numRows, numCols);
    
    if board.content[row][col].currentState {
        if neighborPopulation < 2 || neighborPopulation > 3 {
            board.content[row][col].nextState = false;
        }
    } else {
        if neighborPopulation == 3 {
            board.content[row][col].nextState = true;
        }
    }
}

fn nextGenerationBoard(board: &mut Board, numRows: usize, numCols: usize) {
    for row in 0..numRows {
        for col in 0..numCols {
            assignNextState(board, row, col, numRows, numCols);
        }
    }

    for row in 0..numRows {
        for col in 0..numCols {
            board.content[row][col].currentState = board.content[row][col].nextState;
        }
    }
}

fn main() {
    println!("Welcome to Rust implementaion of Game of Life!");

    let numRows: usize;
    let numCols: usize;
    let numLiveCells: u32;
    let numIterations: u32;

    println!("Please enter the number of rows: ");
    numRows = getInput() as usize;
    println!("Please enter the number of columns: ");
    numCols = getInput() as usize;
    println!("Please enter the number of live cells: ");
    numLiveCells = getInput();
    println!("Please enter the number of iterations: ");
    numIterations = getInput();

    let mut board = Board::new(numRows, numCols, numLiveCells);
    displayBoard(&board, numRows, numCols);

    for i in 0..numIterations {
        println!("Iteration {} done", i + 1);
        nextGenerationBoard(&mut board, numRows, numCols);
        displayBoard(&board, numRows, numCols);
    }

    println!("Thank you!");
}