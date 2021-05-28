use std::collections::HashSet;
use std::collections::VecDeque;
use std::vec::Vec;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str;
use std::process;
use std::hash::{Hash, Hasher};
use rand::Rng;
use std::time::{Duration, Instant};

const DIR: [(i16, i16); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const BLOCK_BITS: [[usize; 16]; 16] = [[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 
                                     [16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31], 
                                     [32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47], 
                                     [48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63], 
                                     [64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79], 
                                     [80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95], 
                                     [96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111], 
                                     [112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127], 
                                     [128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143], 
                                     [144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159], 
                                     [160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175], 
                                     [176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191], 
                                     [192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207], 
                                     [208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223], 
                                     [224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239], 
                                     [240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255]];

#[derive(Hash)]
#[derive(Debug)]
struct Block {
    x: i16,
    y: i16,
    len: i16,
    width: i16,
}

impl Block {
    fn new(x: i16, y: i16, len: i16, width: i16) -> Block {
        //Assert
        Block {
            x,
            y,
            len,
            width,
        }
    }

    fn copy(block: &Block) -> Block {
        Block {
            x: block.x,
            y: block.y,
            len: block.len,
            width: block.width,
        }
    }
}

impl PartialEq for Block { 
    fn eq(&self, other: &Self) -> bool {
        return
            self.x == other.x
            && self.y == other.y
            && self.len == other.len 
            && self.width == other.width;
    }
}

impl Eq for Block {}

struct Board {
    blocks: Vec<Block>,
    len: i16,
    width: i16,
    zobrist: u64,
    moves: u16
}

impl Board {
    fn moved_board(blocks: Vec<Block>, len: i16, width: i16, zobrist: u64, moves: u16) -> Board {
        Board {
            blocks,
            len,
            width,
            zobrist,
            moves,

        }
    }

    fn find_moves(&self, zobrist_pos: &Vec<Vec<Vec<u64>>>) -> Vec<Board> {
        let mut boards: Vec<Board> = Vec::new();
        let length: usize = self.len as usize;
        let width: usize = self.width as usize;

        let mut state = vec![vec![false; length]; width];

        for block in &self.blocks {
            let block_left: usize = block.x as usize;
            let block_right: usize = (block.x + block.width) as usize;
            let block_bot: usize = block.y as usize;
            let block_top: usize = (block.y + block.len) as usize;

            for x in block_left..block_right {
                for y in block_bot..block_top {
                    state[x][y] = true;
                }
            }
        }

        for block in &self.blocks {
            for i in 0..4 {
                Board::do_move(&self, &mut boards, &state, block, i, zobrist_pos);
            }
        }
        return boards;
    }

    fn do_move(board: &Board, boards: &mut Vec<Board>, state: &Vec<Vec<bool>>, block: &Block, dir: usize, zobrist_pos: &Vec<Vec<Vec<u64>>>) {
        let new_left = block.x + DIR[dir].0;
        let new_right = (block.x + block.width) + DIR[dir].0;
        let new_bot = block.y + DIR[dir].1;
        let new_top = (block.y + block.len) + DIR[dir].1;

        if new_left < 0 || new_bot < 0 || new_right > board.width || new_top > board.len {
            return;
        }
        
        match dir {
            0 => { 
                for x in new_left..new_right {
                    if state[x as usize][(new_top - 1) as usize] {
                        return;
                    }
                }
            }
            1 => { 
                for x in new_left..new_right {
                    if state[x as usize][new_bot as usize] {
                        return;
                    }
                }
            }
            2 => { 
                for y in new_bot..new_top {
                    if state[(new_right - 1) as usize][y as usize] {
                        return;
                    }
                }
            }
            3 => { 
                for y in new_bot..new_top {
                    if state[new_left as usize][y as usize] {
                        return;
                    }
                }
            }
            _ => return,
        }

        let mut new_blocks: Vec<Block> = Vec::new();

        for b in &board.blocks {
            if Block::eq(block, b) {
                new_blocks.push(Block::new(new_left, new_bot, b.len, b.width));
            }
            else {
                new_blocks.push(Block::copy(b));
            }
        }

        let mut new_hash: u64 = board.zobrist ^ zobrist_pos[block.x as usize][block.y as usize][BLOCK_BITS[block.width as usize][block.len as usize]];
        new_hash = new_hash ^ zobrist_pos[new_left as usize][new_bot as usize][BLOCK_BITS[block.width as usize][block.len as usize]];

        let new_board: Board = Board::moved_board(new_blocks, board.len, board.width, new_hash, board.moves + 1);
        boards.push(new_board);
    }
}

impl PartialEq for Board { 
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len || self.width != other.width {
            return false;
        }
        else {
            for block in &other.blocks {
                if !(self.blocks.contains(&block)) {
                    return false;
                }
            }
        }
        return true;
    }
}

impl Eq for Board {}
impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.zobrist.hash(state);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut start_file: String = String::from("src/start/");
    let mut finish_file: String = String::from("src/finish/");
    start_file.push_str(&args[1]);
    finish_file.push_str(&args[1]);

    let (start, finish): (Board, Board) = read_input(start_file, finish_file);

    solve(start, finish);  
}

