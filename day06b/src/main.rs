use std::collections::HashSet;

pub fn main() {

    let chars = include_str!("../input.txt").chars();
    let stream: Vec<char> = chars.collect();
    let stream_len = stream.len();
    let chunk_size = 14;

    for i in 0..stream_len-chunk_size {

        let chunk = &stream[i..i+chunk_size];

        let hs: HashSet<char> = HashSet::from_iter(chunk.to_vec());

        if hs.len() == chunk_size {
            println!("Chars processed: {}, Chunk: {}", i+chunk_size, String::from_iter(chunk.iter()));
            break;
        }
    }

}
