use std::mem::transmute;

use crate::print;

use super::{
    math,
    Point::{self, Float, Point2D, Point3D},
};
#[derive(Debug)]
pub struct Mesh3D {
    pub vertecies: Vec<Triangle3D>,
}

impl Mesh3D {
    pub fn new(vertecies: Vec<Triangle3D>) -> Self {
        Self { vertecies }
    }

    pub fn new_direct(vertecies: &[Float]) -> Result<Self, &str> {
        //9 vertecies = 1 triangle
        if vertecies.len() % 9 != 0 {
            return Err("len of vertecies have to be % 3 = 0");
        }
        let len = vertecies.len() / 9;
        let mut target = Vec::with_capacity(len);
        for i in 0..len {
            let p1 = Point3D::new(
                vertecies[i * 9 + 0],
                vertecies[i * 9 + 1],
                vertecies[i * 9 + 2],
            );
            let p2 = Point3D::new(
                vertecies[i * 9 + 3],
                vertecies[i * 9 + 4],
                vertecies[i * 9 + 5],
            );
            let p3 = Point3D::new(
                vertecies[i * 9 + 6],
                vertecies[i * 9 + 7],
                vertecies[i * 9 + 8],
            );
            target.push(Triangle3D::new(p1, p2, p3))
        }
        //print(format!("{:?}",target));
        return Ok(Mesh3D::new(target));
    }

    pub fn new_slice(vertecies: &[Triangle3D]) -> Result<Self, &str> {
        return Ok(Mesh3D::new(
            vertecies
                .iter()
                .map(|s| s.clone())
                .collect::<Vec<Triangle3D>>(),
        ));
    }

    pub fn plane(p1: Point3D, p2: Point3D,z1: i32, z2: i32) -> [Triangle3D;2] {
        let z = (p1.z + p2.z) / 2.0;
        return [
            Triangle3D::from_array_raw([p1.arr(), p2.arr(), [p2.x, p1.y, z1 as Float]]),
            Triangle3D::from_array_raw([p1.arr(), p2.arr(), [p1.x, p2.y, z2 as Float]]),
        ];
    }

    pub fn cube() -> Self{
        let front = Self::plane(Point3D::newi(-1, -1, 1), Point3D::newi(1, 1, 1),1,1);
        let left = Self::plane(Point3D::newi(-1, -1, 1), Point3D::newi(-1, 1, -1),-1,1);
        let top = Self::plane(Point3D::newi(1, -1, -1),Point3D::newi(-1, -1, 1),-1,1);
        let right = Self::plane(Point3D::newi(1, 1, 1),Point3D::newi(1, -1, -1),-1,1);
        let bot = Self::plane(Point3D::newi(1, 1, 1),Point3D::newi(-1, 1, -1),1,-1);
        let back = Self::plane(Point3D::newi(-1, -1, -1), Point3D::newi(1, 1, -1),-1,-1);
        let mut trs = Vec::new();
        trs.extend_from_slice(&front);
        trs.extend_from_slice(&left);
        trs.extend_from_slice(&top);
        trs.extend_from_slice(&right);
        trs.extend_from_slice(&bot);
        trs.extend_from_slice(&back);
        Self::new(trs)
    }

    pub fn rotateX(&mut self, a: Float) {
        let matrix = [
            [1.0, 0.0, 0.0],
            [0.0, a.cos(), -a.sin()],
            [0.0, a.sin(), a.cos()],
        ];
        self.applyMatrix(matrix);
    }

    pub fn rotateY(&mut self, a: Float) {
        let matrix = [
            [a.cos(), 0.0, a.sin()],
            [0.0, 1.0, 0.0],
            [-a.sin(), 0.0, a.cos()],
        ];

        self.applyMatrix(matrix);
    }

    fn applyMatrix(&mut self, mat: [[Float; 3]; 3]) {
        for p in &mut self.vertecies {
            let res = p
                .vertecies
                .map(|p| math::mat_mul(mat, p.arr_table()))
                .map(|arr| Point3D::from_array_table(arr));
            *p = Triangle3D::from_array(res);
        }
    }

    pub fn rotateZ(&mut self, a: Float) {
        let matrix = [
            [a.cos(), -a.sin(), 0.0],
            [a.sin(), a.cos(), 0.0],
            [0.0, 0.0, 1.0],
        ];
        self.applyMatrix(matrix);
    }
}
#[derive(Debug,Clone)]
pub struct Mesh2D {
    pub vertecies: Vec<(Triangle2D,Float)>,
}

impl Mesh2D {
    pub fn new(vertecies: Vec<(Triangle2D,Float)>) -> Self {
        Self { vertecies }
    }

    pub fn new_direct(vertecies: &[Float]) -> Result<Self, &str> {
        if vertecies.len() % 2 != 0 {
            return Err("len of vertecies have to be % 3 = 0");
        }
        let vertecies = vertecies.to_vec();
        todo!();
        //Ok(Self::new(unsafe { std::mem::transmute(vertecies) }))
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Triangle3D {
    pub vertecies: [Point3D; 3],
}

impl Triangle3D {
    pub fn new(t1: Point3D, t2: Point3D, t3: Point3D) -> Self {
        Triangle3D {
            vertecies: [t1, t2, t3],
        }
    }

    pub fn array_table(self) -> [[Point3D; 1]; 3] {
        unsafe { std::mem::transmute(self) }
    }

    pub fn from_array(arr: [Point3D; 3]) -> Self {
        unsafe { std::mem::transmute(arr) }
    }

    pub fn from_array_raw(arr: [[Float; 3]; 3]) -> Self {
        Self::from_array(arr.map(|s| Point3D::new(s[0], s[1], s[2])))
    }
    pub fn from_array_rawi(arr: [[i32; 3]; 3]) -> Self {
        Self::from_array(arr.map(|s| Point3D::new(s[0] as Float, s[1] as Float, s[2] as Float)))
    }
    pub fn from_array_table(points: [[Point3D; 1]; 3]) -> Self {
        unsafe {
            Triangle3D {
                vertecies: std::mem::transmute(points),
            }
        }
    }
}
#[derive(Debug,Clone, Copy)]
pub struct Triangle2D {
    pub vertecies: [Point2D; 3],
}

impl Triangle2D {
    pub fn from_array(arr: [Point2D; 3]) -> Self {
        unsafe { std::mem::transmute(arr) }
    }
}
