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

def liveNeighbours(board,i,j,numRows,numCols)
  count = 0
  directions = Array.new(8) { Array.new(2) }
  directions[0] = [0, 1];
  directions[1] = [1,0];
  directions[2] = [1,1];
  directions[3] = [1,-1];
  directions[4] = [0,-1];
  directions[5] = [-1,0];
  directions[6] = [-1,1];
  directions[7] = [-1,1];
  directions.each do |direction|
    x, y = [i + direction[0], j + direction[1]];
    if x.between?(0, numRows - 1) && y.between?(0, numCols - 1)
      if board[x][y] == 1 || board[x][y] == 3
        count += 1
      end
    end
  end
  count;
end

def generateNextStep(board,numRows, numCols)
  for i in 0..numRows
    for j in 0..numCols
      countLiveCells = liveNeighbours(board,i,j,numRows,numCols);
      if board[i][j] == 0
        if countLiveCells == 3
          board[i][j] = 2
        end
      else
        if countLiveCells < 2 or countLiveCells > 3
          board[i][j] = 3
        end
      end
    end
  end

    for i in 0..numRows-1
      for j in 0..numCols-1
        if board[i][j] == 2
          board[i][j] = 1
        end
       if board[i][j] == 3
         board[i][j] = 0
       end
      end
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
