
pub fn main() {

    const INSTR_NOOP: &str = "noop";
    const INSTR_ADDX: &str = "addx";

    let instructions = include_str!("../input.txt").lines();

    let mut cycle_registers: Vec<i32> = Vec::new();

    for instr in instructions {
        let curr_cycle = cycle_registers.len();
        // default to 1 for first cycle
        let curr_reg_x = if cycle_registers.len() > 0 { cycle_registers[curr_cycle-1] } else { 1 };

        match instr {
            INSTR_NOOP => {
                println!("NOOP - Cycle: {}, Reg: {curr_reg_x}", curr_cycle + 1);
                cycle_registers.push(curr_reg_x);
            },
            i if i.starts_with(INSTR_ADDX) => {
                let x = i.split_once(" ")
                                                .unwrap().1
                                                .parse::<i32>()
                                                .unwrap();

                println!("A C1 - Cycle: {}, Reg: {curr_reg_x}", curr_cycle + 1);

                cycle_registers.push(curr_reg_x);
                let reg_x = curr_reg_x + x;
                cycle_registers.push(reg_x);

                println!("A C2 - Cycle: {}, Reg: {reg_x}, Added: {x}", curr_cycle+2);
            },
            _ => unreachable!()
        }
    }

    println!();

    let sig_strengths: i32 = cycle_registers[18..].iter()
                                    .step_by(40).enumerate()
                                    .map(|(i,r)| {
                                        let c = (i as i32 * 40) + 20;
                                        let sig_strength = c * r;

                                        println!("Cycle: {c}, Reg: {r}, Sig strength: {sig_strength}");

                                         return sig_strength;
                                    })
                                    .sum();

    println!();
    println!("Total signal strengths: {}", sig_strengths);
}


