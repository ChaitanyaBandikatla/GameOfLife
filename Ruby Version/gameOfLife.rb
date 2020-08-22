def assignLives(board, numRows, numCols, numLiveCells)
  for i in 0..numLiveCells-1
    rowRand = rand(numRows);
    colRand = rand(numCols);
    if(board[rowRand][colRand]==1)
      i=i-1;
    else
    board[rowRand][colRand] = 1;
    end
  end
  board
end

def createBoard(numRows, numCols, numLiveCells)
  board = Array.new(numRows) { Array.new(numCols) };
  for i in 0..numRows-1
    for j in 0..numCols-1
      board[i][j] = 0;
    end
  end
  board = assignLives(board, numRows, numCols, numLiveCells);
  board;
end

def printBoard(board,numRows,numCols)
  for i in 0..numRows-1
    for j in 0..numCols-1
      if board[i][j] == 1
        print"O";
      elsif board[i][j] == 0
        print "X";
      end
    end
    puts();
  end
end

puts "Welcome!\n";
puts "Please enter the number of rows in the board: \n";
numRows = gets.to_i;
puts "Please enter the number of columns in the board: \n";
numCols = gets.to_i;
puts "Please enter the number of live cells in the board: \n";
numLiveCells = gets.to_i;
puts "Please enter the number of iterations: \n";
numIterations = gets.to_i;

puts "Initial Configuration \n";
board = createBoard(numRows, numCols, numLiveCells);
printBoard(board,numRows,numCols);

for i in 0..numIterations-1
  puts("After iteration #{i+1}");
  generateNextStep(board,numRows, numCols);
  printBoard(board,numRows,numCols);
end
puts "Thank you! See you soon!!!";
