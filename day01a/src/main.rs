
pub fn main() {

    let items = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<String>>();

    let mut vec: Vec<u32> = Vec::new();

    let mut x = 0;

    for item in items.iter() {
        if !item.trim().is_empty() {
            let c = item.parse::<u32>().unwrap();

            x = x + c;
        }
        else {
            vec.push(x);
            x = 0;
        }
    }

    let max = vec.iter().max().unwrap();
    println!("{}", max);

}
