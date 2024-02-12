use std::default;

use rand::random;
use raylib::{
    collision,
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
};

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    value: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            x: 0,
            y: 0,
            value: false,
        }
    }
}

impl Cell {
    pub fn is_alive(&self) -> bool {
        self.value
    }

    pub fn update_state(&mut self, neighbors: [[Cell; 3]; 3]) {
        let mut live_neighbors = 0;
        for i in 0..3 {
            for j in 0..3 {
                if i == 1 && j == 1 {
                    continue;
                }
                if neighbors[i][j].is_alive() {
                    live_neighbors += 1;
                }
            }
        }

        if self.is_alive() {
            if live_neighbors < 2 || live_neighbors > 3 {
                self.value = false;
            }
        } else {
            if live_neighbors == 3 {
                self.value = true;
            }
        }
    }
}

pub struct GameOfLife {
    pub cells: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
}

impl GameOfLife {
    pub fn new(rows: i32, cols: i32) -> Self {
        let mut output: Vec<Vec<Cell>> = Vec::new();

        for i in 0..rows {
            let mut row: Vec<Cell> = Vec::new();
            for j in 0..cols {
                row.push(Cell {
                    x: j as usize,
                    y: i as usize,
                    value: rand::random::<bool>(),
                });
            }
            output.push(row);
        }

        // glider pattern
        // output[0][1].value = true;
        // output[1][2].value = true;
        // output[2][0].value = true;
        // output[2][1].value = true;
        // output[2][2].value = true;
        

        GameOfLife {
            cells: output,
            width: cols as usize,
            height: rows as usize,
        }
    }

    pub fn update(&mut self) {
        let mut new_cells = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let mut neighbors: [[Cell; 3]; 3] = [[Cell::default(); 3]; 3];
                for i in 0..3 {
                    for j in 0..3 {
                        let x = (col as i32 - 1 + i) as usize;
                        let y = (row as i32 - 1 + j) as usize;
                        if x < self.width && y < self.height {
                            neighbors[i as usize][j as usize] = self.cells[y][x];
                        }
                    }
                }
                new_cells[row][col].update_state(neighbors);
            }
        }
        self.cells = new_cells;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                if cell.is_alive() {
                    d.draw_rectangle(
                        cell.x as i32 * (1000 / self.width as i32),
                        cell.y as i32 * 1000 / self.height as i32,
                        1000 / self.width as i32,
                        1000 / self.height as i32,
                        Color::WHITE,
                    );
                }
            }
        }
    }
}
