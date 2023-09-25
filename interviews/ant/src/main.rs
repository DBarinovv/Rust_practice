use bit_vec::BitVec;
use image::{ImageBuffer, Luma};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

/// Represents the possible directions that the ant can face.
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    /// Rotates the ant's direction 90 degrees to the right.
    ///
    /// # Returns
    ///
    /// The new Direction after rotating right.
    fn rotate_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    /// Rotates the ant's direction 90 degrees to the left.
    ///
    /// # Returns
    ///
    /// The new Direction after rotating left.
    fn rotate_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

/// Represents the ant in the simulation, including its position and direction.
struct Ant {
    x: i32, // The x-coordinate of the ant.
    y: i32, // The y-coordinate of the ant.
    direction: Direction, // The current direction that the ant is facing.
}

impl Ant {
    /// Moves the ant one step based on the color of the cell it is currently on.
    ///
    /// # Arguments
    ///
    /// * `grid` - The BitVec representing the grid.
    fn step(&mut self, grid: &mut BitVec) {
        let idx = (self.y as u32 * WIDTH + self.x as u32) as usize;
        let on_white_cell = grid[idx];

        // Rotate and flip the cell color.
        self.direction = if on_white_cell {
            self.direction.rotate_right()
        } else {
            self.direction.rotate_left()
        };

        grid.set(idx, !on_white_cell);

        // Move forward in the current direction.
        self.move_forward();
    }

    /// Moves the ant one step forward in its current direction.
    fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
        };
    }
}

fn main() {
    // Initialize a white grid and the ant.
    let mut grid = BitVec::from_elem((WIDTH * HEIGHT) as usize, true);
    let mut ant = Ant {
        x: 511,
        y: 511,
        direction: Direction::Up,
    };

    // Run the simulation until the ant moves out of the grid bounds.
    while ant.x >= 0 && ant.x < WIDTH as i32 && ant.y >= 0 && ant.y < HEIGHT as i32 {
        ant.step(&mut grid);
    }

    // Count and print the number of black cells.
    let black_cells_count = grid.iter().filter(|&v| !v).count();
    println!("Black cells count: {}", black_cells_count);

    // Create and save the resulting image.
    let img = ImageBuffer::<Luma<u8>, Vec<u8>>::from_raw(
        WIDTH,
        HEIGHT,
        grid.into_iter().map(|v| if v { 255 } else { 0 }).collect(),
    )
    .unwrap();

    img.save("ant_path.png").unwrap();
}
