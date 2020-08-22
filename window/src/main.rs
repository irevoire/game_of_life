use core::Grid;
use minifb::{Window, WindowOptions};

fn grid_to_pixel(grid: &Grid) -> Vec<u32> {
    grid.grid
        .iter()
        .flat_map(|line| line.iter())
        .map(|el| if *el { 0xffffffff } else { 0x00000000 })
        .collect()
}

fn main() {
    let file = std::env::args().skip(1).next();
    let mut grid = match file {
        Some(f) => Grid::from_file(&f).unwrap(),
        None => panic!("give me a name"),
    };

    let mut window = Window::new(
        "Test",
        800,
        800,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");

    let mut i = 0;
    println!("step: {}", i);
    while window.is_open() {
        println!("step: {}", i);
        window
            .update_with_buffer(&grid_to_pixel(&grid), grid.width(), grid.height())
            .unwrap();
        grid.step();
        i += 1;
    }
}
