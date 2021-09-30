mod state;

use std::{env, path};

use crate::state::State;
use ggez::conf::*;
use ggez::*;

pub(crate) const WINDOW_WIDTH: f32 = 1400.0;
pub(crate) const WINDOW_HEIGHT: f32 = 960.0;

pub fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ContextBuilder::new("chess", "Olle Strand")
        .window_setup(WindowSetup::default().title("Olle Schack").vsync(true))
        .window_mode(
            WindowMode::default()
                .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
                .resizable(false),
        )
        .add_resource_path(resource_dir)
        .build()
        .unwrap();

    let state: State = State::new(&mut ctx).unwrap();

    event::run(ctx, event_loop, state);
}
