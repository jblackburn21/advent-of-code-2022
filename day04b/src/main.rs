
pub fn main() {

    let overlapping_pairs: u16 = include_str!("../input.txt")
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
            return if s1 <= e2 && e1 >= s2 {
                println!("F: {s1}-{e1}, S: {s2}-{e2} :: Has overlap");
                1
            } else {
                println!("F: {s1}-{e1}, S: {s2}-{e2} :: No overlap");
                0
            }
        })
        .sum();

    println!("Overlapping pairs: {overlapping_pairs}");
}


// 5-7,7-9 overlaps in a single section, 7.
// 2-8,3-7 overlaps all of the sections 3 through 7.
// 6-6,4-6 overlaps in a single section, 6.
// 2-6,4-8 overlaps in sections 4, 5, and 6.
