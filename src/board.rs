use crate::direction::DIRECTION;



#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
  pub x: u32,
  pub y: u32
}

#[derive(Debug)]
pub struct Board {
  pub xmax: u32,
  pub ymax: u32,
  pub snake: Vec<Point>,
  pub foods: Vec<Point>
}

impl Board {
  fn eat_food(&mut self, point: Point) {
    self.snake.insert(0, point);
  }

  pub fn create_random_cell(&self) -> Point {
    Point {
      x: rand::random::<u32>()% self.xmax, 
      y: rand::random::<u32>()% self.ymax
    }
  }

  pub fn move_to(&mut self, point: Point) {
    self.snake.insert(0, point);
    self.snake.pop();
  }
  
  pub fn initialize(&mut self) {
    let sx = self.xmax / 2;
    let sy = self.ymax / 2;
    self.snake.push(Point{x: sx, y: sy});
    // self.snake.push(Point{x: sx-1, y: sy-1});
    // self.snake.push(Point{x: sx-2, y: sy-2});
    // self.snake.push(Point{x: sx-3, y: sy-3});
    // self.snake.push(Point{x: sx-4, y: sy-4});
    // self.snake.push(Point{x: sx-5, y: sy-5});
  }

  pub fn move_to_directoin(&mut self, d: DIRECTION, paused: bool) -> Point {
    let mut p = self.snake[0];
    match d  {
        DIRECTION::RIGHT => {
            p = self.moveright();
        },
        DIRECTION::DOWN => {
            p = self.movedown();
        },
        DIRECTION::UP => {
            p = self.moveup();
        },
        DIRECTION::LEFT => {
            p = self.moveleft();
        },
        DIRECTION::PAUSED => ()

    }
    return p;
  }

  pub fn moveright(&mut self) -> Point{
    let head = &self.snake[0];
    let mut new_x = head.x as i32;
    let mut new_y = head.y as i32;
    new_x += 1;
    if new_x <= 5 || new_y <= 5 {
      new_x = (self.xmax - 1) as i32;
    } else if new_x >= self.xmax as i32 || new_y >= self.ymax as i32 {
      new_x = 6;
    }
    Point { x: new_x as u32, y: new_y as u32 }
  }

  pub fn moveleft(&mut self) -> Point{
    let head = &self.snake[0];
    let mut new_x = head.x as i32;
    let mut new_y = head.y as i32;
    new_x -= 1;
    if new_x <= 5 {
      new_x = (self.xmax - 1) as i32;
    } else if new_x >= self.xmax as i32 || new_y >= self.ymax as i32 {
      new_x = 5 as i32;
    }
    Point { x: new_x as u32, y: new_y as u32 }
  }

  pub fn movedown(&mut self) -> Point{
    let head = &self.snake[0];
    let mut new_x = head.x as i32;
    let mut new_y = head.y as i32;
    new_y += 1;
    if new_x <= 5 || new_y <= 5 {
      new_y = (self.ymax - 1) as i32;
    } else if new_x >= self.xmax as i32 || new_y >= self.ymax as i32 {
      new_y = 6;
    }
    Point { x: new_x as u32, y: new_y as u32 }
  }

  pub fn moveup(&mut self) -> Point{
    let head = &self.snake[0];
    let mut new_x = head.x as i32;
    let mut new_y = head.y as i32;
    new_y -= 1;
    if new_x <= 5 || new_y <= 5 {
      new_y = (self.ymax-1) as i32;
    } else if new_x >= self.xmax as i32 || new_y + 1 >= self.ymax as i32 {
      new_y = 5;
    }
    Point { x: new_x as u32, y: new_y as u32 }
  }

}