use std::num::ParseIntError;

use line_drawing::{Bresenham, XiaolinWu};
use wasm_bindgen::prelude::wasm_bindgen;
//Bresenham's line algorithm

//Xialin Wu's line algorithm
use crate::{
    color::Color,
    print,
    Pixel::{number, PixelCoord},
    Point::{Float, Point2D},
};

/**
 * (smaller, bigger)
 */
fn min_max<T: std::cmp::PartialOrd>(t1: T, t2: T) -> (T, T) {
    if t1 < t2 {
        return (t1, t2);
    }
    return (t2, t1);
}

/**
 * (smaller, bigger)
 */
fn sortPoint(t1: Point2D, t2: Point2D) -> (Point2D, Point2D) {
    if t1.x < t2.x {
        return (t1, t2);
    }
    return (t2, t1);
}

#[wasm_bindgen]
pub struct GameWindow {
    pub width: number,
    pub height: number,
    buffer: Box<[u32]>,
    pre: GameWindowChache,
}

struct GameWindowChache {
    widthF: Float,
    heightF: Float,
    widthF2: Float,
    heightF2: Float,
}

#[wasm_bindgen]
impl GameWindow {
    pub fn new(width: number, height: number) -> Self {
        Self {
            buffer: vec![0; (width * height) as usize].into_boxed_slice(),
            width,
            height,
            pre: GameWindowChache {
                widthF: width as Float,
                heightF: height as Float,
                widthF2: width as Float / 2.0,
                heightF2: height as Float / 2.0,
            },
        }
    }

    pub fn bufferPtr(&self) -> *const u32 {
        return self.buffer.as_ptr();
    }
}

impl GameWindow {
    /*pub fn setPixel(&mut self, p: PixelCoord, c: Color) {
        self.buffer[(self.width * p.y + p.x) as usize] = c.as_u32();
    }*/

    pub fn getBuffer(&self) -> &Box<[u32]> {
        &self.buffer
    }

    pub fn clear(&mut self){
        for elem in self.buffer.iter_mut() { *elem = 0; }
    }

    pub fn fillRect(&mut self, p1: PixelCoord, p2: PixelCoord, c: Color) {
        p1.in_bounds_panic(self);
        p2.in_bounds_panic(self);

        for x in p1.x..p2.x {
            for y in p1.y..p2.y {
                todo!()
                //self.setPixel(PixelCoord::new(x, y), c)
            }
        }
    }

    pub fn drawLine(&mut self, mut p1: Point2D, mut p2: Point2D, c: Color) {
        p1.x += self.pre.widthF2;
        p1.y += self.pre.heightF2;
        p2.x += self.pre.widthF2;
        p2.y += self.pre.heightF2;
        if true{
            let p1 = (p1.x as i32, p1.y as i32);
            let p2 =(p2.x as i32, p2.y as i32);
            for (x,y) in Bresenham::new(p1,p2){
                if x < 0 || y < 0 || y >= self.height as i32 || x >= self.width as i32{
                    continue;
                }
                self.drawPoint(x as usize, y as usize, c)
            }
        }else{
            for ((x, y), value) in XiaolinWu::<Float, i32>::new(p1.tuple(), p2.tuple()) {
                //c.a = (c.a as f32 * value) as u8;
                if x < 0 || y < 0 || y >= self.height as i32 || x >= self.width as i32{
                    continue;
                }
                self.drawPoint(x as usize, y as usize, c);
                
            }
        }
    }

    fn drawPoint(&mut self, x: usize, y: usize, color: Color) {
        self.buffer[self.width * y + x] = color.as_u32();
    }
    //https://gist.githubusercontent.com/gszauer/5708246/raw/04467d694981db77f0e895e58d35e72c8867d319/Alpha%2520Blending%2520Tutorial
    fn drawPointAlpha(&mut self, x: usize, y: usize, color: Color) {
        let pos = &mut self.buffer[self.width * y + x];
        *pos = color.as_u32();
    }

}

/*


  (p1, p2) = sortPoint(p1, p2);
        let b = (p2.y - p1.y) / (p2.x - p1.x);
/*
        //Reposition p1
        if p1.x < 0.0 {
            if p2.x < 0.0 {
                return;
            }
            p1.y += (0.0 - p1.x) * b;
            p1.x = 0.0;
        }

        //Reposition p2
        if p2.x > self.pre.widthF {
            p2.y -= (p2.x - self.pre.widthF) * b;
            p2.x = self.pre.widthF;
        }

        //print(format!("x: {:?}, y: {:?}, b: {}",x,y,b));
        */
        //Draw by x coords
        {
            let mut y = p1.y;
            let mut x = p1.x as usize as f32;
            while x <= p2.x && y < self.pre.heightF{
                self.drawPoint(x as usize, y as usize, c);
                y += b;
                x += 1.0;
            }
        }
        //Draw by y coords
        {
            let mut y = p1.y as usize as f32;
            let mut x = p1.x;
            while y < p2.y && x < self.pre.widthF{
                self.drawPoint(x as usize, y as usize, c);
                //x += 1.0/b;
                y += 1.0;
            }
        }
*/
