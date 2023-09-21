use bit_vec::BitVec;
use image::{ImageBuffer, Luma};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Ant {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Ant {
    fn step(&mut self, grid: &mut BitVec) {
        let idx = (self.y as u32 * WIDTH + self.x as u32) as usize;
        let on_white_cell = !grid[idx];

        self.direction = if on_white_cell {
            match self.direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            }
        } else {
            match self.direction {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
            }
        };

        grid.set(idx, on_white_cell);

        match self.direction {
            Direction::Up => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
        }
    }
}

fn main() {
    let mut grid = BitVec::from_elem((WIDTH * HEIGHT) as usize, false);
    let mut ant = Ant {
        x: 512,
        y: 512,
        direction: Direction::Up,
    };

    while ant.x >= 0 && ant.x < WIDTH as i32 && ant.y >= 0 && ant.y < HEIGHT as i32 {
        ant.step(&mut grid);
    }

    // black cells cnt
    let black_cells_count = grid.iter().filter(|&v| v).count();
    println!("Black cells cnt: {}", black_cells_count);

    // creating img
    let img = ImageBuffer::<Luma<u8>, Vec<u8>>::from_raw(
        WIDTH,
        HEIGHT,
        grid.into_iter().map(|v| if v { 0 } else { 255 }).collect(),
    )
    .unwrap();

    img.save("ant_path.png").unwrap();
}
