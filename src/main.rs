use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

fn main() {
    // We first get the PathBuf to the current executable
    let mut file_path: PathBuf = std::env::current_exe().expect("Can't retrieve executable path");

    // We then get the executable file name
    let exe_name = file_path
        .file_name()
        .expect("Can't retrieve executable file name");

    // Convert exe file name to a &str
    let exe_name = exe_name
        .to_str()
        .expect("Can't convert executable file name to UTF-8");

    // We get the file name for the points of the graph
    let args: Vec<String> = env::args().collect();

    // If user didn't specify a file name
    if args.len() < 2 {
        panic!("Usage : ./{exe_name} exemple.pts");
    }

    // Get file name
    let file_name: &String = &args[1];

    // We add the correct file name to the path
    file_path.pop();
    file_path.push(file_name);
    let file_path: String = file_path.as_path().display().to_string();

    let point_vector: Vec<(f64, f64)>;
    let radius: f64;

    //dbg!("Full path of file is {}", &file_path);

    (radius, point_vector) = read_file(&file_path);

    //dbg!(format!("r={}", radius));
    //for (x, y) in point_vector.iter() {
    //dbg!(format!("x={}, y={}", x, y));
    //}

    // In our algorithm, the size of a cell is sqrt(radius)/2, so that two adjacent cells have at most a distance of radius
    let cell_size: f64 = radius / (2 as f64).sqrt();
    let matrix_size: usize = (1. / cell_size).ceil() as usize;

    // We initialize our matrices
    let mut point_matrix: Vec<Vec<Vec<(f64, f64)>>> =
        vec![vec![Vec::new(); matrix_size]; matrix_size];
    let mut color_matrix: Vec<Vec<u8>> = vec![vec![0; matrix_size]; matrix_size];

    // We initialize the length vector which holds the length of each composantes connexes
    let mut size_vector: Vec<u16> = vec![0];

    // We initialize the first color
    let mut color_counter: u8 = 1;

    // We do a first pass to fill the point matrix
    for (x, y) in &point_vector {
        let i: usize = (x / cell_size).floor() as usize;
        let j: usize = (y / cell_size).floor() as usize;
        point_matrix[j][i].push((*x, *y))
    }

    // We do a first pass to eliminate all empty cells
    for i in 0..matrix_size {
        for j in 0..matrix_size {
            if !point_in_cell(&i, &j, &point_matrix) {
                color_matrix[j][i] = 0b1111_1111
            }
        }
    }
    //println!("Initial matrix");
    //print_matrix(&color_matrix);

    // Next we color each non-empty cell
    for j in 0..matrix_size {
        for i in 0..matrix_size {
            if color_matrix[j][i] == 0b0000_0000 {
                add_color_to_matrix_cell(
                    &i,
                    &j,
                    &mut color_matrix,
                    &point_matrix,
                    &mut size_vector,
                    &color_counter,
                    &radius,
                );

                //println!("{color_counter}");
                //print_matrix(&color_matrix);

                // We increment the color counter because we filled the composante connexe
                color_counter = color_counter + 1;
                size_vector.push(0)
            }
        }
    }

    size_vector.pop().expect("Size vector should not be empty");
    size_vector.sort();
    size_vector.reverse();

    print_matrix(&color_matrix, &point_matrix);
    println!("{:?}", size_vector);
}

fn print_matrix(color_matrix: &Vec<Vec<u8>>, point_matrix: &Vec<Vec<Vec<(f64, f64)>>>) {
    // We print the color matrix
    let matrix_size = color_matrix.len();
    for i in 0..matrix_size {
        for j in 0..matrix_size {
            print!(
                "{:width$} ({:width$}) - ",
                color_matrix[i][j].to_string(),
                point_matrix[i][j].len().to_string(),
                width = 3
            )
        }
        print!("\n")
    }
    print!("\n")
}

