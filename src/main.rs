//need this for carrying add function in ADC
#![feature(bigint_helper_methods)]

use std::fs;

use pretty_assertions::{assert_eq, assert_ne};

mod cart;
mod cpu;
mod instr;
mod my_views;
mod nes;
mod tui;

use cart::Cart;
use cpu::Cpu;
use nes::NES;

use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

use std::io::Write;

fn main() {
    //loading our log
    let good_log = "./test-roms/nestest-redux/nestest_cpu_relined.log";
    let log_file = fs::read_to_string(good_log).expect("log file not found");
    let mut log = log_file
        .split("\n")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    //we dont need to check initial state
    log.remove(0);
    //reverse the log so we can pop values from it
    log = log.into_iter().rev().collect();

    //load our rom
    let filename = "./test-roms/nestest/nestest.nes";
    let rom_file = fs::read(filename).expect("file not found!");

    //make our cpu :D
    let mut cpu = Cpu::new();
    //make our "cart"
    let cart = Cart::new(rom_file[0x10..=0x400f].to_vec());
    //set PC to 0xc000
    cpu.PC = 0xc000;

    //make our full system
    let mut nes = NES::new(cpu, cart);

    ///////////////////////////////////////////////////////////////////////////////////////////////
    let mut tui = crate::tui::setup_tui(&mut nes);

    ///////////////////////////////////////////////////////////////////////////////////////////////
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    thread::spawn(move || nes.run(tx, log));

    let mut error = (String::new(), String::new());
    'running: loop {
        let mut step_running = false;
        tui.with_user_data(|s: &mut crate::tui::AppState| {
            if s.is_running {
                step_running = true;
            }
        });

        //only actually do stuff if we are currently running
        if step_running {
            //get all pending log lines and append them to the buffer view
            let mut pending: Vec<String> = rx.try_iter().collect();
            /*let mut good: Vec<String> = Vec::new();
            let mut all_good = true;
            for l in pending {
                let good_line = log.pop();
                all_good = good_line.unwrap().eq(&l);
                good.push(l.to_string());
                //stop processing
                if !all_good {
                    error = (good_line.unwrap().to_string(), l.to_string());
                    break;
                }
            }*/

            tui.call_on_name("log", |view: &mut crate::my_views::BufferView| {
                view.update(&mut pending)
            });

            /*if !all_good {
                tui.with_user_data(|s: &mut crate::tui::AppState| {
                    step_running = false;
                });
            }*/
        }

        let _tui_event_received = tui.step();
        tui.refresh();
    }

    tui.quit();

    /*let mut outlog = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("./errorlog.log")
        .expect("couldnt open");
    outlog
        .write(format!("good: {}\nbad:{}", error.0, error.1).as_bytes())
        .expect("couldnt write");
    outlog.flush().expect("couldnt flush");*/

    /*fs::write(
        "./errorlog.log",
        format!("good: {}\nbad:{}", error.0, error.1),
    )
    .expect("Unable to write file");*/
    //assert_eq!(error.1, error.0);
}
