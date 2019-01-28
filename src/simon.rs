extern crate quicksilver;
extern crate image;

use quicksilver::{
    Result,
    geom::{Rectangle, Transform, Shape, Vector},
    graphics::{Background::Col, Color, Image, Background::Img},
    lifecycle::{Window, Asset, State}
};

use ::{Game};

pub struct SimonUpdateStruct {
    pub x_pos: i32,
    pub y_pos: i32,
    asset: Asset<Image>
}

pub fn init() ->SimonUpdateStruct{
    SimonUpdateStruct{x_pos: 0, y_pos:0, asset: Asset::new(Image::load("./image.png"))}
}

impl State for SimonUpdateStruct{

    fn new() -> Result<SimonUpdateStruct> {
        Ok(init())
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, _window: &mut Window) -> Result<()> {
        //_window.clear(Color::WHITE)?;
         self.asset.execute(|image| {
            _window.draw(&image.area().with_center((400, 300)), Img(&image));
            Ok(())
        })
        //_window.draw_ex(&Rectangle::new((100, 100), (32, 32)), Col(Color::BLUE), Transform::rotate(45), 0);
        //Ok(()); //Ok ok?
    }

}