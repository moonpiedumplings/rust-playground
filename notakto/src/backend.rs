

let grid: [[i32; 3]; 3] = [
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
];

let check : [i32; 3] = [1, 1, 1];


// see https://en.wikipedia.org/wiki/Notakto
// checks if the game is lost
fn check_loss() -> bool {

    for i in 0..2 {

        let mut counter = 0;
        for j in 0..2 {
            if grid[i][j] == 1 {
                counter = counter + 1;
            }
            if counter = 3 {
                return true;
            }
        }
    }

    for i in 0..2 {
    }
}