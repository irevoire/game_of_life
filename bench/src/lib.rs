#![feature(test)]
extern crate test;

use core::Grid;

fn exe(mut grid: Grid, iter: usize) -> usize {
    for _ in 0..iter {
        grid.step();
    }
    iter // we return iter so this function won’t be optimized out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader};
    use test::Bencher;
    type Error = Box<dyn std::error::Error>;

    fn parse_grid() -> Result<Vec<Vec<bool>>, Error> {
        let file = std::fs::File::open("../pattern/pulsar")?;
        let reader = BufReader::new(file);
        reader
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
            .collect::<Result<Vec<Vec<bool>>, Error>>()
    }

    fn generate_large_grid() -> Result<Grid, Error> {
        let mut grid = parse_grid()?;
        grid.iter_mut().for_each(|line| {
            *line = line.repeat(1000);
        });
        Ok(Grid::from(grid))
    }

    #[bench]
    fn large_bench(b: &mut Bencher) {
        let grid = generate_large_grid().unwrap();
        b.iter(|| exe(grid.clone(), 10));
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let grid = Grid::from_file("../pattern/pulsar").unwrap();
        b.iter(|| exe(grid.clone(), 1000));
    }
}
