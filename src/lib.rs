use std::{fmt::Debug, num::ParseIntError};

use dimension::{
    mesh::{Mesh3D, Triangle2D, Triangle3D},
    viewpoint::ViewPoint, Point::Float,
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
    let mut mesh = Mesh3D::cube();
    //let val = 0.6;
    mesh.rotateY(*f2);
    mesh.rotateX(*f2);
    mesh.rotateZ(*f2);
    let mut mesh2 = Mesh3D::new(vec![Triangle3D::from_array_rawi([
        [1, 1, 1],
        [1, 3, 1],
        [-1, 2, 1],
    ])]);

    /*for t in view_point.project_vision(&mesh).vertecies {
        let v = t.vertecies;
        window.drawLine(v[0], v[1], black);
        window.drawLine(v[0], v[2], black);
        window.drawLine(v[1], v[2], black);
    }*/
    let mut c = 0;
    let tranformed = view_point.project_vision(&mesh);
    let colored = tranformed.vertecies.iter().map(|vert| {
        c += 20;
       // return (*vert, Color::new(0, 0, 0, 255))
       //backface culling
        return (*vert, Color::new(c, c, c, 255));
    });

    for ((triangle,z_index), col) in colored {
        fill_triangle(window, triangle, col, z_index);
    }
}

fn fill_triangle(window: &mut GameWindow, t: Triangle2D, color: Color,z_index: Float) {
    //http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html

    let mut v = t.vertecies;

    let peak;
    if v[1].y == v[0].y {
        if v[1].y == v[2].y {
            //All are on same height
            return;
        }
        peak = 2;
    } else if v[2].y == v[0].y {
        peak = 1;
    } else if v[1].y == v[2].y {
        peak = 0
    } else {
        //Split triangle
        v.sort_unstable_by(|p1, p2| p1.y.total_cmp(&p2.y));
        let mut is_left = false;
        if v[1].x < v[2].x {
            is_left = true;
        }
        //x4 = x1 + ((y2 - y1) / (v3 -y1)) * (x3-x1)
        let v4 = (
            v[0].x + ((v[1].y - v[0].y) / (v[2].y - v[0].y)) * (v[2].x - v[0].x),
            v[1].y,
        );
        let v4 = Point2D::new(v4.0, v4.1);
        if is_left {
            fill_peak(window, [v[0], v4, v[1]], color,z_index);
            fill_peak(window, [v[2], v4, v[1]], color,z_index);
        } else {
            fill_peak(window, [v[0], v[1], v4], color,z_index);
            fill_peak(window, [v[2], v[1], v4], color,z_index);
        }

        return;
    }
    v.swap(peak, 0);
    if v[2].x > v[1].x {
        v.swap(1, 2)
    }
    //Peak is at pos 0, v[2] is left, v[1] is right, v[2].y == v[1].y
    //If v0.y is bigger => peak at bottom
    fill_peak(window, v, color,z_index);

    fn fill_peak(window: &mut GameWindow, mut v: [Point2D; 3], color: Color,z_index: Float) {
        for t in &mut v {
            t.x += window.pre.widthF2;
            t.y += window.pre.heightF2;
        }
        fill_between_lines(window, v, color,z_index);
    }

    fn fill_between_lines(window: &mut GameWindow, v: [Point2D; 3], color: Color,z_index: Float) {
        //creating line v0-v2 and v0-v1
        let mut l2 = Bresenham::new(v[0].tuplei32(), v[2].tuplei32()).into_iter(); //Line on left
        let mut l1 = Bresenham::new(v[0].tuplei32(), v[1].tuplei32()).into_iter(); //Line on right

        let mut y2 = l2.next().unwrap();
        let mut y1 = l1.next().unwrap();
        let mut curry = y2.1;
        loop {
            for x in y2.0 - 1..=y1.0 {
                window.drawPoint_z(x as usize, curry as usize,z_index, color);
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
