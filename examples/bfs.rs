#![allow(dead_code, unused_imports)]

use std::collections::{VecDeque, HashSet};
use rand::{self, Rng};

use draw::{self, Circle, Pixel, Point, Result, Sprite, Rectangle, SpriteId, Transform};

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<usize>>,
}

impl Grid {
    pub fn new(width: usize, height: usize, default: usize) -> Grid {
        let mut cells:Vec<Vec<usize>> = Vec::with_capacity(height);
        for _ in 0..height {
            cells.push(vec![default; width])
        }

        Grid { width, height, cells }
    }

    pub fn set_cell(&mut self, x:usize, y: usize, value: usize) {
        self.cells[y][x] = value;
    }

    pub fn get_cell(&self, x:usize, y: usize) -> usize {
        self.cells[y][x]
    }

    pub fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let x = x as isize;
        let y = y as isize;
        let cardinal: Vec<(isize, isize)> = vec![
            (x - 1, y), // left
            (x + 1, y), // right
            (x, y - 1), // up
            (x, y + 1), // down
        ];

        cardinal
            .into_iter()
            .filter(|(x, y)| {
                (*x >= 0) && (*y >= 0) && (*x < self.width as isize) && (*y < self.height as isize)
            })
            .map(|(x, y)| {
                (x as usize, y as usize)
            })
            .collect()
    }

    pub fn increment(&mut self, x: usize, y: usize) {
        self.cells[y][x] += 1;
    }

    pub fn mark_grid(self: &mut Grid, x: usize, y: usize) {
        let mut deque: VecDeque<(usize, usize, usize)> = VecDeque::new();
        let mut seen: HashSet<(usize, usize)> = HashSet::new();

        deque.push_back((x, y, 0));
        seen.insert((x, y));

        while !deque.is_empty() {
            let (x, y, weight) = deque.pop_front().unwrap();

            if weight < self.get_cell(x, y) {
                self.set_cell(x, y, weight);

                for neighbor in self.neighbors(x, y) {
                    if !seen.contains(&neighbor) {
                        seen.insert(neighbor);
                        deque.push_back((neighbor.0, neighbor.1, weight + 1));
                    }
                }
            }
        }
    }
}

fn draw_grid(grid: &Grid, width: i32, height: i32) -> Sprite {
    let cell_width = width / grid.width as i32;
    let cell_height = height / grid.height as i32;
    let mut parent = Sprite::new_container(width, height, 2);
    parent.background(Pixel::white());

    for x in 0..grid.width {
        for y in 0..grid.height {
            let color = match grid.get_cell(x, y) {
                0 => Pixel::rgb(240, 240, 255),
                i => Pixel::grey(255 - 20 * i as u8)
            };

            parent.add_child(
                Rectangle::new(
                    cell_width as u32,
                    cell_height as u32,
                    x as i32 * cell_width as i32,
                    y as i32 * cell_height as i32,
                    color,
                )
            );
        }
    }

    parent
}

/// This marks up a grid with the cost it takes to travel from a random point to the rest of the
/// grid, stopping when it reaches the border of another node's territory.
fn main() -> Result<()> {
    let width = 75;
    let height = 25;

    let mut grid = Grid::new(width, height, 1000000);
    let mut rng = rand::thread_rng();

    for _ in 0..150 {
        grid.mark_grid(
            rng.gen_range(0, width),
            rng.gen_range(0, height),
        )
    }

    draw::save(&draw_grid(&grid, 1500, 500),"tests/img/bfs_clouds.png")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}