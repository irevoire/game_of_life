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
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let grid = Grid::from_file("../pattern/pulsar").unwrap();
        b.iter(|| exe(grid.clone(), 1000));
    }
}
