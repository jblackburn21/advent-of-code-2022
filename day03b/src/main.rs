use array_tool::vec::Intersect;

pub fn main() {

    let lines: Vec<&str> = include_str!("../input.txt")
        .lines()
        .collect();

    println!("Lines: {}", lines.len());

    let sum: u32 = lines
        .chunks(3)
        .map(|chunk| {

            println!("{} {} {}", chunk[0], chunk[1], chunk[2]);

            let chars0: Vec<char> = chunk[0].chars().collect();
            let chars1: Vec<char> = chunk[1].chars().collect();
            let chars2: Vec<char> = chunk[2].chars().collect();

            return (chars0, chars1, chars2);
        })
        .map(|(c0,c1, c2)| {
            let i: Vec<char> = c0.intersect(c1).intersect(c2);

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

    println!("Sum: {sum}");
}
