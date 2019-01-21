

pub fn angleToDirectionVector(angle:f64) ->(f64, f64){
    let mut x = -1.0*angle.sin();
    //if angle > 0.0{
    //    x = x*(-1.0);
    //}
    let y = angle.cos();

    (x,y)

}