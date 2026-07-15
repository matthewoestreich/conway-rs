use crate::{
    conway::Conway,
    grid::Grid,
    render::{Renderer, Viewport},
};
use raylib::{
    ffi::{Color, KeyboardKey, MouseButton},
    prelude::RaylibDraw as _,
};

mod conway;
mod grid;
mod gui;
mod render;

struct AppState {
    pub is_paused: bool,
    //pub is_fast_forwarding: bool,
    //pub fast_forward_speed: usize,
}

const BACKGROUND_COLOR: Color = Color::BLACK;
const WINDOW_WIDTH: i32 = 1200;
const WINDOW_HEIGHT: i32 = 800;
const VIEWPORT_WIDTH: i32 = 1000;
const VIEWPORT_HEIGHT: i32 = 800;
const ROWS: u32 = 50;
const COLS: u32 = 50;
const UPDATE_INTERVAL_SECS: f32 = 0.3; // 1.0 == 1 second

fn main() {
    let (mut rl, thread) = raylib::init()
        .title("Conway")
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .build();

    rl.set_target_fps(60);

    let mut app_state = AppState {
        is_paused: true,
        //is_fast_forwarding: false,
        //fast_forward_speed: 5,
    };

    let renderer = Renderer::new(Viewport::new(
        0,
        0,
        VIEWPORT_WIDTH,
        VIEWPORT_HEIGHT,
        COLS,
        ROWS,
    ));

    let mut conway = Conway::new(UPDATE_INTERVAL_SECS, Grid::new(COLS, ROWS));

    while !rl.window_should_close() {
        if app_state.is_paused {
            rl.set_window_title(&thread, "Conway PRESS SPACE TO UNPAUSE");
        } else {
            rl.set_window_title(&thread, "Conway");
            conway.update(rl.get_frame_time());
        }

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            app_state.is_paused = !app_state.is_paused;
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            && let Some(pos) = renderer.viewport.world_to_cell(rl.get_mouse_position())
        {
            conway.clicked_coords(pos.x, pos.y);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BACKGROUND_COLOR);

        renderer.draw(&mut d, &mut conway);
    }
}
