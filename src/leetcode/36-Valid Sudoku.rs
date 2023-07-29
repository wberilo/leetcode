pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
  let mut quadrants: Vec<Vec<HashSet<char>>> = vec![vec![HashSet::new();3];3];
  let mut lines: Vec<HashSet<char>> = vec![HashSet::new(); 9];
  let mut columns: Vec<HashSet<char>> = vec![HashSet::new(); 9];


  for (i,line) in board.iter().enumerate() {
      for (j, element) in line.iter().enumerate() {
          if !element.is_numeric() {
              continue;
          }

          if quadrants[i/3][j/3].contains(element){ 
              return false;
          }
          quadrants[i/3][j/3].insert(*element);
          if lines[i].contains(element) {
              return false;
          }
          lines[i].insert(*element);
          if columns[j].contains(element) {
              return false;
          }
          columns[j].insert(*element);
      }
      
  }

  true
}
