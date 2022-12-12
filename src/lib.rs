use std::num::ParseIntError;

use viewpoint::ViewPoint;
use wasm_bindgen::prelude::*;

extern crate wasm_bindgen;
mod gamewindow;
use gamewindow::{GameWindow};

mod Pixel;
use Pixel::{PixelCoord};

mod color;
use color::Color;

mod Point;
use Point::Point2D;
mod mesh;
mod viewpoint;
use mesh::Mesh3D;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn alert(s: &str);
}


pub fn print<T: ToString>(t: T){
    log(t.to_string().as_str())
}

static mut f: f32 = 0.0;

#[wasm_bindgen]
pub fn render(window: &mut GameWindow) {
    window.clear();
    let f2 = unsafe{&mut f};
    *f2=*f2 + 0.01;
    let c1: Color = Color::decode("EEEEEEFF").unwrap();
    let black: Color = Color::decode("000000ff").unwrap();

    let mut pi = 0.0;
    let w2 = (window.width as f32 / 2.0);
    let h2 = (window.height as f32 / 2.0);

    
    
    let viewPoint = ViewPoint::default();
    let mut mesh = Mesh3D::cube();
    //mesh.rotateX(*f2);
    mesh.rotateZ(*f2);
    let mesh2d = viewPoint.project_vision(&mesh);
   // window.drawLine(PixelCoord::default(), window.size(), black);
    
    let v = mesh2d.vertecies;

    for i in 0..v.len(){
        let a = v[i];
        let b = v[if i+1==v.len(){0}else{i+1}];
        print(format!("a:{:?},b:{:?}",a,b));
        window.drawLine(a, b, black);
    }
}


/*


    while pi < 2.0*std::f32::consts::PI{
        pi+=0.0001;
        let (x,y) = (pi.cos(),pi.sin());
        let l = 4600.0;
        let p = Point2D::new(x*l + w2, y*l +  h2);
       // print(format!("{:?}",p));
        window.drawLine(p, Point2D::new(w2,h2), black);

    }

*/