use std::collections::HashSet;

#[derive(Debug)]
struct Board {
    store: [u32; 5 * 5],

}

impl Board {
    const BOARD_WIDTH: usize = 5;
    fn new() -> Self {
        Board {
            store: [0; 5 * 5],
        }
    }

    fn new_from(board_repr: &[&str]) -> Self {
        let mut board = Board {
            store: [0; 5 * 5],
        };

        for (index, val) in board_repr.iter().enumerate() {
            board.store[index] = val.parse().unwrap();
        }

        board
    }

    fn check_row(&self, row: usize) -> [u32; 5] {
        let mut result: [u32; 5] = [0; 5];
        let start_row = row * Board::BOARD_WIDTH;

        for pos in 0..Board::BOARD_WIDTH {
            let row_index = start_row + pos;
            result[pos] = self.store[row_index];
        }

        result
    }


    //NOTE: This was actually not necessary i was a bit too quick when i read
    //the word bingo...
    fn check_diagonals(&self) -> ([u32; 5], [u32; 5]) {
        let mut lr_diag: [u32; 5] = [0; 5];
        let mut rl_diag: [u32; 5] = [0; 5];

        for pos in 0..Board::BOARD_WIDTH {
            let diag_index_lr = pos * Board::BOARD_WIDTH + pos;
            let diag_index_rl = ((pos + 1) * Board::BOARD_WIDTH) - (pos + 1);

            lr_diag[pos] = self.store[diag_index_lr];
            rl_diag[pos] = self.store[diag_index_rl];
        }

        (lr_diag, rl_diag)
    }

    fn check_column(&self, column: usize) -> [u32; 5] {
        let mut result: [u32; 5] = [0; 5];

        for pos in 0..Board::BOARD_WIDTH {
            let column_index = pos * Board::BOARD_WIDTH + column;
            result[pos] = self.store[column_index];
        }

        result
    }

    fn check_bingo(&self, numbers: &HashSet<u32>) -> bool {
        for pos in 0..5 {
            let current_row = HashSet::from(self.check_row(pos));
            let current_column = HashSet::from(self.check_column(pos));

            if current_row.intersection(&numbers).count() == 5 {
                return true;
            }

            if current_column.intersection(&numbers).count() == 5 {
                return true;
            }
        }
        
        false
    }

}

#[derive(Debug)]
struct Game {
    numbers: Vec<u32>,
    round: usize,
    boards: Vec<Board>,
}

impl Game {
    fn new(numbers: Vec<u32>, boards: Vec<Board>) -> Self {
        Self {
            numbers,
            round: 5,
            boards,
        }
    }

    fn next_round(&mut self) {
        self.round += 1;
    }

    fn bingo(&mut self) -> Option<usize> {
        let mut in_play = HashSet::new();
        for num in self.numbers[0..self.round].iter() {
            in_play.insert(num.clone());
        }

        for (index, board) in self.boards.iter().enumerate() {

            if board.check_bingo(&in_play) {
                return Some(index);
            }

        }

        self.next_round();

        None
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let input_split: Vec<&str> = input.split("\n\n").collect::<Vec<&str>>();
    let drawn_numbers: Vec<u32> = input_split[0].split(',').map(|v| v.parse().unwrap()).collect();

    let boards = &input_split[1..]; 


    let mut b = Vec::new();
    for board in boards {
        let board = b.push(Board::new_from(&board.split_whitespace().collect::<Vec<&str>>()));
    }

    let mut game = Game::new(drawn_numbers, b);

    loop {
        if let Some(index) = game.bingo() {
            println!("Found a board index: {}", index);
            let winning_board = &game.boards[index];
            let mut winning_set = HashSet::new();
            for num in winning_board.store {
                winning_set.insert(num);
            }

            for num in &game.numbers[0..game.round] {
                winning_set.remove(num);
            }

            let mut count = 0;

            for num in winning_set {
                count += num;
            }

            let final_number = game.numbers[0..game.round].last().unwrap();
            println!("{} * {} = {}", count, final_number, count * final_number);
            break;
        }

        println!("Advancing a round {:?}", game.numbers);
    }
}
