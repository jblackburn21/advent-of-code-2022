
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

    vec.sort_by(|a,b| b.cmp(a));

    let top3: u32 = vec.iter().take(3).sum();

    println!("{}", top3);

}
