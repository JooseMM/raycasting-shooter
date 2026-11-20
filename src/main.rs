use raylib::{
    drawing::{RaylibDraw, RaylibDrawHandle},
    ffi::Color,
};

struct Vector {
    x: u8,
    y: u8,
}

const CAMARA_REACH: u8 = 10;

struct Player {
    pos: Vector,
    direction: Vector,
}

const SCREEN_WIDTH: i32 = 600;
const SCREEN_HEIGHT: i32 = 600;

const POINT_WIDTH: i32 = SCREEN_WIDTH / 24;
const POINT_HEIGHT: i32 = SCREEN_HEIGHT / 24;

const MAP: [[u8; 24]; 24] = [
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
];

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Raycast")
        .build();

    rl.set_target_fps(60);

    let p: Player = Player {
        pos: Vector { x: 12, y: 12 },
        direction: Vector { x: 0, y: 1 },
    };

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHEAT);
        draw_scene(&mut d, &p);
    }
}

fn draw_ray(p: &Player, handler: &mut RaylibDrawHandle) {
    let mut x = p.pos.x as i32;
    let mut y = p.pos.y as i32;

    if p.direction.y != 0 {
        y = 1 * p.direction.y as i32 * POINT_HEIGHT;
    }

    if p.direction.x != 0 {
        x += 2 * p.direction.x as i32 * POINT_WIDTH;
    }

    handler.draw_rectangle(x as i32, y as i32, POINT_WIDTH, POINT_HEIGHT, Color::YELLOW);
}

fn draw_player(p: &Player, handler: &mut RaylibDrawHandle) {
    let x: i32 = (p.pos.x as i32 - 1) * POINT_WIDTH;
    let y: i32 = (p.pos.y as i32 - 1) * POINT_HEIGHT;

    handler.draw_rectangle(x as i32, y as i32, POINT_WIDTH, POINT_HEIGHT, Color::GREEN);
}

fn draw_scene(handler: &mut RaylibDrawHandle, p: &Player) {
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

    draw_player(p, handler);
}
