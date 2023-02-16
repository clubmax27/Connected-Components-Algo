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
        println!("Usage : ./{exe_name} exemple.pts");
        return;
    }

    // Get file name
    let file_name: &String = &args[1];

    // We add the correct file name to the path
    file_path.pop();
    file_path.push(file_name);
    let file_path: String = file_path.as_path().display().to_string();

    let point_vector: Vec<(f64, f64)>;
    let radius: f64;

    println!("Full path of file is {}", file_path);

    (radius, point_vector) = read_file(&file_path);

    println!("r={radius}");
    for (x, y) in point_vector.iter() {
        println!("x={x}, y={y}");
    }
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
