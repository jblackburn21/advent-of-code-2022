
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

    let mut max_scenic_score = 0;

    println!("Row count: {row_count}, Col count: {col_count}");
    println!();

    for x in 0..row_count {

        for y in 0..col_count {

            let tree_size = rows[x][y];

            let visible_left = rows[x][..y].iter().rev()
                                                .take_while(|&s| *s < tree_size)
                                                .count() + 1;


            let visible_right = rows[x][y+1..].iter()
                                            .take_while(|&s| *s < tree_size)
                                            .count();

            let col: Vec<u32> = rows.iter()
                                .map(|r| r[y])
                                .collect();

            let visible_top = col[..x].iter().rev()
                                            .take_while(|&s| *s < tree_size)
                                            .count();

            let visible_bottom = col[x+1..].iter()
                                            .take_while(|&s| *s < tree_size)
                                            .count();

            let scenic_score = visible_left * visible_right * visible_top * visible_bottom;

            if max_scenic_score < scenic_score {
                max_scenic_score = scenic_score;
            }

            // println!("Tree: [{},{}], Left: {visible_left}, Right: {visible_right}, Top: {visible_top}, Bottom: {visible_bottom}, Scenic score: {scenic_score}", x+1, y+1);
        }

        // println!();
    }

    println!("Max scenic score: {max_scenic_score}");
}

