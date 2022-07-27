use ggez::{
    conf, event, graphics,
    input::{
        keyboard::{self, KeyCode},
        mouse::{self, MouseButton},
    },
    mint, timer, Context, ContextBuilder, GameResult,
};
use rand::Rng;

const WIDTH: f32 = 600.;
const HEIGHT: f32 = 600.;

const SIZE: u16 = 120;
const INNER_SCALE: f32 = 0.99;

const CELL_SCALE: f32 = 0.6;

const BACK_COLOR: graphics::Color = graphics::Color::BLACK;
const LINE_COLOR: graphics::Color = graphics::Color::WHITE;
const SELECT_COLOR: graphics::Color = graphics::Color::BLUE;

const X_COLOR: graphics::Color = graphics::Color::GREEN;
const O_COLOR: graphics::Color = graphics::Color::RED;

struct ToggleKey {
    was_down: bool,
}
impl ToggleKey {
    pub fn new() -> Self {
        Self { was_down: false }
    }
    fn down(&mut self, state: bool) -> bool {
        if !self.was_down && state {
            self.was_down = true;
            return true;
        } else if !state {
            self.was_down = false;
        }
        false
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Empty,
    X,
    O,
}

#[derive(PartialEq)]
enum State {
    Playing,
    Won(Cell),
    Draw,
}

struct Controller {
    batch: graphics::spritebatch::SpriteBatch,
    cells: [[Cell; 3]; 3],
    turn: Cell,
    state: State,
    mouse_tk: ToggleKey,
}
impl Controller {
    pub fn new(white_batch: graphics::spritebatch::SpriteBatch) -> Self {
        Self {
            batch: white_batch,
            cells: [[Cell::Empty; 3]; 3],
            turn: Cell::X,
            state: State::Playing,
            mouse_tk: ToggleKey::new(),
        }
    }
    fn reset(&mut self) {
        self.cells = [[Cell::Empty; 3]; 3];
        self.turn = Cell::X;
    }
    fn check_win(&mut self) -> State {
        for i in 0..3 {
            if self.cells[i][0] != Cell::Empty
                && self.cells[i][0] == self.cells[i][1]
                && self.cells[i][1] == self.cells[i][2]
            {
                return State::Won(self.cells[i][0]);
            }
        }
        for i in 0..3 {
            if self.cells[0][i] != Cell::Empty
                && self.cells[0][i] == self.cells[1][i]
                && self.cells[1][i] == self.cells[2][i]
            {
                return State::Won(self.cells[0][i]);
            }
        }
        if self.cells[0][0] != Cell::Empty
            && self.cells[0][0] == self.cells[1][1]
            && self.cells[1][1] == self.cells[2][2]
        {
            return State::Won(self.cells[0][0]);
        }
        if self.cells[0][2] != Cell::Empty
            && self.cells[0][2] == self.cells[1][1]
            && self.cells[1][1] == self.cells[2][0]
        {
            return State::Won(self.cells[0][2]);
        }
        if self.cells.iter().flatten().all(|c| *c != Cell::Empty) {
            return State::Draw;
        }
        State::Playing
    }
    fn draw_grid(&mut self, context: &mut Context) -> GameResult {
        if keyboard::is_key_pressed(context, KeyCode::R) {
            self.reset();
        }
        self.state = self.check_win();

        let pt = mint::Point2 {
            x: (WIDTH - 3. * SIZE as f32) / 2.,
            y: (HEIGHT - 3. * SIZE as f32) / 2.,
        };
        let params = graphics::DrawParam::new()
            .scale(mint::Vector2 { x: 3., y: 3. })
            .dest(pt)
            .color(LINE_COLOR);
        self.batch.add(params);

        for x in 0..3 {
            for y in 0..3 {
                let pt = mint::Point2 {
                    x: (WIDTH - 3. * SIZE as f32) / 2.
                        + x as f32 * (SIZE as f32)
                        + (1. - INNER_SCALE) / 2. * SIZE as f32,
                    y: (HEIGHT - 3. * SIZE as f32) / 2.
                        + y as f32 * (SIZE as f32)
                        + (1. - INNER_SCALE) / 2. * SIZE as f32,
                };
                let scale = mint::Vector2 {
                    x: INNER_SCALE,
                    y: INNER_SCALE,
                };
                let mut params = graphics::DrawParam::new().scale(scale).dest(pt);
                let mouse_pos = mouse::position(context);

                if self.cells[x][y] != Cell::Empty
                    || !(mouse_pos.x >= pt.x
                        && mouse_pos.x <= pt.x + INNER_SCALE * SIZE as f32
                        && mouse_pos.y >= pt.y
                        && mouse_pos.y <= pt.y + INNER_SCALE * SIZE as f32)
                    || self.state != State::Playing
                {
                    params = params.color(BACK_COLOR);
                } else {
                    params = params.color(SELECT_COLOR);
                    if self
                        .mouse_tk
                        .down(mouse::button_pressed(context, MouseButton::Left))
                        && self.state == State::Playing
                    {
                        self.cells[x][y] = self.turn;
                        self.turn = match self.turn {
                            Cell::X => Cell::O,
                            Cell::O => Cell::X,
                            _ => unreachable!(),
                        };
                    }
                }
                self.batch.add(params);

                if self.cells[x][y] != Cell::Empty {
                    let pt = mint::Point2 {
                        x: (WIDTH - 3. * SIZE as f32) / 2.
                            + x as f32 * (SIZE as f32)
                            + (1. - CELL_SCALE) / 2. * SIZE as f32,
                        y: (HEIGHT - 3. * SIZE as f32) / 2.
                            + y as f32 * (SIZE as f32)
                            + (1. - CELL_SCALE) / 2. * SIZE as f32,
                    };
                    let scale = mint::Vector2 {
                        x: CELL_SCALE,
                        y: CELL_SCALE,
                    };
                    let params = graphics::DrawParam::new()
                        .scale(scale)
                        .dest(pt)
                        .color(match self.cells[x][y] {
                            Cell::X => X_COLOR,
                            Cell::O => O_COLOR,
                            _ => unreachable!(), // should be unreachable
                        });
                    self.batch.add(params);
                }
            }
        }

        Ok(())
    }
    // fn draw_cells(&mut self, context: &mut Context) -> GameResult {
    //     for x in 0..WIDTH as usize {
    //         for y in 0..HEIGHT as usize {
    //             let color = self.cells[x][y].get_color(self.draw_mode);
    //             let pt = mint::Point2 {
    //                 x: x as f32 * (SIZE + SPACER) + SPACER,
    //                 y: y as f32 * (SIZE + SPACER) + SPACER,
    //             };
    //             if color != graphics::Color::BLACK {
    //                 // no point in drawing if it is the same color as the background
    //                 let params = graphics::DrawParam::new().dest(pt).color(color);
    //                 self.batch.add(params);
    //                 if self.draw_mode == 3 {
    //                     let text = graphics::Text::new(self.cells[x][y].neighbors.to_string());
    //                     let pt = mint::Point2 {
    //                         x: pt.x + SIZE * 1.0 / 4.0,
    //                         y: pt.y + SIZE * 1.0 / 8.0,
    //                     };
    //                     graphics::queue_text(context, &text, pt, Some(graphics::Color::RED));
    //                 }
    //             }
    //         }
    //     }
    //     println!(
    //         "FPS: {} | Updates per second: {}",
    //         timer::fps(context) as usize,
    //         self.fps
    //     );
    //     Ok(())
    // }
    // fn handle_input(&mut self, context: &Context) {
    //     if self
    //         .space_toggle
    //         .down(keyboard::is_key_pressed(context, KeyCode::Space))
    //     {
    //         self.draw_mode = (self.draw_mode + 1) % 4;
    //     }
    //     if self
    //         .mouse_toggle
    //         .down(mouse::button_pressed(context, MouseButton::Left))
    //     {
    //         self.paused = !self.paused;
    //     }
    // }
}
impl event::EventHandler for Controller {
    fn update(&mut self, context: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BACK_COLOR);
        self.batch.clear();

        self.draw_grid(context)?;
        graphics::draw(context, &self.batch, graphics::DrawParam::new())?;
        graphics::draw_queued_text(
            context,
            graphics::DrawParam::new(),
            None,
            graphics::FilterMode::Linear,
        )?;

        graphics::present(context)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ContextBuilder::new("TicTacToe", "LelsersLasers")
        .window_setup(conf::WindowSetup::default().title("TicTacToe"))
        .window_mode(conf::WindowMode::default().dimensions(WIDTH, HEIGHT));
    let (mut context, event_loop) = cb.build()?;

    let controller = Controller::new(graphics::spritebatch::SpriteBatch::new(
        graphics::Image::solid(&mut context, SIZE, graphics::Color::WHITE)?,
    ));

    event::run(context, event_loop, controller);
}
