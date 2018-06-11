extern crate piston_window;
extern crate rand;

use piston_window::*;

#[derive(Clone)]
struct Cell {
    is_revealed: bool,
    is_mine:bool,
}

#[derive(Clone)]
struct Board {
    cells: Vec<Vec<Cell>>,
}

#[derive(PartialEq, Debug)]
enum MouseState {
    NoneDown,
    LeftDown,
    RightDown,
    BothDown,
}

impl Board {
    fn empty(dim_x: usize, dim_y: usize) -> Self {
        let line : Vec<_> = (0..dim_x).map(|_| Cell{is_revealed: false, is_mine: false}).collect();
        let mut cells : Vec<_> = (0..dim_y).map(|_|line.clone()).collect();

        for l in &mut cells {
            for c in l {
                c.is_revealed = rand::random();
            }
        }
        Board { cells }
    }

    fn dim_x(&self) -> usize { self.cells[0].len() }
    fn dim_y(&self) -> usize { self.cells.len() }

    fn draw<'a>(&self, c: &Context, gl: &mut G2d, glyphs: &mut Glyphs, metrics: &Metrics, hovered_cell: &Option<[usize; 2]>, mouse_state: &MouseState)
    {
        let draw = |color, rect: [f64; 4], gl: &mut G2d| {
            Rectangle::new(color).draw(rect, &DrawState::default(), c.transform, gl);
        };
       
        for y in 0..self.dim_y() {
            for x in 0..self.dim_x() {
                let block_pixels = metrics.block_pixels as f64;
                let border_size = block_pixels / 20.0;
                let outer = [block_pixels * (x as f64) + metrics.insets[0] as f64, block_pixels * (y as f64) + metrics.insets[1] as f64, block_pixels, block_pixels];
                let inner = [outer[0] + border_size, outer[1] + border_size,
                       outer[2] - border_size * 2.0, outer[3] - border_size * 2.0];

                draw([0.2, 0.2, 0.2, 1.0], outer, gl);

                let is_pressed_cell = mouse_state != &MouseState::NoneDown && hovered_cell == &Some([x, y]);

                let inner_color = match self.cells[y][x].is_revealed {
                    false => match is_pressed_cell {
                        false => [0.5,0.5,0.5,1.0],
                        true => [0.65,0.65,0.65,1.0],
                    },
                    true => [0.8,0.8,0.8,1.0],
                };

                draw(inner_color, inner, gl);

                let transform = c.transform.trans(inner[0], inner[1] + inner[3]);
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], (metrics.block_pixels - 4) as u32).draw(
                    "3",
                    glyphs,
                    &c.draw_state,
                    transform,
                    gl
                ).unwrap();
            }
        }
    }
}


struct Metrics {
    block_pixels: usize,
    board_x: usize,
    board_y: usize,
    insets: [u32; 4],
}

impl Metrics {
    fn resolution(&self) -> [u32; 2] {
        [(self.board_x * self.block_pixels) as u32 + self.insets[0] + self.insets[2],
         (self.board_y * self.block_pixels) as u32 + self.insets[1] + self.insets[3]]
    }

    fn cell_at(&self, pos: &[f64; 2]) -> Option<[usize; 2]> {
        let res = self.resolution();
        if pos[1] < self.insets[1] as f64 {
            None
        } else {
            Some([((pos[0] - self.insets[0] as f64) / self.block_pixels as f64) as usize, ((pos[1] - self.insets[1] as f64) / self.block_pixels as f64) as usize])
        }
    }
}

struct Game {
    board: Board,
    metrics: Metrics,
    state: State,
    mouse_pos: [f64; 2],
    mouse_states: [bool; 2],
    mouse_down_cell: Option<[usize; 2]>,
}

enum State {
    Idle,
    CursorDown([usize; 2]),
    CursorDoubleDown([usize; 2]),
    GameOver,
}

