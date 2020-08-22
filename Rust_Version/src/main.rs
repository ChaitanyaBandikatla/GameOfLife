#![allow(non_snake_case)]

use std::io;
use std::io::prelude::*;
use rand::Rng;
use std::collections::HashSet;

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
                let mut _i = 0;
                let mut generatedCells = HashSet::new();
                while _i < numLiveCells {
                    let rowGenerated = rng.gen_range(0, numRows);
                    let colGenerated = rng.gen_range(0, numCols);
                    if !generatedCells.contains(&(rowGenerated, colGenerated)) {
                        totalVector[rowGenerated][colGenerated].currentState = true;
                        generatedCells.insert((rowGenerated, colGenerated));
                        _i = _i + 1;
                    }
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

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn displayBoard(board: &Board) {
    println!("-------------------------");
    for r in 0..board.rows {
        for c in 0..board.cols {
            print!("{} ", board.content[r][c].to_string());
        }
        println!();
    }
    println!("-------------------------");
    pause();
}

fn countNeighbourPopulation(board: &Board, row: usize, col: usize) -> i32 {
    let mut neighborPopulation = 0;
    let mut r = if row > 0 {row - 1} else {0};
    while r <= row + 1 {        
        let mut c = if col > 0 {col - 1} else {0};
        while c <= col + 1 {
            if (r < board.rows && c < board.cols) && (r != row || c != col) {
                neighborPopulation += if board.content[r][c].currentState {1} else {0};
            }
            c = c + 1;
        }
        r = r + 1;
    }
    return neighborPopulation;
}

fn assignNextState(board: &mut Board, row: usize, col: usize) {
    let neighborPopulation = countNeighbourPopulation(board, row, col);
    
    if board.content[row][col].currentState {
        if neighborPopulation < 2 || neighborPopulation > 3 {
            board.content[row][col].nextState = false;
        } else if neighborPopulation == 2 || neighborPopulation == 3 {
            board.content[row][col].nextState = true;
        }
    } else {
        if neighborPopulation == 3 {
            board.content[row][col].nextState = true;
        }
    }
}

fn nextGenerationBoard(board: &mut Board) {
    let numRows = board.rows;
    let numCols = board.cols;

    for row in 0..numRows {
        for col in 0..numCols {
            assignNextState(board, row, col);
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
    displayBoard(&board);

    for i in 0..numIterations {
        println!("\nIteration {} done", i + 1);
        nextGenerationBoard(&mut board);
        displayBoard(&board);
    }

    println!("Thank you!");
}