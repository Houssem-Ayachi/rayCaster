#![allow(nonstandard_style)]

pub struct Angle{
    pub value: f32
}

impl Angle{

    pub fn new(mut angle: f32) -> Self{
        if angle < 0.0 {
            angle += 360.0;
        }else{
            angle %= 360.0;
        }
        Angle { value: angle }
    }

    pub fn rotate(&mut self, angle: f32){
        self.value += angle;
        self.bind();
    }

    pub fn bind(&mut self){
        if self.value < 0.0 {
            self.value += 360.0;
        }else{
            self.value %= 360.0;
        }
    }

}