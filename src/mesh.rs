use std::mem::transmute;

use crate::{Point::{Point3D, Float, Point2D}, Pixel::PixelCoord};




pub struct Mesh3D{
    pub vertecies: Vec<Point3D>
}

impl Mesh3D{
    pub fn new(vertecies: Vec<Point3D>)->Self{
        Self{vertecies}
    }

    pub fn new_direct(vertecies: &[Float])->Result<Self,&str>{
        if vertecies.len() % 3 != 0{
            return Err("len of vertecies have to be % 3 = 0");
        }
        let len = vertecies.len()/3;
        let mut target = Vec::with_capacity(len);
        for i in 0..len{
            target.push(Point3D::new(vertecies[i*3], vertecies[i*3+1], vertecies[i*3+2]));
        }
        return Ok(Mesh3D::new(target));
    }



    pub fn cube() -> Self{
        Self::new_direct(&[
            -1.0, -1.0, 1.0,
            1.0, -1.0, 1.0,
            1.0, 1.0, 1.0,
            -1.0, 1.0, 1.0,

            
        ]).unwrap()
    }


    pub fn rotateX(&mut self, a: Float){
        let matrix: &[Float] = &[1.0,0.0,0.0,
                                 0.0,a.cos(),-a.sin(),
                                 0.0,a.sin(),a.cos()];

        for  p in &mut self.vertecies{
            p.matMul3x3(matrix);
        }
    }

    pub fn rotateY(&mut self, a: Float){
        let matrix: &[Float] = &[a.cos(),0.0,a.sin(),
                                 0.0,1.0,0.0,
                                 -a.sin(),0.0,a.cos()];

        for  p in &mut self.vertecies{
            p.matMul3x3(matrix);
        }
    }

    pub fn rotateZ(&mut self, a: Float){
        let matrix: &[Float] = &[a.cos(),-a.sin(),0.0,
                                 a.sin(),a.cos(),0.0,
                                 0.0,0.0,1.0];

        for  p in &mut self.vertecies{
            p.matMul3x3(matrix);
        }
    }
}

pub struct Mesh2D{
    pub vertecies: Vec<Point2D>
}

impl Mesh2D{
    pub fn new(vertecies: Vec<Point2D>)->Self{
        Self{vertecies}
    }

    pub fn new_direct(vertecies: &[Float])->Result<Self,&str>{
        if vertecies.len() % 2 != 0{
            return Err("len of vertecies have to be % 3 = 0");
        }
        let vertecies = vertecies.to_vec();
        Ok(Self::new(unsafe{std::mem::transmute(vertecies)}))
    }
}