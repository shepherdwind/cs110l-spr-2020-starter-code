use model::*;
use nannou::prelude::*;

pub mod grid;
pub mod model;
pub mod stone;

const ROWS: u32 = 30;
const COLS: u32 = 30;
const SIZE: u32 = 20;
const LINE_WIDTH: f32 = 0.06;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 4 * MARGIN;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::rate_fps(60.0))
        .run()
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();

    Model::new(ROWS as usize, COLS as usize, SIZE as usize)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.next_step();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let gdraw = draw
        .scale(SIZE as f32)
        .scale_y(-1.0)
        .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);

    draw.background().color(SNOW);

    model.grid.walk(Box::new(move |stone, _row, _col| {
        let cdraw = gdraw.x_y(stone.x, stone.y);

        let color = if stone.active { GREEN } else { WHITE };

        cdraw
            .rect()
            .color(color)
            .stroke(BLACK)
            .stroke_weight(LINE_WIDTH)
            .w_h(1.0, 1.0)
            .x_y(0.0, 0.0);
    }));

    let mut message = "Press 'n' to next step, 'c' to clear, 's' to start, 'p' to stop. step: ".to_string();
    message.push_str(&model.step.to_string());

    draw.x_y(0.0, (HEIGHT / 2 - 8) as f32)
        .text(&message)
        .w(WIDTH as f32)
        .color(BLACK)
        .font_size(12);
    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::N => {
            model.next_status();
        }
        Key::C => {
            model.clear();
        }
        Key::S => {
            model.start();
        }
        Key::P => {
            model.stop();
        }
        _other_key => {}
    }
}

fn mouse_pressed(app: &App, model: &mut Model, _button: MouseButton) {
    app.mouse.x;
    model.set_click(app.mouse.x, app.mouse.y);
}
