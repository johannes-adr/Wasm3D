use std::{fmt::Debug, num::ParseIntError};

use dimension::{
    mesh::{Mesh3D, Triangle2D, Triangle3D},
    viewpoint::ViewPoint,
};
use line_drawing::Bresenham;
use wasm_bindgen::prelude::*;

extern crate wasm_bindgen;
mod gamewindow;
use gamewindow::GameWindow;

mod Pixel;
use Pixel::PixelCoord;

mod color;
use color::Color;

use crate::dimension::Point::Point2D;

mod dimension;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn alert(s: &str);
}

pub fn print<T: ToString>(t: T) {
    log(t.to_string().as_str())
}

pub fn print2<T: Debug>(t: T) {
    print(format!("{:?}", t));
}

static mut f: f32 = 0.0;

#[wasm_bindgen]
pub fn render(window: &mut GameWindow) {
    window.clear();
    let f2 = unsafe { &mut f };
    *f2 = *f2 + 0.02;
    let c1: Color = Color::decode("EEEEEEFF").unwrap();
    let black: Color = Color::decode("000000ff").unwrap();

    let mut pi = 0.0;
    let w2 = (window.width as f32 / 2.0);
    let h2 = (window.height as f32 / 2.0);

    let view_point = ViewPoint::default();
    /*let mut mesh = Mesh3D::cube();
    mesh.rotateY(*f2);
    mesh.rotateX(*f2);
    mesh.rotateZ(*f2);*/
    let mut mesh = Mesh3D::new(vec![Triangle3D::from_array_rawi([
        [-1, -1, 1],
        [1, 1, 1],
        [-1, 1, 1],
    ])]);

    /*for t in view_point.project_vision(&mesh).vertecies {
        let v = t.vertecies;
        window.drawLine(v[0], v[1], black);
        window.drawLine(v[0], v[2], black);
        window.drawLine(v[1], v[2], black);
    }*/

    for t in view_point.project_vision(&mesh).vertecies {
        fill_triangle(window, t, black.clone());
    }
}

fn fill_triangle(window: &mut GameWindow, t: Triangle2D, color: Color) {
    

    
    let mut v = t.vertecies;

    


    fill_peak_top(window, v, color);
    
    fn fill_peak_top(window: &mut GameWindow, mut v: [Point2D;3], color: Color) {
        //Get index of highest point and put at 0
        let mut max_i = 0;
        for i in 1..3 {
            if v[i].y < v[max_i].y {
                max_i = i
            }
        }
        v.swap(max_i, 0);

        if v[2].x > v[1].x {
            v.swap(1, 2)
        }

        for t in &mut v {
            t.x += window.pre.widthF2;
            t.y += window.pre.heightF2;
        }
        fill_between_lines(window, v, color);
    }


    fn fill_between_lines(window: &mut GameWindow, v: [Point2D; 3], color: Color) {
        //creating line v0-v2 and v0-v1
        let mut l2 = Bresenham::new(v[0].tuplei32(), v[2].tuplei32()).into_iter(); //Line on left
        let mut l1 = Bresenham::new(v[0].tuplei32(), v[1].tuplei32()).into_iter(); //Line on right

        let mut y2 = l2.next().unwrap();
        let mut y1 = l1.next().unwrap();
        let mut curry = y2.1;
        loop {
            for x in y2.0..y1.0 {
                window.drawPoint(x as usize, curry as usize, color);
            }

            let mut l2done = false;
            let mut l1done = false;

            'l2loop: while let Some(p) = l2.next() {
                if p.1 != curry {
                    y2 = p;
                    l2done = true;
                    break 'l2loop;
                }
            }

            'l1loop: while let Some(p) = l1.next() {
                if p.1 != curry {
                    y1 = p;
                    l1done = true;
                    break 'l1loop;
                }
            }
            curry = y2.1;
            if !(l2done && l1done) {
                break;
            }
        }
    }
}
