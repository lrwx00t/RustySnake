#![warn(clippy::all, clippy::pedantic)]
use std::{
    thread,
    time::{self, Duration},
};
extern crate ncurses;
use ncurses::*;

mod board;
mod direction;
use board::*;
use direction::*;

fn main() {
    let num_food = 100;
    initscr();
    cbreak();
    noecho();
    keypad(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // hide cursor
    timeout(100);
    let mut xmax: i32 = 0;
    let mut ymax: i32 = 0;
    let margin = 5;
    getmaxyx(stdscr(), &mut ymax, &mut xmax);
    ymax -= margin;
    xmax -= margin;
    let mut board = board::Board {
        xmax: xmax as u32,
        ymax: ymax as u32,
        foods: vec![],
        snake: vec![],
    };
    board.initialize();
    for _ in 1..num_food {
        let mut point = board.create_random_cell();
        while point.x < (margin + 1) as u32
            || point.y < (margin + 1) as u32
            || point.x > board.xmax
            || point.y > board.ymax
        {
            point = board.create_random_cell();
        }
        board.foods.push(point);
    }
    let mut previous_direction = DIRECTION::RIGHT;
    let mut ppd = previous_direction;
    let mut p = board.moveright();
    let mut paused = false;
    'outer: loop {
        clear();
        // y
        for i in margin..ymax - 1 {
            mvaddch(i + 1 as i32, margin as i32, ACS_VLINE());
            mvaddch(i + 1 as i32, xmax as i32, ACS_VLINE());
        }
        // x
        for i in margin..xmax - 1 {
            mvaddch(margin as i32, i + 1 as i32, ACS_HLINE());
            mvaddch(ymax as i32, i + 1 as i32, ACS_HLINE());
        }
        display_points(&board.snake);
        display_points(&board.foods);
        refresh();

        let mut d = getdirection(previous_direction);
        // println!("prev: {:?} - d: {:?}", previous_direction, d);
        match d {
            DIRECTION::RIGHT => {
                p = board.moveright();
                previous_direction = d;
            }
            DIRECTION::DOWN => {
                p = board.movedown();
                previous_direction = d;
            }
            DIRECTION::UP => {
                p = board.moveup();
                previous_direction = d;
            }
            DIRECTION::LEFT => {
                p = board.moveleft();
                previous_direction = d;
            }
            DIRECTION::PAUSED => {
                paused = !paused;
            }
        }
        if !paused && d != DIRECTION::PAUSED {
            previous_direction = d;
            if board.foods.contains(&p) {
                board.foods.retain(|&x| x != p);
                board.snake.insert(0, p);
            }
            board.move_to(p);
        }
    }
}

fn display_points(snake: &[Point]) {
    for point in snake {
        mvaddch(point.y as i32, point.x as i32, ACS_CKBOARD());
    }
}