fn read_input(start_file: String, finish_file: String) -> (Board, Board) {
    let mut len: i16 = 1;
    let mut width: i16 = 1;

    let mut block_len: i16;
    let mut block_width: i16;
    let mut block_x: i16;
    let mut block_y: i16;

    let mut start_blocks: Vec<Block> = Vec::new();
    let mut finish_blocks: Vec<Block> = Vec::new();

    if let Ok(lines) = read_lines(start_file) {
        for line in lines {
            if let Ok(split) = line {
                let splitter = split.split_whitespace();
                let nums = splitter.collect::<Vec<&str>>();

                if nums.len() == 2 {
                    width = nums[0].parse().expect("");
                    len = nums[1].parse().expect("");
                }

                if nums.len() == 4 {
                    block_width = nums[0].parse().expect("");
                    block_len = nums[1].parse().expect("");
                    block_x = nums[2].parse().expect("");
                    block_y = nums[3].parse().expect("");

                    start_blocks.push(Block{x: block_x, y: block_y,len: block_len, width: block_width});
                }
            }
        }
    }

    let start: Board = Board{blocks: start_blocks, len: len, width: width, zobrist: 0, moves: 0};

    if let Ok(lines) = read_lines(finish_file) {
        for line in lines {
            if let Ok(split) = line {
                let splitter = split.split_whitespace();
                let nums = splitter.collect::<Vec<&str>>();

                if nums.len() == 2 {
                    let fin_width: i16 = nums[0].parse().expect("");
                    let fin_len: i16 = nums[1].parse().expect("");

                    assert_eq!(fin_width, width);
                    assert_eq!(fin_len, len);
                }

                if nums.len() == 4 {
                    block_width = nums[0].parse().expect("");
                    block_len = nums[1].parse().expect("");
                    block_x = nums[2].parse().expect("");
                    block_y = nums[3].parse().expect("");

                    finish_blocks.push(Block{x: block_x, y: block_y, width: block_width, len: block_len});
                }
            }
        }
    }

    let finish: Board = Board{blocks: finish_blocks, len: len, width: width, zobrist: 0, moves: 0};

    (start, finish)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn solve(start: Board, finish: Board) {
    let start_time = Instant::now();
    let mut boards: HashSet<u64> = HashSet::new();
    let mut queue: VecDeque<Board> = VecDeque::new();
    let mut vec_boards: Vec<Board>;

    let mut total_boards = 0;
    let mut total_moves = 0;

    let mut zobrist_pos: Vec<Vec<Vec<u64>>> = vec![vec![vec![0; 256]; start.len as usize]; start.width as usize];
    for i in 0..start.width as usize {
        for j in 0..start.len as usize {
            for k in 0..256 {
                zobrist_pos[i][j][k] = rand::thread_rng().gen();
            }
        }
    }

    let zobrist_pos = &zobrist_pos;

    queue.push_back(start);

    let mut found = false;

    while !(queue.is_empty()) {
        total_boards += 1;

        let cur_board: Board = queue.pop_front().unwrap();
        boards.insert(cur_board.zobrist);
    
        if cur_board.eq(&finish) {
            total_moves = cur_board.moves;
            found = true;
            break;
        }

        vec_boards = cur_board.find_moves(zobrist_pos);
    
        for board in vec_boards {
            let zobrist: u64 = board.zobrist;
            if !boards.contains(&zobrist) {
                queue.push_back(board);
                boards.insert(zobrist);
            }
        }
    }

    if found {
        println!("Done");
        println!("Moves made: {}", total_moves);
    }

    else {
        println!("No solution found");
    }

    let duration = start_time.elapsed();

    println!("Total boards seen: {}", total_boards);
    println!("Total unique boards seen: {}", boards.len());
    println!("Time elapsed taken is: {:?}", duration);
    
    process::exit(0);
}