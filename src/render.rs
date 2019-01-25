// render.rs --- 
// 
// Filename: render.rs
// Author: Louise <ludwigette>
// Created: Fri Jan 25 20:21:32 2019 (+0100)
// Last-Updated: Fri Jan 25 20:26:54 2019 (+0100)
//           By: Louise <ludwigette>
// 
use piston_window::{PistonWindow, WindowSettings};
use specs::prelude::*;

pub struct RenderSys {
    window: PistonWindow
}

impl RenderSys {
    pub fn new() -> Self {
        let window = WindowSettings::new("GGJ 2019", [640, 480])
            .exit_on_esc(true).build().unwrap();

        RenderSys {
            window
        }
    }
}

impl<'a> System<'a> for RenderSys {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {
        
    }
}
