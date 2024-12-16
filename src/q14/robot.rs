use crate::{grid::Grid, vec2::Vec2};
use std::{str::FromStr, usize};

#[derive(Debug, Clone, Copy)]
pub enum Quadrant {
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
}

#[derive(Debug, Clone, Copy)]
pub struct Robot {
    pos: Vec2<i128>,
    vel: Vec2<i128>,
}

impl Robot {
    pub fn new(pos: Vec2<i128>, vel: Vec2<i128>) -> Self {
        Self { pos, vel }
    }

    pub fn move_robot(&mut self, (max_x, max_y): (i128, i128), n: i128) {
        let new_pos = self.pos + (self.vel * n);

        let mut norm_pos = new_pos % Vec2::new(max_x, max_y);
        if norm_pos.x < 0 {
            norm_pos.x += max_x;
        }
        if norm_pos.y < 0 {
            norm_pos.y += max_y;
        }
        self.pos = norm_pos;
    }

    pub fn quadrant(&self, (max_x, max_y): (i128, i128)) -> Option<Quadrant> {
        use Quadrant::*;
        let middle_x = max_x / 2;
        let middle_y = max_y / 2;
        match (self.pos.x, self.pos.y) {
            (x, y) if x < middle_x && y < middle_y => Some(TopLeft),
            (x, y) if x < middle_x && y > middle_y => Some(BottomLeft),
            (x, y) if x > middle_x && y < middle_y => Some(TopRight),
            (x, y) if x > middle_x && y > middle_y => Some(BottomRight),
            _ => None,
        }
    }
}

impl FromStr for Robot {
    type Err = &'static str;
    // p=0,4 v=3,-3
    // p=6,3 v=-1,2
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = s
            .split_once(" ")
            .ok_or("Failed to split into velocity and position")?;

        let (x, y) = pos
            .strip_prefix("p=")
            .ok_or("Failed to strip position prefix")?
            .split_once(",")
            .ok_or("Failed to get (x, y) components of position")?;

        let x = x
            .parse()
            .map_err(|_| "Failed to parse x position component")?;
        let y = y
            .parse()
            .map_err(|_| "Failed to parse y position component")?;

        let (x_vel, y_vel) = vel
            .strip_prefix("v=")
            .ok_or("Failed to strip velocity prefix")?
            .split_once(",")
            .ok_or("Failed to get (x,y) components of velocity")?;

        let x_vel = x_vel
            .parse()
            .map_err(|_| "Failed to parse x velocity component")?;
        let y_vel = y_vel
            .parse()
            .map_err(|_| "Failed to parse y velocity component")?;

        Ok(Self {
            pos: Vec2::new(x, y),
            vel: Vec2::new(x_vel, y_vel),
        })
    }
}

impl From<&[Robot]> for Grid<bool> {
    fn from(value: &[Robot]) -> Self {
        let mut max_x = 0;
        let mut min_x = usize::MAX;
        let mut max_y = 0;
        let mut min_y = usize::MAX;

        for robot in value.iter() {
            max_x = max_x.max(robot.pos.x as usize);
            min_x = min_x.min(robot.pos.x as usize);
            max_y = max_y.max(robot.pos.y as usize);
            min_y = min_y.min(robot.pos.y as usize);
        }

        let rows = max_y - min_y + 1;
        let cols = max_x - min_x + 1;

        let mut grid = Grid::with_capacity_and_default(rows, cols, false);

        for robot in value.iter() {
            let x = (robot.pos.x - min_x as i128) as usize;
            let y = (robot.pos.y - min_y as i128) as usize;
            grid[(y, x)] = true;
        }

        grid
    }
}
