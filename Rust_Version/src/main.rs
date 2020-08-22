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
            return " ".to_string();
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

fn displayBoard(board: Board, numRows: usize, numCols: usize) {
    println!("-------------------------");
    for r in 0..numRows {
        for c in 0..numCols {
            print!("{}", board.content[r][c].to_string());
        }
        println!();
    }
    println!("-------------------------Generation done");
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
    displayBoard(board, numRows, numCols);

    for i in 0..numIterations {
        println!("Iteration {} done", i + 1);
        //board = nextGenerationBoard();
        //displayBoard(board, numRows, numCols);
    }

    println!("Thank you!");
}