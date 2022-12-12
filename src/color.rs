use std::{num::ParseIntError, ops::Mul};

#[derive(Debug,Clone,Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {

    pub const fn new(r:u8,g:u8,b:u8,a:u8)->Self{
        Self{r,g,b,a}
    }

    pub fn decode(s: &str) -> Result<Self,ParseIntError> {
        let s = u32::from_str_radix(s, 16)?;
        let b = u32::to_be_bytes(s);
        Ok(Self::from_bytes(b))
    }

    pub fn from_bytes(j: [u8;4])->Self{
        return Self{r: j[0],g:j[1],b: j[2],a: j[3]}
    }

    pub fn to_bytes(&self)->[u8;4]{
        return [self.r,self.g,self.b,self.a];
    }


    pub fn as_u32(&self) -> u32 {
        return u32::from_ne_bytes([self.r, self.g, self.b, self.a]);
    }

    pub fn from_u32(c: u32) -> Self {
        return Self::from_bytes(c.to_ne_bytes());
    }

}