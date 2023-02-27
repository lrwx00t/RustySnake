extern crate ncurses;
use ncurses::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DIRECTION {
    UP, DOWN, RIGHT, LEFT, PAUSED
}

pub fn getdirection(prev: DIRECTION) -> DIRECTION {
    let space = ' ' as i32;
    let d = getch();
    if d == space {
      return DIRECTION::PAUSED;
    }
    match d {
      KEY_RIGHT => DIRECTION::RIGHT,
      KEY_LEFT => DIRECTION::LEFT,
      KEY_DOWN => DIRECTION::DOWN,
      KEY_UP => DIRECTION::UP,
      _ => prev
    }
}