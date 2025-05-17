use array2d::{Array2D, Error};
use std::{io, thread, time};
use std::io::{Read, Write};


#[allow(dead_code)]
fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn draw_buffer(array2d: &Array2D<bool>) {
    let rows = array2d.num_rows();
    let columns = array2d.num_columns();

    for row in 0..rows {
        for column in 0..columns {
            if array2d[(row, column)] {
                print!("*");
            } else {
                print!(" ");
            }
        }
    }

    io::stdout().flush().unwrap()
}

fn will_live(neighbors: [bool; 9]) -> bool {
    let neighbours_count = neighbors
        .iter()
        .enumerate()
        .filter(|(i,_)| *i != 4)
        .filter(|(_,alive)| **alive)
        .count();

    match (neighbors[4], neighbours_count) {
        (_, 0) => false,
        (_, 1) => false,
        (_, 2) => neighbors[4],
        (_, 3) => true,
        _ => false
    }
}

#[allow(dead_code)]
fn get_next_step(cells: Array2D<bool>) -> Array2D<bool> {
    let mut output: Array2D<bool> = cells.clone();
    for x in 0..120 {
        for y in 0..29 {
            let not_first_row = y != 0;
            let not_first_column = x != 0;
            let not_last_row = y != 28;
            let not_last_column = x != 119;
            output[(y, x)] = will_live([
                not_first_column && not_first_row && cells[(y - 1, x - 1)], not_first_row && cells[(y - 1, x)], not_last_column && not_first_row && cells[(y - 1, x + 1)],
                not_first_column && cells[(y, x - 1)],                      cells[(y, x)],                      not_last_column && cells[(y, x + 1)],
                not_first_column && not_last_row && cells[(y + 1, x - 1)],  not_last_row && cells[(y + 1, x)],  not_last_column && not_last_row && cells[(y + 1, x + 1)]
            ]);
        }
    }

    output
}

#[allow(dead_code)]
fn initilize_board(pattern: [[char; 10]; 10]) -> Array2D<bool> {
    let mut result = Array2D::filled_with(false, 29, 120);
    for x in 0..10 {
        for y in 0..10 {
            result.set(y, x, pattern[x][y] != ' ').unwrap();
        }
    }

    result
}

fn main() -> Result<(), Error> {
    let _pattern = [[
        '*', '*', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '*'],[
        '*', '*', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],[
        ' ', ' ', ' ', ' ', ' ', '*', ' ', ' ', ' ', ' '],[
        ' ', ' ', ' ', ' ', '*', '*', '*', ' ', ' ', ' '],[
        ' ', ' ', ' ', ' ', ' ', '*', '*', ' ', ' ', ' '],[
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],[
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],[
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],[
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],[
        '*', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '*']];

    let mut buffer = initilize_board(_pattern);

    let mut counter = 0;
    let pause_iteration = 10;
    
    loop {
        if counter == pause_iteration {
            pause();
            counter = 0;
        }
        
        buffer = get_next_step(buffer);
        draw_buffer(&buffer);
        
        thread::sleep(time::Duration::from_millis(1000));
        
        counter += 1;
    }
}
