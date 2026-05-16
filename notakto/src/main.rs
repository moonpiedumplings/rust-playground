mod map;

use std::io;
use std::process;

//use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
// use ratatui::{
//     buffer::Buffer,
//     layout::Rect,
//     style::Stylize,
//     symbols::border,
//     text::{Line, Text},
//     widgets::{Block, Paragraph, Widget},
//     DefaultTerminal, Frame,
// };

// fn main() -> io::Result<()> {
//     ratatui::run(|terminal| map::App::default().run(terminal))
// }





static row_true: [i32; 3] = [1, 1, 1]; 


// see https://en.wikipedia.org/wiki/Notakto
// checks if the game is lost
fn check_loss(grid: [[i32; 3]; 3]) -> bool {

    for i in 0..3 {
        if (grid[i] == row_true) {
            return true;
        }
    }
    for i in 0..3 {
        let mut count = 0;
        for j in 0..3 {
            if (grid[j][i] == 1) {
                count = count + 1;
            }
        }
        if (count == 3) {
            return true;
        }
    }
    if ([grid[0][0], grid[1][1], grid[2][2]] == [1, 1, 1]) {
        return true;
    }
    if ([grid[1][2], grid[2][2], grid[2][1]] == [1, 1, 1]) {
        return true;
    }
    
   return false;
}

// this code does the AI move. Notakto's algorithm is very simple and a forced win for the first player (but don't tell anybody lmao)
fn handle_player_move(grid: &mut[[i32; 3]; 3], x: i32, y: i32) {  
    // try -2, -1 first

    grid[x as usize][y as usize] = 1;
    if (grid[(x - 2).abs() as usize][(y - 1).abs() as usize] != 1) {
        grid[(x - 2).abs() as usize][(y - 1).abs() as usize] = 1;
    }
    else {
        grid[(x -1).abs() as usize][(y - 2).abs() as usize] = 1;
    }
    // then try -1, -2
}

fn print_grid(grid: [[i32; 3]; 3]) {
     for i in 0..3 {
        println!("{:?}", grid[i]); 
    }
}

fn main () {

    let mut grid: [[i32; 3]; 3] = [
        [0, 0, 0],
        [0, 1, 0],
        [0, 0, 0],
    ];

    // main game loop
    println!("Start!!!");
    print_grid(grid);

    while (true) {

        println!();

        println!("Input next move (x y seperated by space):");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // needed because readline keeps trailing newlines
        let input = input.trim();

        let (x, y) = input.split_once(' ').unwrap();
        let x: i32 = x.parse().unwrap();
        let y: i32 = y.parse().unwrap();

        handle_player_move(&mut grid, x, y);

        if (check_loss(grid)) {
            println!("\n");
            println!("You lose!");
        }

        // let x: usize = x.try_into().unwrap();
        // let y: usize = y.try_into().unwrap();

        // grid[x][y] = 1;

        print_grid(grid);
        }

    std::process::exit(0);
}
