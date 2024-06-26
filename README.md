# Point Clustering Algorithm

This Rust program implements an algorithm to cluster points based on a given radius. It reads a file containing points, clusters them into connected components based on a given radius, and outputs the clusters with assigned colors.

![Example image of the program](images/example.png)

## Pre-requisites

1. Ensure you have Rust installed on your machine.
2. Clone this repository to your local machine.
3. Navigate to the project directory.

### Running the Program

Run the following command to compile and execute the program:

```bash
cargo run --release <file_name>
```

Replace `<file_name>` with the name of the file containing points in the format specified below.

## Input File Format

The input file should have the following format:

```
<radius>
x1, y1
x2, y2
...
```

-   `<radius>`: Radius value (float) used for clustering. Must be between 0 and 1.
-   `xi, yi`: Coordinates of points (floats) separated by a comma. Must be between 0 and 1.

Example input file (`points.pts`):

```
0.2
0.29, 0.14
0.81, 0.28
0.02, 0.09
0.9, 0.76
0.54, 0.49
```

## Output

The program outputs the clustering matrix where each cell represents a cluster, and each number represents a cluster's color. It also outputs the size vector containing the sizes of each connected component.

## Algorithm Overview

1. Read the input file to get the radius and points.
2. Splice the area in a grid of a specific cell size (based on the radius), where two points in the same cell must be the same color.
3. Cluster points into connected components by grouping the points in the cells of the grid.
4. For each adjacent cell of a cell, check if two points are connected. If so, change the color of the second cell to the color of the first one.
5. Output the resulting color matrix and size of the connected components.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