impl Game {
    fn new(metrics: Metrics) -> Self {
        let __ = 0;
        let xx = 01;

        Game {
            board: Board::empty(metrics.board_x, metrics.board_y),
            state: State::Idle,
            metrics,
            mouse_pos: [0.0, 0.0],
            mouse_states: [false, false],
            mouse_down_cell: None,
        }
    }

    fn progress(&mut self) {
        let disp = match &mut self.state {
            State::GameOver => return,
            State::Idle => return,
            _ => (), //State::InProgress => return,

        };
    }

    fn render(&self, gl: &mut G2d, c: &Context, glyphs: &mut Glyphs) {

        let mouse_state = if self.mouse_down_cell == self.metrics.cell_at(&self.mouse_pos) {
            match self.mouse_states {
                [false, false] => MouseState::NoneDown,
                [true, false] => MouseState::LeftDown,
                [false, true] => MouseState::RightDown,
                [true, true] => MouseState::BothDown,
            }
        } else {
            MouseState::NoneDown
        };

        self.board.draw(c, gl, glyphs, &self.metrics, &self.mouse_down_cell, &mouse_state);
    }

    fn on_press(&mut self, args: &Button) {
        match args {
            Button::Keyboard(key) => { self.on_key(key); }
            Button::Mouse(button) => { self.on_mouse_down(button); }
            _ => {},
        }
    }

    fn on_release(&mut self, args: &Button) {
        match args {
            Button::Mouse(button) => { self.on_mouse_up(button); }
            _ => {},
        }
    }
    fn on_key(&mut self, key: &Key) {

    }

    fn on_mouse_move(&mut self, pos: &[f64; 2]) {
        self.mouse_pos = *pos;
    }

    fn on_mouse_down(&mut self, button: &MouseButton) {
        if button == &MouseButton::Left {
            self.mouse_states[0] = true;

            if let Some(cell_at_cursor) = self.metrics.cell_at(&self.mouse_pos) {
                self.mouse_down_cell = Some(cell_at_cursor);
            }
        } else if button == &MouseButton::Right {
            self.mouse_states[1] = true;
        }
    }

    fn on_mouse_up(&mut self, button: &MouseButton) {
        if button == &MouseButton::Left {
            self.mouse_states[0] = false;

            if let Some(cell_at_cursor) = self.metrics.cell_at(&self.mouse_pos) {
                let mut cell = &mut self.board.cells[cell_at_cursor[1]][cell_at_cursor[0]];

                if self.mouse_down_cell == Some(cell_at_cursor) {
                    if !(*cell).is_revealed {
                        (*cell).is_revealed = !cell.is_revealed;
                    }
                }
            }
        } else if button == &MouseButton::Right {
            self.mouse_states[1] = false;
        }
    }
}

fn main() {
    let metrics = Metrics {
        block_pixels: 30,
        board_x: 16,
        board_y: 16,
        insets: [0, 20, 0, 0],
    };

    let mut window: PistonWindow
        = WindowSettings::new("Minesweeper", metrics.resolution()).exit_on_esc(true).build().unwrap_or_else(
            |e| { panic!("Failed: {}", e) }
        );

    let mut game = Game::new(metrics);
    let factory = window.factory.clone();
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);

    let mut glyphs = Glyphs::new("assets/FiraSans-Regular.ttf", factory, texture_settings).unwrap();

    while let Some(event) = window.next() {
        game.progress();

        if let Some(args) = event.render_args() {
            window.draw_2d(&event, |c, g| {
                // Set a white background
                clear([1.0, 1.0, 1.0, 1.0], g);

                game.render(g, &c, &mut glyphs);
            });
        }

        if let Some(pos) = event.mouse_cursor_args() {
            game.on_mouse_move(&pos);
        }

        if let Some(args) = event.press_args() {
            game.on_press(&args);
        }

        if let Some(args) = event.release_args() {
            game.on_release(&args);
        }
    }
}
