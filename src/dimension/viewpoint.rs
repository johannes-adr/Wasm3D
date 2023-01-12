use crate::print2;

use super::{
    math::{self, mat_mul},
    mesh::{Mesh2D, Mesh3D, Triangle3D, Triangle2D},
    Point::{Float, Point2D, Point3D},
};

pub struct ViewPoint {
    pos: Point3D,
}

impl Default for ViewPoint {
    fn default() -> Self {
        Self {
            pos: Point3D {
                x: 0.0,
                y: 0.0,
                z: 60.0,
            },
        }
    }
}

impl ViewPoint {
    pub fn project_vision(&self, mesh: &Mesh3D) -> Mesh2D {
        let target: Vec<(Triangle2D,Float)> = mesh
            .vertecies
            .iter()
            .map(|t|{
                let mut z = 0.0;
                let v = t.vertecies;
                for p in v{
                    z+=p.z;
                }
                let edge1 = [v[0],v[1]];


                let z = (v[0].z + v[1].z + v[2].z) / 3.0;
                (self.project_vision_triangle(t),z)


            })
            .collect();
        Mesh2D::new(target)
    }

    fn project_vision_triangle(&self, triangle: &Triangle3D) -> Triangle2D {
        let z = self.pos.z;

        /*let transformed = triangle
            .vertecies
            .map(|point| math::mat_mul([[z, 0.0, 0.0], [0.0, z, 0.0]], point.arr_table()))
            .map(Point2D::from_array_table);*/
        let mat = [[z, 0.0, 0.0], [0.0, z, 0.0]];
        let mut res = [Default::default();3];
        for t in triangle.vertecies.into_iter().enumerate(){
            let tnew = Point2D::from_array_table(mat_mul(mat,t.1.arr_table()));
            res[t.0]=tnew;
        }
        //print2(triangle);
        let t = Triangle2D::from_array(res);
        return t;
    }
}
