

extern crate quicksilver;
extern crate image;

use quicksilver::{
    Result,
    geom::{Shape, Vector},
    graphics::{Background::Img, Color, Image},
    lifecycle::{Asset, Settings, State, Window, run},
};
//use std::io::Result;
use std::net::UdpSocket;

use ::{Game};


pub struct JanUpdateStruct {
    pub test: u8,
    socket:UdpSocket,
    asset:Asset<Image>,
}

//Spaghetticode: unwrap() several times instead of proper result handling

pub fn init() ->JanUpdateStruct{
    let mut bind_socket :UdpSocket = UdpSocket::bind("127.0.0.1:34254").unwrap();
    //TODO: include errorhandling for bind (e.g. loop while bind unsuccessful?)


    //testing purposes:

    bind_socket.send_to(&[1; 10], "127.0.0.1:34254");
    let asset = Asset::new(Image::load("./image.png"));
    //return:
    JanUpdateStruct{
        test:0,
        socket : bind_socket,
        asset,
    }

}

impl State for JanUpdateStruct {
    fn new() -> Result<JanUpdateStruct> {
        Ok(init())
    }


     fn draw(&mut self, _window: &mut Window) ->Result<()> {
        _window.clear(Color::WHITE)?;
        self.asset.execute(|image| {
            _window.draw(&image.area().with_center((400,300)), Img(&image));
            Ok(())
        })
    }
}