const SIZE: usize   = 8;
const EMPTY: &'static str = ".";
const QUEEN: &'static str = "Q";

struct Board {
   size:   usize,
   queens: Vec<Queen>,
}

struct Queen {
   row: usize,
   col: usize,
}

impl Queen {
   fn threatens(&self, row: usize, col: usize) -> bool {
      self.row == row
         || self.col == col
         || self.threatens_diagonally(row, col)
   }

   fn threatens_diagonally(&self, row: usize, col: usize) -> bool {
      let row_delta = (self.row as isize - row as isize).abs();
      let col_delta = (self.col as isize - col as isize).abs();

      row_delta == col_delta
   }
}

impl Board {
   fn new() -> Board {
      Board { size: SIZE, queens: Vec::new() }
   }

   fn position_threatened(&self, row: usize, col: usize) -> bool {
      for queen in &self.queens {
         if queen.threatens(row, col) { return true }
      }
      false
   }

   fn place_queen(&mut self, col: usize) {
      let queen = Queen { row: self.queens.len(), col: col };
      self.queens.push(queen);
   }

   fn remove_last_queen(&mut self) {
      self.queens.pop();
   }

   fn display(&self) {
      for queen in &self.queens {
         let mut row = vec![EMPTY; self.size];
         row[queen.col] = QUEEN;

         println!("{}", row.join(" "));
      }
      println!("");
   }
}

// Recursively finds all solutions, prints them as it finds them
// and returns the number of found solutions
fn solve(board: &mut Board, row: usize, found_solutions: &mut usize) {
   if row == SIZE {
      *found_solutions += 1;
      board.display();
      return
   }

   for col in 0..SIZE {
      if board.position_threatened(row, col) {
         continue;
      }

      board.place_queen(col);
      solve(board, row + 1, found_solutions);

      board.remove_last_queen();
   }
}

fn main() {
   let mut board = Board::new();
   let mut count = 0;

   solve(&mut board, 0, &mut count);
   println!("Found {} solutions", count);
}
