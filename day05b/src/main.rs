pub fn main() {

    let (stacks_raw, moves_raw) = include_str!("../input.txt")
                                        .split_once("\n\n").unwrap();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    let stacks_init = stacks_raw.lines().rev().skip(1);

    for si in stacks_init {

        let chars = si.chars().skip(1);

        for (i, c) in chars.step_by(4).enumerate() {
            if c.is_alphabetic() {

                if stacks.len() < i + 1 {
                    stacks.push(Vec::new());
                }

                println!("Char: {c}, Stack: {i}");
                stacks[i].push(c);
            }
        }

        println!();
    }

    let moves = moves_raw.lines()
        .map(|l| {
           let v: Vec<&str> = l.split(" ").collect();
            (
                v[1].parse::<u16>().unwrap(),
                v[3].parse::<usize>().unwrap() - 1,
                v[5].parse::<usize>().unwrap() - 1
            )
        });


    for (move_count, from_stack, to_stack) in moves {
        println!("Move Count: {move_count}, From Stack: {from_stack}, To Stack: {to_stack}");

        let final_length = (stacks[from_stack].len() - (move_count - 1) as usize) - 1;

        let crates: Vec<char> = stacks[from_stack].drain(final_length..).collect();

        for c in crates {

            stacks[to_stack].push(c);
        }
    }

    println!();

    for mut s in stacks {
        print!("{}", s.pop().unwrap());
    }

    println!();
}
