use quicksilver::{
    Result,
    geom::{Circle, Rectangle, Transform, Vector},
    input::{Key},
    graphics::{Background::Col, Color},
    lifecycle::{Settings, State, Window, run},
};

pub fn angleToDirectionVector(angle:f64) ->(f64, f64){
    let mut x = -1.0*angle.sin();
    //if angle > 0.0{
    //    x = x*(-1.0);
    //}
    let y = angle.cos();

    (x,y)

}


pub fn distance_between_coords(v1:Vector, v2:Vector)->(f64){
    let (dx, dy) = (v1.x-v2.x, v1.y-v2.y);
    let a_squared_plus_b_squared = dx*dx + dy*dy;
    a_squared_plus_b_squared.sqrt() as f64

}