fn add_color_to_matrix_cell(
    i: &usize,
    j: &usize,
    color_matrix: &mut Vec<Vec<u8>>,
    point_matrix: &Vec<Vec<Vec<(f64, f64)>>>,
    size_vector: &mut Vec<u16>,
    color: &u8,
    radius: &f64,
) -> () {
    // We color the right cell of the color matrix
    //dbg!("Colored cell i={} j={}", *i, *j);
    color_matrix[*j][*i] = *color;

    // We add the correct amount of elements to the size vector
    size_vector[(color - 1) as usize] =
        size_vector[(color - 1) as usize] + point_matrix[*j][*i].len() as u16;

    let mut offsets: Vec<(i8, i8)> = vec![];
    for x in -2..=2 {
        for y in -2..=2 {
            let distance = (x as i8).abs() + (y as i8).abs();
            if distance != 4 && distance != 0 {
                offsets.push((x as i8, y as i8));
            }
        }
    }

    /* let offsets: [(i8, i8); 20] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 1),
        (1, 1),S
        (0, 1),
        (-1, 0),
        (1, 0),
        (-2, -1),
        (-2, 0),
        (-2, 1),
        (2, -1),
        (-2, 0),
        (-2, 1),
        (-1, -2),
        (0, -2),
        (1, -2),
        (-1, 2),
        (0, 2),
        (1, 2),
    ]; */

    for (x_offset, y_offset) in offsets {
        let adjacent_i = (*i as i8) + x_offset;
        let adjacent_j = (*j as i8) + y_offset;

        // If 0 <= i+offset < len of matrix
        if adjacent_i < 0 || adjacent_i >= color_matrix.len() as i8 {
            //dbg!("ajdacent_i outside of accepted range");
            continue;
        }
        // If 0 <= j+offset < len of matrix
        if adjacent_j < 0 || adjacent_j >= color_matrix.len() as i8 {
            //dbg!("ajdacent_j outside of accepted range");
            continue;
        }

        // If the ajdacent cell was already colored
        if color_matrix[adjacent_j as usize][adjacent_i as usize] != 0b0000_0000 {
            //dbg!("Cell2 was already colored");
            continue;
        }

        // If the two cells are connexe
        if !two_cells_are_connexe(
            &i,
            &j,
            &(adjacent_i as usize),
            &(adjacent_j as usize),
            &point_matrix,
            &radius,
        ) {
            //dbg!("Cells were not connexe");
            continue;
        }

        //dbg!("Cells are connexe!");
        add_color_to_matrix_cell(
            &(adjacent_i as usize),
            &(adjacent_j as usize),
            color_matrix,
            &point_matrix,
            size_vector,
            &color,
            &radius,
        );
    }
}

fn two_cells_are_connexe(
    i1: &usize,
    j1: &usize,
    i2: &usize,
    j2: &usize,
    point_matrix: &Vec<Vec<Vec<(f64, f64)>>>,
    radius: &f64,
) -> bool {
    let radius_squared = (*radius).powi(2);
    let cell1 = &point_matrix[*j1][*i1];
    let cell2 = &point_matrix[*j2][*i2];

    for (x_1, y_1) in cell1 {
        for (x_2, y_2) in cell2 {
            if (*x_2 - *x_1).powi(2) + (*y_2 - *y_1).powi(2) <= radius_squared {
                return true;
            }
        }
    }
    false
}

fn point_in_cell(i: &usize, j: &usize, point_matrix: &Vec<Vec<Vec<(f64, f64)>>>) -> bool {
    return point_matrix[*j][*i].len() > 0;
}

fn read_file(file_path: &String) -> (f64, Vec<(f64, f64)>) {
    let file = File::open(file_path).expect(
        format!("Specified file was not found, {file_path} should be a readable file").as_str(),
    );

    let mut buf_reader = BufReader::new(file);

    // We read the first line
    let mut radius: String = String::new();
    buf_reader
        .read_line(&mut radius)
        .expect("File should not be empty");

    // We remove the new line at the end of radius
    trim_newline(&mut radius);

    let radius: f64 = radius.parse::<f64>().expect("Radius should be a float");

    let mut point_vector: Vec<(f64, f64)> = vec![];

    // For each line, we extract the x and y values of the point
    for line in buf_reader.lines() {
        let point_line: String = line.expect("Can't read line");
        let point_line: Vec<&str> = point_line.split(",").collect();

        let x: f64 = point_line[0]
            .trim()
            .parse::<f64>()
            .expect("Value in file should be floats");
        let y: f64 = point_line[1]
            .trim()
            .parse::<f64>()
            .expect("Value in file should be floats");

        point_vector.push((x, y));
    }

    (radius, point_vector)
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
