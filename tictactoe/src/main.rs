use ggez::{
    conf, event, graphics,
    input::{
        keyboard::{self, KeyCode},
        mouse::{self, MouseButton},
    },
    mint, Context, ContextBuilder, GameResult,
};

const WIDTH: f32 = 600.;
const HEIGHT: f32 = 600.;

const SIZE: u16 = 120;
const INNER_SCALE: f32 = 0.99;
const CELL_SCALE: f32 = 0.6;

const TEXT_SCALE: mint::Vector2<f32> = mint::Vector2 {
    x: 2.,
    y: 2.,
};

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
impl Cell {
    fn not_empty(&self) -> bool {
        *self != Cell::Empty
    }
    fn color(&self) -> graphics::Color {
        match *self {
            Cell::X => X_COLOR,
            Cell::O => O_COLOR,
            _ => unreachable!(),
        }
    }
    fn switch_turn(&mut self) {
        *self = match *self {
            Cell::X => Cell::O,
            Cell::O => Cell::X,
            _ => unreachable!(),
        }
    }
    fn as_str(&self) -> &str {
        match *self {
            Cell::X => "Green",
            Cell::O => "Red",
            _ => " ",
        }
    }
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
            if self.cells[i][0].not_empty()
                && self.cells[i][0] == self.cells[i][1]
                && self.cells[i][1] == self.cells[i][2]
            {
                return State::Won(self.cells[i][0]);
            }
        }
        for i in 0..3 {
            if self.cells[0][i].not_empty()
                && self.cells[0][i] == self.cells[1][i]
                && self.cells[1][i] == self.cells[2][i]
            {
                return State::Won(self.cells[0][i]);
            }
        }
        if self.cells[0][0].not_empty()
            && self.cells[0][0] == self.cells[1][1]
            && self.cells[1][1] == self.cells[2][2]
        {
            return State::Won(self.cells[0][0]);
        }
        if self.cells[0][2].not_empty()
            && self.cells[0][2] == self.cells[1][1]
            && self.cells[1][1] == self.cells[2][0]
        {
            return State::Won(self.cells[0][2]);
        }
        if self.cells.iter().flatten().all(|c| c.not_empty()) {
            return State::Draw;
        }
        State::Playing
    }
    fn draw_and_update(&mut self, context: &mut Context) -> GameResult {
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

                if self.cells[x][y].not_empty()
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
                        self.turn.switch_turn();
                    }
                }
                self.batch.add(params);

                if self.cells[x][y].not_empty() {
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
                        .color(self.cells[x][y].color());
                    self.batch.add(params);
                }
            }
        }

        let text = graphics::Text::new(
            match self.state {
                State::Playing => format!("Turn: {}", self.turn.as_str()),
                State::Won(c) => format!("{} won!", c.as_str()),
                State::Draw => "Draw".to_string(),
            }
        );
        let pt = mint::Point2 {
            x: WIDTH * 0.03,
            y: HEIGHT * 0.03,
        };
        graphics::queue_text(context, &text, pt, Some(LINE_COLOR));

        Ok(())
    }
}

impl event::EventHandler for Controller {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BACK_COLOR);
        self.batch.clear();

        self.draw_and_update(context)?;

        graphics::draw(context, &self.batch, graphics::DrawParam::new())?;
        graphics::draw_queued_text(
            context,
            graphics::DrawParam::new().scale(TEXT_SCALE),
            None,
            graphics::FilterMode::Nearest,
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
