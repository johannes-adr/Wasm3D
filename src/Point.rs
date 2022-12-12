use std::{ops::{Div, Add, Sub, Mul}, num::ParseIntError};

use crate::{gamewindow::GameWindow, Pixel::PixelCoord};


pub type Float = f32;
#[derive(Debug,Clone,Copy)]
pub struct Point3D {
    pub x: Float,
    pub y: Float,  
    pub z: Float
}


impl Point3D{
    pub fn new(x: Float,y: Float,z: Float)->Self{
        Self{x,y,z}
    }

    pub fn arr(self) -> [Float;3]{
       unsafe{std::mem::transmute(self)}
    }


    pub fn matMul3x3(&mut self, m: &[Float]){
        self.x = self.x*m[0] + self.y*m[1] + self.z*m[2];
        self.y = self.x*m[3] + self.y*m[4] + self.z*m[5];
        self.z = self.x*m[6] + self.y*m[7] + self.z*m[8];
    }
}

#[derive(Debug,Clone,Copy)]
pub struct Point2D{
    pub x: Float,
    pub y: Float,  
}

impl Point2D{
    pub fn new(x: Float, y: Float)->Self{
        Self{x,y}
    }

    pub fn newi(x: i32, y: i32)->Self{
        Self{x: x as Float,y: y as Float}
    }

    pub fn tuple(&self) -> (Float,Float){
        (self.x,self.y)
    }
}