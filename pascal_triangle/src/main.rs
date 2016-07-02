fn main() {
    let limit = 15;
    let triangle = pascal(limit);

    print(triangle);
}

type Triangle = Vec<Vec<usize>>;

fn pascal(limit: usize) -> Triangle {
    let mut triangle = vec![];
    let mut last_row = vec![1];

    for _ in 0..limit {
        let mut row = vec![1];

        for pair in last_row.windows(2) {
            row.push(pair[0] + pair[1]);
        }

        row.push(1);

        triangle.push(last_row);
        last_row = row;
    }

    return triangle;
}

fn print(triangle: Triangle) {
    for row in triangle {
        for item in row {
            print!("{:^6}", item);
        }
        println!("");
    }
}
