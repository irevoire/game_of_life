use core::Grid;
use std::io::{stdin, stdout, Write};
use termion::cursor::DetectCursorPos;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

fn print(grid: &Grid) {
    for line in grid.grid.iter() {
        for cell in line.iter() {
            let res = match cell {
                true => '#',
                false => ' ',
            };
            print!("{}", res);
        }
        print!("\r\n");
    }
}

fn main() {
    let file = std::env::args().skip(1).next();
    let mut grid = match file {
        Some(f) => Grid::from_file(&f).unwrap(),
        None => panic!("give me a name"),
    };

    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let mut evt = stdin.events();
    let (basex, basey) = stdout.cursor_pos().unwrap();
    let (basex, basey) = (basex as usize - 1, basey as usize);

    let mut i = 0;
    loop {
        write!(stdout, "step: {}\r\n", i).unwrap();
        print(&grid);

        match evt.next().unwrap().unwrap() {
            Event::Key(Key::Char('q')) | Event::Key(Key::Esc) => break,
            Event::Key(Key::Ctrl('c')) | Event::Key(Key::Ctrl('d')) => break,
            Event::Key(_) => {
                grid.step();
                i += 1;
            }
            Event::Mouse(me) => match me {
                MouseEvent::Press(_, x, y) => {
                    let (x, y) = (x as usize - 1, y as usize - 1);
                    if x > basex
                        && y > basey
                        && x - basex < grid.grid.len()
                        && y - basey < grid.grid.len()
                    {
                        let (x, y) = (x - basex, y - basey);
                        grid.grid[y][x] = !grid.grid[y][x];
                    }
                }
                _ => (),
            },
            _ => {}
        }
        print!("{}", termion::cursor::Up(grid.grid.len() as u16 + 1));
    }
}
