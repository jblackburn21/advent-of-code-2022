use std::collections::HashSet;

pub fn main() {

    const DIR_R: char = 'R';
    const DIR_L: char = 'L';
    const DIR_U: char = 'U';
    const DIR_D: char = 'D';

    let motions = include_str!("../input.txt")
                            .lines()
                            .map(|l| l.split_once(" ").unwrap())
                            .map(|(dir, count)| {
                                (dir.chars().nth(0).unwrap(), count.parse::<usize>().unwrap())
                            });

    let mut knots_pos: Vec<(i16, i16)> = vec![(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0)];

    let mut visited_positions: HashSet<(i16, i16)> = HashSet::from([knots_pos[9]]);

    for (dir, count) in motions {

        for _ in 0..count {

            println!("Start 0 - H: [{},{}],", knots_pos[0].0, knots_pos[0].1);

            knots_pos[0] = match dir {
                DIR_R => (knots_pos[0].0+1, knots_pos[0].1),
                DIR_L => (knots_pos[0].0-1, knots_pos[0].1),
                DIR_U => (knots_pos[0].0, knots_pos[0].1+1),
                DIR_D => (knots_pos[0].0, knots_pos[0].1-1),
                _ => unreachable!()
            };

            println!("End 0 - H: [{},{}],", knots_pos[0].0, knots_pos[0].1);

            for i in 1..10 {
                let prev_knot_pos = knots_pos[i-1];
                let curr_knot_pos = knots_pos[i];

                println!("Start {i} - Curr: [{},{}]", curr_knot_pos.0, curr_knot_pos.1);

                knots_pos[i] = get_knot_pos(prev_knot_pos, curr_knot_pos);

                println!("End {i} - Curr: [{},{}]", knots_pos[i].0, knots_pos[i].1);
            }

            visited_positions.insert(knots_pos[9]);

            println!();
        }
    }

    println!();
    println!("Visited positions: {}", visited_positions.len());
}

fn get_knot_pos(h_curr_pos: (i16,i16), t_curr_pos: (i16,i16)) -> (i16,i16) {
    return match (h_curr_pos, t_curr_pos) {
        // x same, y up by 2
        (h, t) if h.0 == t.0 && h.1 > t.1 && (h.1 - t.1) > 1 => (t_curr_pos.0, t_curr_pos.1+1),
        // x same, y down by 2
        (h, t) if h.0 == t.0 && h.1 < t.1  && (t.1 - h.1) > 1 => (t_curr_pos.0, t_curr_pos.1-1),
        // x right by 2, y same
        (h, t) if h.0 > t.0 && (h.0 - t.0) > 1 && h.1 == t.1 => (t_curr_pos.0+1, t_curr_pos.1),
        // x left by 2, y same
        (h, t) if h.0 < t.0 && (t.0 - h.0) > 1 && h.1 == t.1 => (t_curr_pos.0-1, t_curr_pos.1),

        // corners

        // x right, y up by 2
        (h, t) if h.0 > t.0 && h.1 > t.1 && (h.1 - t.1) > 1 => (t_curr_pos.0+1, t_curr_pos.1+1),
        // x right by 2, y up
        (h, t) if h.0 > t.0 && (h.0 - t.0) > 1 && h.1 > t.1 => (t_curr_pos.0+1, t_curr_pos.1+1),

        // x left, y up by 2
        (h, t) if h.0 < t.0 && h.1 > t.1 && (h.1 - t.1) > 1 => (t_curr_pos.0-1, t_curr_pos.1+1),
        // x left by 2, y up
        (h, t) if h.0 < t.0 && (t.0 - h.0) > 1 && h.1 > t.1 => (t_curr_pos.0-1, t_curr_pos.1+1),

        // x right, y down by 2
        (h, t) if h.0 > t.0 && h.1 < t.1 && (t.1 - h.1) > 1 => (t_curr_pos.0+1, t_curr_pos.1-1),
        // x right by 2, y down
        (h, t) if h.0 > t.0 && (h.0 - t.0) > 1 && h.1 < t.1 => (t_curr_pos.0+1, t_curr_pos.1-1),

        // x left, y down by 2
        (h, t) if h.0 < t.0 && h.1 < t.1 && (t.1 - h.1) > 1 => (t_curr_pos.0-1, t_curr_pos.1-1),
        // x left by 2, y down
        (h, t) if h.0 < t.0 && (t.0 - h.0) > 1 && h.1 < t.1 => (t_curr_pos.0-1, t_curr_pos.1-1),

        // T stays in current position
        _ => t_curr_pos
    };
}

