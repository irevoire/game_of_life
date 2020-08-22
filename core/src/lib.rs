use rayon::prelude::*;
use std::io::{BufRead, BufReader};

type Error = Box<dyn std::error::Error>;

#[derive(Clone)]
pub struct Grid {
    pub grid: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![vec![false; width]; height],
        }
    }

    pub fn from(grid: Vec<Vec<bool>>) -> Self {
        Self { grid: grid.clone() }
    }

    pub fn from_file(file: &str) -> Result<Self, Error> {
        let file = std::fs::File::open(file)?;
        let reader = BufReader::new(file);
        let grid: Vec<Vec<bool>> = reader
            .lines()
            .map(|line| {
                Ok(line?
                    .split(',')
                    .map(|s| match s {
                        "#" | "1" | "true" => Ok(true),
                        " " | "0" | "false" => Ok(false),
                        _ => Err(format!("Unknown character found: {}", s).into()),
                    })
                    .collect::<Result<Vec<bool>, Error>>()?)
            })
            .collect::<Result<Vec<Vec<bool>>, Error>>()?;
        Ok(Self::from(grid))
    }

    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    /// convert a cell to an usize if the cell does not exist / is out of bounds
    /// return false
    fn get(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
        match grid.get(y).unwrap_or(&Vec::new()).get(x).unwrap_or(&false) {
            false => 0,
            true => 1,
        }
    }

    /// return the number of neighbours
    fn neighbours(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
        let txy = if x > 0 && y > 0 {
            Self::get(grid, x - 1, y - 1)
        } else {
            0
        };
        let tx = if x > 0 {
            Self::get(grid, x - 1, y) + Self::get(grid, x - 1, y + 1)
        } else {
            0
        };
        let ty = if y > 0 {
            Self::get(grid, x, y - 1) + Self::get(grid, x + 1, y - 1)
        } else {
            0
        };
        txy + tx
            + ty
            + Self::get(grid, x + 1, y)
            + Self::get(grid, x, y + 1)
            + Self::get(grid, x + 1, y + 1)
    }

    /// update a cell
    fn update(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
        match (Self::get(grid, x, y), Self::neighbours(grid, x, y)) {
            // 1. any live cell with two or three neighbors survives
            (1, 2..=3) => true,
            // 2. Any dead cell with three live neighbors becomes a live cell
            (0, 3) => true,
            // 3. All other live cells die in the next generation
            // Similarly, all other dead cells stay dead
            _ => false,
        }
    }

    /// return the total number of cells
    fn nb_cells(&self) -> usize {
        self.grid.first().map(|line| line.len()).unwrap_or(0) * self.grid.len()
    }

    pub fn step(&mut self) {
        let tmp = self.grid.clone();

        if self.nb_cells() > 1000 {
            self.grid.par_iter_mut().enumerate().for_each(|(y, line)| {
                line.iter_mut()
                    .enumerate()
                    .for_each(|(x, cell)| *cell = Self::update(&tmp, x, y))
            });
        } else {
            self.grid.iter_mut().enumerate().for_each(|(y, line)| {
                line.iter_mut()
                    .enumerate()
                    .for_each(|(x, cell)| *cell = Self::update(&tmp, x, y))
            });
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> Vec<Vec<bool>> {
        vec![
            vec![false, true, false],
            vec![false, true, false],
            vec![false, true, false],
        ]
    }

    #[test]
    fn test_get() {
        let grid = init();
        // top line
        assert_eq!(Grid::get(&grid, 0, 0), 0);
        assert_eq!(Grid::get(&grid, 1, 0), 1);
        assert_eq!(Grid::get(&grid, 2, 0), 0);

        // middle line
        assert_eq!(Grid::get(&grid, 0, 1), 0);
        assert_eq!(Grid::get(&grid, 1, 1), 1);
        assert_eq!(Grid::get(&grid, 2, 1), 0);

        // bottom line
        assert_eq!(Grid::get(&grid, 0, 2), 0);
        assert_eq!(Grid::get(&grid, 1, 2), 1);
        assert_eq!(Grid::get(&grid, 2, 2), 0);

        // out of bound
        assert_eq!(Grid::get(&grid, 0, 3), 0);
        assert_eq!(Grid::get(&grid, 1, 3), 0);
        assert_eq!(Grid::get(&grid, 3, 2), 0);
        assert_eq!(Grid::get(&grid, 4, 4), 0);
    }

    #[test]
    fn test_neighbours() {
        let grid = init();
        // top line
        assert_eq!(Grid::neighbours(&grid, 0, 0), 2);
        assert_eq!(Grid::neighbours(&grid, 1, 0), 1);
        assert_eq!(Grid::neighbours(&grid, 2, 0), 2);

        // top line
        assert_eq!(Grid::neighbours(&grid, 0, 1), 3);
        assert_eq!(Grid::neighbours(&grid, 1, 1), 2);
        assert_eq!(Grid::neighbours(&grid, 2, 1), 3);

        // bottom line
        assert_eq!(Grid::neighbours(&grid, 0, 2), 2);
        assert_eq!(Grid::neighbours(&grid, 1, 2), 1);
        assert_eq!(Grid::neighbours(&grid, 2, 2), 2);
    }
}
