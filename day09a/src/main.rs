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

    let mut h_curr_pos: (i16, i16) = (0, 0);
    let mut t_curr_pos: (i16, i16) = (0, 0);
    let mut visited_positions: HashSet<(i16, i16)> = HashSet::from([t_curr_pos]);

    for (dir, count) in motions {

        for _ in 0..count {

            println!("Start - H: [{},{}], T: [{},{}]", h_curr_pos.0, h_curr_pos.1, t_curr_pos.0, t_curr_pos.1);

            h_curr_pos = match dir {
                DIR_R => (h_curr_pos.0+1, h_curr_pos.1),
                DIR_L => (h_curr_pos.0-1, h_curr_pos.1),
                DIR_U => (h_curr_pos.0, h_curr_pos.1+1),
                DIR_D => (h_curr_pos.0, h_curr_pos.1-1),
                _ => unreachable!()
            };

            t_curr_pos = match (h_curr_pos, t_curr_pos) {
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

            visited_positions.insert(t_curr_pos);

            println!("End - H: [{},{}], T: [{},{}]", h_curr_pos.0, h_curr_pos.1, t_curr_pos.0, t_curr_pos.1);
            println!();
        }
    }

    println!();
    println!("Visited positions: {}", visited_positions.len());
}


