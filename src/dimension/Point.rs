use std::{ops::{Div, Add, Sub, Mul}, num::ParseIntError};

use crate::{gamewindow::GameWindow, Pixel::PixelCoord};


pub type Float = f32;
#[derive(Debug,Clone,Copy,Default)]
pub struct Point3D {
    pub x: Float,
    pub y: Float,  
    pub z: Float
}


impl Point3D{
    pub fn new(x: Float,y: Float,z: Float)->Self{
        Self{x,y,z}
    }

    pub fn newi(x: i32,y: i32,z: i32)->Self{
        Self{x: x as Float,y:y as Float,z:z as Float}
    }

    pub fn arr(self) -> [Float;3]{
       unsafe{std::mem::transmute(self)}
    }

    pub fn arr_table(self) -> [[Float;1];3]{
        [[self.x],[self.y],[self.z]]
    }

    pub fn from_array_table(points: [[Float;1];3]) -> Self{
        let mut p = Self::default();
        p.x = points[0][0];
        p.y = points[1][0];
        p.z = points[2][0];
        return p
    }

    pub fn matMul3x3(&mut self, m: &[Float]){
        let this = self.arr();
        let mut res = [0.0;3];
        for i in 0..3{
            let t = &mut res[i];
            for j in 0..3{
                *t += this[j] * m[i*3+j]
            }
        }
        self.x = res[0];
        self.y = res[1];
        self.z = res[2];
        /*let x = self.x*m[0] + self.y*m[1] + self.z*m[2];
        let y = self.x*m[3] + self.y*m[4] + self.z*m[5];
        let z = self.x*m[6] + self.y*m[7] + self.z*m[8];
        self.x = x;
        self.y = y;
        self.z = z;*/
    }
}

#[derive(Debug,Clone,Copy, Default)]
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

    pub fn from_array_table(arr:  [[Float; 1]; 2])->Self{
        let mut p = Self::default();
        p.x = arr[0][0];
        p.y = arr[1][0];
        return p;
    }

    pub fn tuple(&self) -> (Float,Float){
        (self.x,self.y)
    }

    pub fn tuplei32(&self) -> (i32,i32){
        (self.x as i32,self.y as i32)
    }
}