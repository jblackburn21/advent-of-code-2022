
pub fn main() {

    const INSTR_ADDX: &str = "addx";

    let instructions = include_str!("../input.txt").lines();

    let mut curr_cycle = 0;
    let mut curr_reg = 1;

    for instr in instructions {

        curr_cycle += 1;

        print_pixel(curr_cycle-1, curr_reg);

        if curr_cycle > 39 {
            curr_cycle = 0;
            println!();
        }

        match instr {
            i if i.starts_with(INSTR_ADDX) => {
                let x = i.split_once(" ")
                                                .unwrap().1
                                                .parse::<i32>()
                                                .unwrap();

                curr_cycle += 1;

                print_pixel(curr_cycle-1, curr_reg);

                if curr_cycle > 39 {
                    curr_cycle = 0;
                    println!();
                }

                curr_reg += x;
            },
            _ => () // NOOP
        }
    }

    println!();
}

fn print_pixel(curr_cycle: i32, curr_reg: i32) {

    // sprite 3px, ###
    if curr_reg-1 <= curr_cycle && curr_cycle <= curr_reg+1{
        print!("#");
    }
    else {
        print!(".");
    }

    // println!("  Cycle: {curr_cycle}, Sprint Pixel: [{},{},{}]", curr_reg-1, curr_reg, curr_reg+1);
}


// Mine
// ##..##..##..##..##..##..##..##..##..##..
// ###...###...###...###...###...###...###.
// ####....####....####....####....####....
// #####.....#####.....#####.....#####.....
// ######......######......######......####
// #######.......#######.......#######.....
// Test
// ##..##..##..##..##..##..##..##..##..##..
// ###...###...###...###...###...###...###.
// ####....####....####....####....####....
// #####.....#####.....#####.....#####.....
// ######......######......######......####
// #######.......#######.......#######.....