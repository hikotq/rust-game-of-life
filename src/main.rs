extern crate rand;

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
            (x - 1, x + 1),
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
    let mut field = Field::new(10, 10);
    loop {
        field.show();
        field = field.update_data();
    }
}
