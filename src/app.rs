extern crate sdl2;

use rand::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum::RGB24;
use sdl2::surface::Surface;
//use std::time::Duration;

pub fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-nes", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump()?;

    /*canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();*/
    let mut rng = rand::thread_rng();

    //800x600, 3 bytes per pixel
    //800*3 = 2400 bytes per row
    let mut buffer: Vec<u8> = vec![200; 1_440_000];

    //let surface = Surface::new(800, 600, PixelFormatEnum::RGB24).unwrap();
    let surface = Surface::from_data(&mut buffer, 800, 600, 0, RGB24)?;
    let texture = texture_creator
        .create_texture_from_surface(surface)
        .unwrap();

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

        /*canvas.set_dra799w_color(Color::RGB(
            rng.gen_range(0..u8::MAX),
            rng.gen_range(0..u8::MAX),
            rng.gen_range(0..u8::MAX),
        ));
        canvas.clear();*/

        for _i in 0..100 {
            let x = rng.gen_range(0..800);
            //let x = 800;
            //let y = rng.gen_range(0..600);
            let y = 598;
            let newR = rng.gen_range(0..u8::MAX);
            let newG = rng.gen_range(0..u8::MAX);
            let newB = rng.gen_range(0..u8::MAX);

            //2400 bytes per row of pixels
            let final_index = (y * 2400) + x;
            buffer[final_index] = newR;
            buffer[final_index + 1] = newG;
            buffer[final_index + 2] = newB;
        }
        /*let new_r = rng.gen_range(0..u8::MAX);
        let new_g = rng.gen_range(0..u8::MAX);
        let new_b = rng.gen_range(0..u8::MAX);
        let bottom_right = 595 * 2400 + 797;
        buffer[bottom_right] = new_r;
        buffer[bottom_right + 1] = new_g;
        buffer[bottom_right + 2] = new_b;*/

        //800x600 24 bits per pixel
        let surface = Surface::from_data(&mut buffer, 800, 600, 800, RGB24)?;
        let texture = texture_creator
            .create_texture_from_surface(surface)
            .unwrap();

        canvas.copy(&texture, None, None)?;

        canvas.present();
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}
