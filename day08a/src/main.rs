
pub fn main() {

    let lines = include_str!("../input.txt").lines();

    let rows: Vec<Vec<u32>> = lines
                                .map(|l| {
                                    l.chars()
                                        .map(|c| c.to_digit(10).unwrap())
                                        .collect()
                                })
                                .collect();

    let row_count = rows.len();
    let col_count = rows.get(0).unwrap().len();

    let mut tallest_trees = ((row_count-1) * 2) + ((col_count-1) * 2);

    println!("Row count: {row_count}, Col count: {col_count}, Outer tallest trees: {tallest_trees}");
    println!();

    for x in 1..row_count-1 {

        for y in 1..col_count-1 {

            let tree_size = rows[x][y];

            let taller_left = rows[x][..y].into_iter().all(|s| s < &tree_size);
            let taller_right = rows[x][y+1..].into_iter().all(|s| s < &tree_size);

            let col: Vec<u32> = rows.iter()
                                .map(|r| r[y])
                                .collect();

            let taller_top = col[..x].into_iter().all(|s| s < &tree_size);
            let taller_bottom = col[x+1..].into_iter().all(|s| s < &tree_size);

            let tallest = taller_left || taller_right || taller_top || taller_bottom;

            if tallest {
                tallest_trees += 1;
            }

            // println!("Tree size: {tree_size}, Tallest: {tallest}, Left: {taller_left}, Right: {taller_right}, Top: {taller_top}, Bottom: {taller_bottom}");
        }

        // println!();
    }

    println!("Tallest trees: {tallest_trees}");
}


