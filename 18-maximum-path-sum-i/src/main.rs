use std::cmp::max;

fn main() {
    dfs();
}

fn dfs() {
    let path:Vec<Vec<u32>> = vec![
                                    vec![75],
                                  vec![95, 64],
                                vec![17, 47, 82],
                              vec![18, 35, 87, 10],
                            vec![20, 04, 82, 47, 65],
                          vec![19, 01, 23, 75, 03, 34],
                        vec![88, 02, 77, 73, 07, 63, 67],
                      vec![99, 65, 04, 28, 06, 16, 70, 92],
                    vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
                  vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
                vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
              vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
            vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
          vec![63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
        vec![04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23],
    ];

    // TODO understand DFS algorithms, and implementation
    // TODO figure out where recursion can be used
    // since the 1st term of the tree does not require a choice
    // we start exploring from bottom to top
    // we can ignore the 1st array, since it only contains 1 value. we can compute that at the end
    // size is 15x15, since we start at last vec, and skip 1st,

    // this loop iterates 14 times
    // we start at path[14][0] and look at path[13][0] and path[13][1]
    let iter = (1..(path.len())).rev();
    let mut sum = 0;
    for i in iter {
        // iter starts at the longest vec
        for j in 0..i {
            // this iterates through the vector.
            sum += max(path[i][j], path[i][j+1]);
            print!("{} ", path[i][j]);
        }
        println!("\n {}", sum);
        //
    }
}
