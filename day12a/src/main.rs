use pathfinding::prelude::bfs;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub i16, pub i16);

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct Successor {
    pub pos: Pos,
}

pub struct Board {
    pub width: u8,
    pub height: u8,
    pub data: Vec<Vec<u8>>
}

impl Board {
    pub fn new(board_lines: Vec<&str>) -> Board {
        let width = board_lines[0].len() as u8;
        let height = board_lines.len() as u8;
        let mut data = Vec::new();
        for board_line in board_lines {
            let mut row: Vec<u8> = Vec::new();
            for c in board_line.chars() {
                match c {
                    'S' => row.push('a' as u8),
                    'E' => row.push('z' as u8),
                    _ => row.push(c as u8),
                }
            }
            data.push(row);
        }
        Board { width, height, data }
    }

    pub fn get_successors(&self, position: &Pos) -> Vec<Successor> {
        let mut successors = Vec::new();
        for (dx, dy) in &[(0, -1), (-1, 0), (1, 0), (0, 1)] {

            let new_position = Pos(position.0 + dx, position.1 + dy);

            // check for edge
            if new_position.0 < 0 || new_position.0 >= self.width.into() || new_position.1 < 0 || new_position.1 >= self.height.into() {

                println!("Check Pos: [{},{}], New: [{},{}] - Edge", position.0, position.1, new_position.0, new_position.1);
                continue;
            }

            let pos_val = self.data[position.1 as usize][position.0 as usize];
            let new_pos_val = self.data[new_position.1 as usize][new_position.0 as usize];

            if new_pos_val > pos_val && (new_pos_val - pos_val) > 1 {
                println!("Check Pos: [{},{}] - {}, New: [{},{}] - {} - Skip, too big", position.0, position.1, pos_val, new_position.0, new_position.1, new_pos_val);
                continue;
            }

            println!("Check Pos: [{},{}] - {}, New: [{},{}] - {}", position.0, position.1, pos_val, new_position.0, new_position.1, new_pos_val);

            successors.push(Successor { pos: new_position });
        }

        println!();

        successors
    }
}

fn main() {

    let lines: Vec<&str> = include_str!("../input.txt").lines().collect();

    let board = Board::new(lines);

    let start = Pos(0,20);
    let goal = Pos(91,20);

    let result = bfs(
        &start,
        |p| board.get_successors(p).iter().map(|successor| successor.pos).collect::<Vec<_>>(),
        |p| *p==goal);
    let result = result.expect("No path found");

    println!();

    for p in &result {
        println!("[{},{}], {}", p.0, p.1, board.data[p.1 as usize][p.0 as usize] as char);
    }

    println!();
    println!("total cost: {:}", &result.len()-1);
}
