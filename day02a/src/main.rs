
pub fn main() {

    const PLAYER1_ROCK: &str="A";
    const PLAYER1_PAPER: &str="B";
    const PLAYER1_SCISSORS: &str="C";

    const PLAYER2_ROCK: &str="X";
    const PLAYER2_PAPER: &str="Y";
    const PLAYER2_SCISSORS: &str="Z";

    const SCORE_PLAYER1_WIN: u16=0;
    const SCORE_PLAYER2_WIN: u16=6;
    const SCORE_DRAW: u16=3;
    const SCORE_ROCK: u16=1;
    const SCORE_PAPER: u16=2;
    const SCORE_SCISSORS: u16=3;

    let total_score = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold(0, |accum, (player1_move, player2_move) | {

            if player1_move == PLAYER1_ROCK && player2_move == PLAYER2_ROCK {
                let score = SCORE_DRAW + SCORE_ROCK;
                let a = accum + score;
                println!("P1: ROCK, P2: ROCK - DRAW - Score {score} - Accum: {a}");
                return a;
            }
            else if player1_move == PLAYER1_PAPER && player2_move == PLAYER2_PAPER {
                let score = SCORE_DRAW + SCORE_PAPER;
                let a = accum + score;
                println!("P1: PAPER, P2: PAPER - DRAW - Score {score} - Accum: {a}");
                return a;
            }
            else if player1_move == PLAYER1_SCISSORS && player2_move == PLAYER2_SCISSORS {
                let score = SCORE_DRAW + SCORE_SCISSORS;
                let a = accum + score;
                println!("P1: SCISSORS, P2: SCISSORS - DRAW - Score {score} - Accum: {a}");
                return a;
            }
            else if player1_move == PLAYER1_ROCK && player2_move == PLAYER2_SCISSORS {
                let score = SCORE_PLAYER1_WIN + SCORE_SCISSORS;
                let a = accum + score;
                println!("P1: ROCK, P2: SCISSORS - P1 W - Score {score} - Accum: {a}");
                return a;
            }
            else if player1_move == PLAYER1_ROCK && player2_move == PLAYER2_PAPER {
                let score = SCORE_PLAYER2_WIN + SCORE_PAPER;
                let a = accum + score;
                println!("P1: ROCK, P2: PAPER - P2 W - Score {score} - Accum: {a}");
                return a;
            }
            else if player1_move == PLAYER1_PAPER && player2_move == PLAYER2_ROCK {
                let score = SCORE_PLAYER1_WIN + SCORE_ROCK;
                let a = accum + score;
                println!("P1: PAPER, P2: ROCK - P1 W - Score {score} - Accum: {a}");
                return a;
            }
            else if player1_move == PLAYER1_PAPER && player2_move == PLAYER2_SCISSORS {
                let score = SCORE_PLAYER2_WIN + SCORE_SCISSORS;
                let a = accum + score;
                println!("P1: PAPER, P2: SCISSORS - P2 W - Score {score} - Accum: {a}");
                return a;
            }
            else if player1_move == PLAYER1_SCISSORS && player2_move == PLAYER2_PAPER {
                let score = SCORE_PLAYER1_WIN + SCORE_PAPER;
                let a = accum + score;
                println!("P1: SCISSORS, P2: PAPER - P1 W - Score {score} - Accum: {a}");
                return a;
            }
            else if player1_move == PLAYER1_SCISSORS && player2_move == PLAYER2_ROCK {
                let score = SCORE_PLAYER2_WIN + SCORE_ROCK;
                let a = accum + score;
                println!("P1: SCISSORS, P2: ROCK - P2 W - Score {score} - Accum: {a}");
                return a;
            }
            else {
                unreachable!()
            }
        });

    println!("{total_score}");

}
