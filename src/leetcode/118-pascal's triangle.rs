pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut triangle: Vec<Vec<i32>> = Vec::new();
    for i in 0..num_rows {
        let mut row: Vec<i32> = Vec::new();

        for j in 0..=i {
            if j == 0 || j == i {
                row.push(1);
            }
            else if i > 1 {
                let add1 = triangle[(i - 1) as usize][(j-1) as usize];
                let add2 = triangle[(i - 1) as usize][j as usize];
                row.push(add1+add2);
            }
        }
        triangle.push(row);
    }

    return triangle;
}