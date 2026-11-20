use raylib::{
    drawing::{RaylibDraw, RaylibDrawHandle},
    ffi::Color,
};

use crate::{
    constants::{
        MAP, MAX_MAP_POSITIONS, POINT_HEIGHT, POINT_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH,
        STEP_MAX_REACH,
    },
    core::{Player, Vector},
};

mod constants;
mod core;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Raycast")
        .build();

    rl.set_target_fps(60);

    let p: Player = Player {
        pos: Vector { x: 12, y: 12 },
        direction: Vector { x: 1, y: -1 },
    };

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHEAT);
        draw_scene(&mut d, &p);
    }
}

fn draw_ray(p: &Player, handler: &mut RaylibDrawHandle) {
    let mut x = (p.pos.x as i32 - 1) * POINT_WIDTH;
    let mut y = (p.pos.y as i32 - 1) * POINT_HEIGHT;

    for _ in 0..STEP_MAX_REACH {
        if p.direction.y != 0 {
            y += p.direction.y as i32 * POINT_HEIGHT;
        }

        if p.direction.x != 0 {
            x += p.direction.x as i32 * POINT_WIDTH;
        }

        handler.draw_rectangle(x as i32, y as i32, POINT_WIDTH, POINT_HEIGHT, Color::YELLOW);
    }
}

fn draw_player(p: &Player, handler: &mut RaylibDrawHandle) {
    let x: i32 = (p.pos.x as i32 - 1) * POINT_WIDTH;
    let y: i32 = (p.pos.y as i32 - 1) * POINT_HEIGHT;

    handler.draw_rectangle(x as i32, y as i32, POINT_WIDTH, POINT_HEIGHT, Color::GREEN);
}

fn draw_field(handler: &mut RaylibDrawHandle) {
    let mut next_x = 0;
    let mut next_y = 0;

    for y in MAP {
        for x in y {
            if x > 0 {
                handler.draw_rectangle(next_x, next_y, POINT_WIDTH, POINT_HEIGHT, Color::GRAY);
            } else {
                handler.draw_rectangle_lines(
                    next_x,
                    next_y,
                    POINT_WIDTH,
                    POINT_HEIGHT,
                    Color::GRAY,
                );
            }

            next_x += POINT_WIDTH;
        }

        next_x = 0;
        next_y += POINT_HEIGHT;
    }
}

fn draw_scene(handler: &mut RaylibDrawHandle, p: &Player) {
    draw_field(handler);
    draw_player(p, handler);
    draw_ray(p, handler);
}
