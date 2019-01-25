// main.rs --- 
// 
// Filename: main.rs
// Author: Louise <ludwigette>
// Created: Fri Jan 25 20:21:25 2019 (+0100)
// Last-Updated: Fri Jan 25 20:28:37 2019 (+0100)
//           By: Louise <ludwigette>
//
use specs::prelude::*;
mod render;

fn main() {
    let render_sys = render::RenderSys::new();
}
