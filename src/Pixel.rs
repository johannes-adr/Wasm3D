use core::num;
use std::{ops::{Div, Add, Sub, Mul}, num::ParseIntError};


use crate::gamewindow::GameWindow;

pub type number = usize;

#[derive(Debug,Clone,Copy,Default)]
pub struct PixelCoord {
    pub x: number,
    pub y: number,  
}


impl PixelCoord{
    pub fn new(x: number, y: number) -> Self{
        Self { x, y}
    }


    pub fn in_bounds(&self, win: &GameWindow) -> Result<(),&str>{
       /* if self.x >= win.width {
            return Err("X out of range")
        };
        if self.y >= win.height {
            return Err("Y out of range")
        };*/
        return Ok(())
    }

    pub fn in_bounds_panic(&self, win: &GameWindow){
        self.in_bounds(win).unwrap();
    }

    pub fn arr(self) -> [number;2]{
       unsafe{std::mem::transmute(self)}
    }
}

impl Add for PixelCoord{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x + rhs.x,self.y + rhs.y)
    }
}

impl Add<number> for PixelCoord{
    type Output = Self;

    fn add(self, rhs: number) -> Self::Output {
        Self::Output::new(self.x + rhs,self.y + rhs)
    }
}

impl Sub for PixelCoord{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x,self.y - rhs.y)
    }
}

impl Sub<number> for PixelCoord{
    type Output = Self;

    fn sub(self, rhs: number) -> Self::Output {
        Self::Output::new(self.x - rhs,self.y - rhs)
    }
}


impl Div for PixelCoord{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x / rhs.x,self.y / rhs.y)
    }
}

impl Div<number> for PixelCoord{
    type Output = Self;

    fn div(self, rhs: number) -> Self::Output {
        Self::Output::new(self.x / rhs,self.y / rhs)
    }
}


impl Mul for PixelCoord{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x * rhs.x,self.y * rhs.y)
    }
}

impl Mul<number> for PixelCoord{
    type Output = Self;

    fn mul(self, rhs: number) -> Self::Output {
        Self::Output::new(self.x * rhs,self.y * rhs)
    }
}

