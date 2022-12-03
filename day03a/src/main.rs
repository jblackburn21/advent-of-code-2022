use array_tool::vec::Intersect;

pub fn main() {

    let priority_sum: u32 = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let chars: Vec<char> = l.chars().collect();

            let array_len = chars.len();

            let first_half = chars[..array_len/2].to_vec();
            let second_half = chars[array_len/2..].to_vec();

            return (first_half, second_half);
        })
        .map(|(f,s)| {
            let i: Vec<char> = f.intersect(s);

            return i.into_iter().nth(0).unwrap();
        })
        .map(|c| {

            let char_dec = c as u32;
            // Lower case ASCII in decimal starts at 97, score those 1-26
            // Upper case ASCII in decimal start at 65, score those 27-52
            let priority = if char_dec > 96 { char_dec - 96  } else { char_dec - 38 };

            println!("Char: {c}, Char Dec: {char_dec}, Priority: {priority}");

            return priority;
        })
        .sum();

    println!("Priority sum: {priority_sum}");

}

// The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment
// contains the items vJrwpWtwJgWr, while the second compartment contains the items hcsFMMfFFhFp.
// The only item type that appears in both compartments is lowercase p.