use std::collections::HashMap;

pub fn main() {

    const CMD_CD: &str = "cd";
    const CMD_LS: &str = "ls";
    const CD_UP_LEVEL: &str = "..";
    const CD_TOP_LEVEL: &str = "/";

    let mut dir_sizes: HashMap<String, u32> = HashMap::new();
    let mut current_path: Vec<&str> = Vec::new();

    let lines = include_str!("../input.txt").lines();

    for l in lines {
        if l.starts_with("$") {

            let (_, cmd) = l.split_once(" ").unwrap();

            if cmd.starts_with(CMD_CD) {
                let (_, dir) = cmd.split_once(" ").unwrap();
                println!("Change dir: {dir}");

                match dir{
                    CD_TOP_LEVEL => {
                        current_path.clear();
                        current_path.push(CD_TOP_LEVEL);
                    },
                    CD_UP_LEVEL => {
                        current_path.pop();
                    }
                    _ => {
                        current_path.push(dir);
                    }
                }

                let pwd = current_path.join("/");

                if !dir_sizes.contains_key(&pwd) {
                    dir_sizes.insert(pwd, 0);
                }

            } else if cmd.starts_with(CMD_LS) {
                let pwd = current_path.join("/");
                println!("List current path: /{pwd}");
            } else {
                unreachable!();
            }


        } else {
            let (l1, _) = l.split_once(" ").unwrap();

            match l1.parse::<u32>() {
                Ok(size) => {

                    let pwd = current_path.join("/");

                    let dir_size = dir_sizes.get(&pwd).unwrap();

                    dir_sizes.insert(pwd, dir_size + size);
                },
                Err(_) => {} // find dir, ignore
            }
        }
    }

    println!();

    // Gross
    let mut dir_sizes_clone: HashMap<String, u32> = HashMap::new();
    dir_sizes_clone.clone_from(&dir_sizes);

    let mut dir_sizes_clone2: HashMap<String, u32> = HashMap::new();
    dir_sizes_clone2.clone_from(&dir_sizes);

    for (dir, size) in dir_sizes_clone {
        println!("Top: {dir}");
        for d in dir_sizes.keys() {

            if dir.starts_with(d) && !dir.eq(d) {


                let dir_size = dir_sizes_clone2.get(d).unwrap();

                println!("Adding to: {d}, Size: {dir_size}, Add: {size}");

                dir_sizes_clone2.insert(d.to_string(), dir_size + size);
            }
        }
        println!();
    }

    let max_size: u32 = 100000;

    let total_size: u32 =  dir_sizes_clone2.values()
        .filter(|&s| *s < max_size)
        .sum();

    println!("Total size: {total_size}");
}
