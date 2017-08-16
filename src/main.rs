extern crate rand;

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::env;
use std::{thread, time};
use rand::Rng;

#[derive(Copy, Clone)]
enum Cell {
    Alive,
    Dead,
}

struct Field {
    width: usize,
    height: usize,
    data: Vec<Cell>,
}

impl Field {
    fn read_field(filename: &str) -> Result<Field, String> {
        use self::Cell::*;
        let path = Path::new(filename);
        let display = path.display();

        let file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
            Ok(file) => file,
        };

        let input = BufReader::new(file);
        let mut data = Vec::new();
        let mut line_length = 0;
        for line in input.lines() {
            line_length = line_length + 1;
            let line = match line {
                Ok(line) => line,
                Err(e) => {
                    panic!("An error occurred while reading a line {}", e);
                }
            };
            for c in line.chars() {
                if c == '\n' {
                    continue;
                }
                data.push(if c == '■' { Alive } else { Dead });
            }
        }

        let height = line_length;
        let width = data.len() / height;
        Ok(Field {
            width: width,
            height: height,
            data: data,
        })
    }

    fn new(w: usize, h: usize) -> Field {
        use self::Cell::*;
        let mut data: Vec<Cell> = Vec::new();
        for _ in 0..(w * h) {
            let rand_num = rand::thread_rng().gen_range(0, 2);
            data.push(if rand_num == 1 { Alive } else { Dead });
        }
        Field {
            width: w,
            height: h,
            data: data,
        }
    }

    fn show(&self) {
        use self::Cell::*;
        std::process::Command::new("clear")
            .status()
            .unwrap()
            .success();
        let print_cell = |cell: &Cell| if let Alive = *cell {
            print!("O")
        } else {
            print!(" ")
        };
        for (index, cell) in self.data.iter().enumerate() {
            print_cell(cell);
            if (index + 1) % self.width == 0 {
                println!("");
            }
        }
    }

    fn on_field(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn get_cell(&self, x: usize, y: usize) -> Result<Cell, String> {
        if self.on_field(x, y) {
            Ok(self.data[x + y * self.width])
        } else {
            Err("Out of range!!".to_string())
        }
    }

    fn get_neighbor_cells(&self, cell_index: usize) -> Vec<Cell> {
        let x: i32 = (cell_index % self.width) as i32;
        let y: i32 = ((cell_index - x as usize) / self.width) as i32;
        let dir: Vec<(i32, i32)> = vec![
            (x + 1, y),
            (x + 1, y - 1),
            (x, y - 1),
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];
        let dir: Vec<(usize, usize)> = dir.into_iter()
            .filter(|&(x, y)| x >= 0 && y >= 0)
            .map(|(x, y)| (x as usize, y as usize))
            .collect::<Vec<(usize, usize)>>();
        let neighbor_cells = dir.into_iter()
            .filter(|&(dir_x, dir_y)| self.on_field(dir_x, dir_y))
            .map(|(dir_x, dir_y)| self.get_cell(dir_x, dir_y).unwrap())
            .collect::<Vec<Cell>>();
        return neighbor_cells;
    }

    fn calc_next_cell_status(&self, cell_index: usize) -> Cell {
        use self::Cell::*;
        let neighbor_cells = self.get_neighbor_cells(cell_index);
        let neighbor_cells_num = neighbor_cells
            .into_iter()
            .filter(|cell| match *cell {
                Alive => true,
                Dead => false,
            })
            .collect::<Vec<Cell>>()
            .len();
        if neighbor_cells_num <= 1 || 4 <= neighbor_cells_num {
            Dead
        } else if neighbor_cells_num == 3 {
            Alive
        } else {
            self.data[cell_index]
        }
    }

    fn update_data(&self) -> Field {
        let mut new_data = Vec::new();
        for (cell_index, _) in self.data.iter().enumerate() {
            new_data.push(self.calc_next_cell_status(cell_index));
        }
        Field {
            width: self.width,
            height: self.height,
            data: new_data,
        }
    }
}

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => {
            return;
        }
    };
    let mut field = Field::read_field(&filename).unwrap();
    let sleep_time = time::Duration::from_millis(100);
    loop {
        field.show();
        field = field.update_data();
        thread::sleep(sleep_time);
    }
}
