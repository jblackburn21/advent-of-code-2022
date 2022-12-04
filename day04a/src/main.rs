
pub fn main() {

    let contained_pairs: u16 = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(range1, range2)| {
            let split1 = range1.split_once("-").unwrap();
            let split2 = range2.split_once("-").unwrap();

            let s1 = split1.0.parse::<u16>().unwrap();
            let e1 = split1.1.parse::<u16>().unwrap();
            let s2 = split2.0.parse::<u16>().unwrap();
            let e2 = split2.1.parse::<u16>().unwrap();

            return ((s1,e1), (s2,e2));
        })
        .map(|((s1,e1), (s2,e2))|{
            return if s1 >= s2 && e1 <= e2 {
                println!("F: {s1}-{e1}, S: {s2}-{e2} :: First contained in second");
                1
            } else if s2 >= s1 && e2 <= e1 {
                println!("F: {s1}-{e1}, S: {s2}-{e2} :: Second contained in first");
                1
            } else {
                println!("F: {s1}-{e1}, S: {s2}-{e2} :: Not contained");
                0
            }
        })
        .sum();

    println!("Contained pairs: {contained_pairs}");
}
