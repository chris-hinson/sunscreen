extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum::RGB24;
use sdl2::surface::Surface;
use std::sync::mpsc::Receiver;

pub fn run(channel: Receiver<Vec<u8>>) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-nes", 256, 240)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump()?;

    //let mut rng = rand::thread_rng();
    //256x240, 3 bytes per pixel
    //this is 180kb .... probably too much to put on the stack
    //box it?
    //let mut buffer: Vec<u8> = vec![200; 184_320];

    //this is a loop that does nothing but try to receive a new frame
    //from the ppu continuously, and when it does, turns it into a texture
    //and displays it.
    //TODO: does this need to be changed to be more in line with beam state?
    //perhaps a mutexed array for a buffer so that both threads can access it
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        match channel.try_recv() {
            Ok(mut frame) => {
                //256x240 24 bits per pixel
                let surface = Surface::from_data(&mut frame, 256, 240, 768, RGB24)?;
                let texture = texture_creator
                    .create_texture_from_surface(surface)
                    .unwrap();
                //TODO: if we resize our window, this should handle resizing our
                //texture automagically
                canvas.copy(&texture, None, None)?;
                canvas.present();
            }
            Err(_e) => {
                //nothing to be done if new frame is not ready
            }
        }
    }

    Ok(())
}
