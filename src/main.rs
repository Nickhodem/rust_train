use std::io;

fn main() {

    let mut game = Board{ fields: [[' ';3];3] };
    game.reset();

    let end = game.check_end();
    game.render();
    let mut state = State{gamer: 'x', column:0, row: 0};
    loop{
        state.action(&mut game);
        game.render();
        if game.check_winner() || game.check_end(){
            game.reset();
            game.render();
            println!("Starts over, player {}", state.gamer);
        }
    }
}


struct State{
    gamer: char, // x, or o
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
                println!{"Player {} wins!", self.gamer};
            }
            if board.check_end(){
                println!{"Game over"};
            }
            self.switch_gamer();
        }
        else{
            println!("Invalid move, try again {}", self.gamer);
            self.action(board);
        }

    }

    fn switch_gamer(& mut self){
        if self.gamer == 'x'{
            self.gamer = 'o';
        }
        else{
            self.gamer = 'x';
        }
        println!("Gamer now: {}", self.gamer);
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
    fields: [[char; 3];3],
}

impl Board {

    fn reset(& mut self){
        self.fields = [[' ';3];3];
    }
    fn set_field(& mut self, c: char, x: usize, y: usize) -> bool{
        if x > self.fields.len()-1 || y > self.fields[0].len()-1{
            return false;
        }
        if self.fields[x][y] != ' '{
            return false;
        }
        self.fields[x][y] = c;
        return true;
    }

    fn check_winner(&self) -> bool {
        for y in 0 .. self.fields.len(){
            if self.fields[y][0] != ' ' && self.fields[y][0] == self.fields[y][1] && self.fields[y][1] == self.fields[y][2]{
                return true;
            }
        }
        for y in 0 .. self.fields.len(){
            if self.fields[0][y] != ' ' && self.fields[0][y] == self.fields[1][y] && self.fields[1][y] == self.fields[2][y]{
                return true;
            }
        }

        if self.fields[0][0] != ' ' && self.fields[0][0] == self.fields[1][1] && self.fields[1][1] == self.fields[2][2]{
            return true;
        }
        if self.fields[2][0] != ' ' && self.fields[2][0] == self.fields[1][1] && self.fields[1][1] == self.fields[0][2]{
            return true;
        }
        false
    }
    fn check_end(&self) -> bool {
        for y in 0 .. self.fields.len(){
            for x in 0 .. self.fields[0].len(){
                if self.fields[x][y] == ' ' {
                return false;
                }
            }
            }
        return true;
    }

    fn render(&self){
        for y in 0 .. self.fields.len(){
            for x in 0 .. self.fields[0].len(){
                print!("{}", self.fields[x][y]);
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