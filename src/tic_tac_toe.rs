mod tic_tac_toe;

use std::io;
use crate::Cell::{Empty, Taken};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Taken(Player),
}

fn main() {

    let mut game = Board{ fields: [[Cell::Empty;3];3] };
    game.reset();

    let end = game.check_end();
    game.render();
    let mut state = State{gamer: Player::X, column:0, row: 0};
    loop{
        state.action(&mut game);
        game.render();
        if game.check_winner() || game.check_end(){
            game.reset();
            game.render();
            println!("Starts over, player {:#?}", state.gamer);
        }
    }
}


struct State{
    gamer: Player, // x, or o
    column: usize,
    row: usize,
}

impl State {
    fn action(& mut self, board: &mut Board){
        println!("Please provide column: ");
        let column = self.get_input();
        println!("Please provide row: ");
        let row = self.get_input();
        if board.set_field(self.gamer, column, row){
            if board.check_winner(){
                println!{"Player {:#?} wins!", self.gamer};
            }
            if board.check_end(){
                println!{"Game over"};
            }
            self.switch_gamer();
        }
        else{
            println!("Invalid move, try again {:#?}", self.gamer);
            self.action(board);
        }

    }

    fn switch_gamer(& mut self){
        if self.gamer == Player::X{
            self.gamer = Player::O;
        }
        else{
            self.gamer = Player::X;
        }
        println!("Gamer now: {:#?}", self.gamer);
    }

    fn get_input(&self) -> usize{
        loop {
            let mut provided_column = String::new();

            io::stdin()
                .read_line(&mut provided_column)
                .expect("Read line failed");

            let guess: usize = match provided_column.trim().parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("Incorrect number");
                    continue;
                }
            };
            return guess;
        }
    }
}

struct Board{
    fields: [[Cell; 3];3],
}

impl Board {

    fn reset(& mut self){
        self.fields = [[Cell::Empty;3];3];
    }
    fn set_field(& mut self, player: Player, x: usize, y: usize) -> bool{
        if x > self.fields.len()-1 || y > self.fields[0].len()-1{
            return false;
        }
        if self.fields[x][y] != Cell::Empty{
            return false;
        }

        self.fields[x][y] = Cell::Taken(player);
        return true;
    }

    fn check_winner(&self) -> bool {
        for y in 0 .. self.fields.len(){
            if self.fields[y][0] != Cell::Empty && self.fields[y][0] == self.fields[y][1] && self.fields[y][1] == self.fields[y][2]{
                return true;
            }
        }
        for y in 0 .. self.fields.len(){
            if self.fields[0][y] != Cell::Empty && self.fields[0][y] == self.fields[1][y] && self.fields[1][y] == self.fields[2][y]{
                return true;
            }
        }

        if self.fields[0][0] != Cell::Empty && self.fields[0][0] == self.fields[1][1] && self.fields[1][1] == self.fields[2][2]{
            return true;
        }
        if self.fields[2][0] != Cell::Empty && self.fields[2][0] == self.fields[1][1] && self.fields[1][1] == self.fields[0][2]{
            return true;
        }
        false
    }
    fn check_end(&self) -> bool {
        for y in 0 .. self.fields.len(){
            for x in 0 .. self.fields[0].len(){
                if self.fields[x][y] == Cell::Empty {
                    return false;
                }
            }
        }
        return true;
    }

    fn render(&self){
        for y in 0 .. self.fields.len(){
            for x in 0 .. self.fields[0].len(){
                if self.fields[x][y] == Empty{
                    print!(" ");
                }
                if self.fields[x][y] == Cell::Taken(Player::X){
                    print!("x");
                }
                if self.fields[x][y] == Cell::Taken(Player::O){
                    print!("o");
                }

                if y< self.fields[0].len(){
                    print!("|");
                }
            }
            println!("");
            if y < self.fields.len()-1{
                println!("-----")
            }
        }
    }
}