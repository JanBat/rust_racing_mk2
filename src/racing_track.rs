extern crate quicksilver;



use quicksilver::{
    Result,
    geom::{Circle, Rectangle, Transform, Vector},
    input::{Key},
    graphics::{Background::Col, Color},
    lifecycle::{Settings, State, Window, run},
};
use std::f64::consts::PI;
use ::{Game};

use mathhelper;

//representing a single RacingCar
pub  struct RacingTrackUpdateStruct {
    checkpoints:  Vec<(f64, f64, bool)>,
 }

pub fn init() ->RacingTrackUpdateStruct{
    let mut V = Vec::new();
    V.push((200.0,100.0,true));  //checkpoint 1
    V.push((200.0,200.0,false));  //checkpoint 2
    V.push((200.0,300.0,false));  //checkpoint 3
    V.push((200.0,400.0,false));  //checkpoint 4
    V.push((200.0,500.0,false));  //checkpoint 5
    V.push((200.0,600.0,false));  //checkpoint 6



    RacingTrackUpdateStruct{
        checkpoints: V,

    }
}
impl State for RacingTrackUpdateStruct {

    fn new() -> Result<RacingTrackUpdateStruct> {
        let mut newTrack = init();

        //newTrack.checkpoints.push(Vector::new(200, 200));
        //println!("checkpoints:{:?}", newTrack.checkpoints);
        Ok(newTrack)
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        //self.checkpoints.push(Vector::new(200, 200));
        //println!("checkpoints:{:?}", self.checkpoints);
        
        Ok(())
    }

    fn draw(&mut self, _window: &mut Window) -> Result<()>{


        for i in 0..self.checkpoints.len(){
            let vec = self.checkpoints[i];
            let mut col = Col(Color::GREEN);
            //deactivated checkpoints: red
            if vec.2 == false{
                col = Col(Color::RED);
            }
            _window.draw_ex(&Circle::new((vec.0 as u32, vec.1 as u32), 10),col , Transform::rotate(45), 50);
        }

        Ok(())
    }
}
