#![feature(core)]

extern crate core;
extern crate piston;
extern crate image;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate itertools;

use opengl_graphics::{ Gl, OpenGL };
use sdl2_window::Sdl2Window;
use piston::input::{ Button, MouseButton };

const GRID_SIZE: usize = 200;
const CELL_SIZE: usize = 5;
const DELTAS: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

#[derive(Copy)]
enum Cell {
    Alive,
    Dead,
}

struct Grid {
    buffers: [[[Cell; GRID_SIZE]; GRID_SIZE]; 2],
    current_buffer: usize,
}


struct GridIterator<'a, I> where I: Iterator<Item = (usize, usize)> {
    grid: &'a Grid,
    positions: I,
}

impl<'a, I> Iterator for GridIterator<'a, I>  where I: Iterator<Item = (usize, usize)> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        while let Some((x, y)) = self.positions.next() {
            if let Cell::Alive = self.grid.buffers[self.grid.current_buffer][x][y] {
                return Some((x, y));
            }
        }
        return None;
    }
}


impl Grid {
    fn new() -> Grid {
        return Grid{
            buffers: [[[Cell::Dead; GRID_SIZE]; GRID_SIZE]; 2],
            current_buffer: 0,
        };
    }

    fn alive_cells<'a>(&'a self) -> GridIterator<'a, itertools::Product<core::ops::Range<usize>, core::ops::Range<usize>>> {
        return GridIterator {
            grid: self,
            positions: itertools::Product::new(0..GRID_SIZE, 0..GRID_SIZE),
        };
    }

    fn tick(&mut self) {
        let next_buffer = (self.current_buffer + 1) % self.buffers.len();

        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {

                let mut neighbours = 0;
                for &(dx, dy) in DELTAS.iter() {
                    let n_x = wrap(x as isize + dx);
                    let n_y = wrap(y as isize + dy);

                    let neighbour_state = self.buffers[self.current_buffer][n_x][n_y];
                    if let Cell::Alive = neighbour_state {
                        neighbours += 1;
                    }
                }

                let current_state = (self.buffers[self.current_buffer][x][y], neighbours);
                let next_state = match current_state {
                    (Cell::Alive, 2) => Cell::Alive,
                    (_, 3) => Cell::Alive,
                    (_, _) => Cell::Dead,
                };
                self.buffers[next_buffer][x][y] = next_state;
            }
        }
        self.current_buffer = next_buffer;
    }

    fn set_alive(&mut self, x: usize, y: usize) {
        self.buffers[self.current_buffer][x][y] = Cell::Alive;
    }
}


fn wrap(n: isize) -> usize {
    if n < 0 {
        return (GRID_SIZE as isize + n) as usize;
    } else if n as usize >= GRID_SIZE {
        return n as usize - GRID_SIZE;
    } else {
        return n as usize
    }
}


fn main() {
    let opengl = OpenGL::_3_2;
    let (width, height) = ((GRID_SIZE * CELL_SIZE) as u32, (GRID_SIZE * CELL_SIZE) as u32);
    let window = Sdl2Window::new(
        opengl,
        piston::window::WindowSettings {
            title: "Game of flife".to_string(),
            size: [width, height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let mut grid = Box::new(Grid::new());
    let mut drawing = false;
    let mut frame = 0;
    let mut gl = Gl::new(opengl);

    for e in piston::events(window) {
        use piston::event::{ MouseCursorEvent, PressEvent, ReleaseEvent, RenderEvent };

        frame += 1;
        if let Some(args) = e.render_args() {
            let square = graphics::Rectangle::new(graphics::color::BLACK);
            gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
                graphics::clear([1.0; 4], gl);

                for (x, y) in grid.alive_cells() {
                    const CELL_SIZE_F: f64 = (CELL_SIZE as f64);
                    let rect = [x as f64 * CELL_SIZE_F,
                                y as f64 * CELL_SIZE_F,
                                CELL_SIZE_F, CELL_SIZE_F];
                    square.draw(rect, &c, gl);
                }

            });
        }

        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                drawing = true
            }
        }

        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                drawing = false
            }
        }

        if drawing {
            if let Some([mx, my]) = e.mouse_cursor_args() {
                let (x, y) = (mx as usize, my as usize);
                if (x as u32) < width && (y as u32) < height {
                    grid.set_alive((x/CELL_SIZE),  (y/CELL_SIZE));
                }
            }
        } else if frame % 5 == 0 {
            grid.tick();
            frame = 0;
        }
    }
}

#[cfg(test)]
mod gridtest {
    extern crate test;
    use super::Grid;
    use self::test::Bencher;

    #[bench]
    fn bench_grid_tick(b: &mut Bencher) {
        let mut grid = Grid::new();
        b.iter(|| grid.tick());
    }
}