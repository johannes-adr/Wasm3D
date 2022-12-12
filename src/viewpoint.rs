use crate::{Point::{Point3D, Point2D}, mesh::{Mesh3D, Mesh2D}};


pub struct ViewPoint{
    pos: Point3D
}

impl Default for ViewPoint{
    fn default() -> Self {
        Self { pos: Point3D { x: 0.0, y: 0.0, z: -10.0 } }
    }
}

impl ViewPoint{
    pub fn project_vision(&self,mesh: &Mesh3D) -> Mesh2D {
        let target: Vec<Point2D> = mesh.vertecies
                                                        .iter()
                                                        .map(|p|self.project_vision_point(p))
                                                        .collect();

        Mesh2D::new(target)
    }

    fn project_vision_point(&self,point: &Point3D) -> Point2D{
        let dz = self.pos.z / point.z;
        Point2D::new(point.x * dz,point.y * dz)
    }
}