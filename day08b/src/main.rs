
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

            let mut visible_left = 0;

            for s in rows[x][..y].into_iter().rev() {
                visible_left += 1;

                if s >= &tree_size {
                    break;
                }
            }

            let mut visible_right = 0;

            for s in rows[x][y+1..].into_iter() {
                visible_right += 1;

                if s >= &tree_size {
                    break;
                }
            }

            let col: Vec<u32> = rows.iter()
                                .map(|r| r[y])
                                .collect();

            let mut visible_top = 0;

            for s in col[..x].into_iter().rev() {

                visible_top += 1;

                if s >= &tree_size {
                    break;
                }
            }

            let mut visible_bottom = 0;

            for s in col[x+1..].into_iter() {
                visible_bottom += 1;

                if s >= &tree_size {
                    break;
                }
            }

            // let visible_left = pos_left.map_or(0, |p| p+1);
            let scenic_score = visible_left * visible_right * visible_top * visible_bottom;

            if max_scenic_score < scenic_score {
                max_scenic_score = scenic_score;
            }

            println!("Tree: [{},{}], Left: {visible_left}, Right: {visible_right}, Top: {visible_top}, Bottom: {visible_bottom}, Scenic score: {scenic_score}", x+1, y+1);
        }

        println!();
    }

    println!("Max scenic score: {max_scenic_score}");
}


