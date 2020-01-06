#[macro_use]
extern crate glium;
extern crate hex;
extern crate rand;

use glium::Surface;
use rand::Rng;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::str;

mod register_operations;
use register_operations::RegisterOperation;
mod io_operations;
use io_operations::IOOperation;

fn main() {
    let mut memory: [u8; 0x1000] = [0; 0x1000];
    let mut registers: [u8; 16] = [0; 16];
    let mut I: u16 = 0;
    let mut stack: [u8; 48] = [0; 48];
    let mut delay_timer: usize = 0;
    let mut sound_timer: usize = 0;
    let mut program_counter: usize = 0;
    let mut stack_counter: usize = 0;
    let mut display_buffer_front: [[u8; 64]; 32] = [[0; 64]; 32];

    let mut program = fs::read("./example-programs/randomnumber.ch8")
        .map(|o| hex::encode(o))
        .expect("Could not read file!");

    let opcodes = program
        .as_bytes()
        .chunks(4)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    println!("{:?}", opcodes);

    for opcode in opcodes {
        let intcode = opcode.chars().next().unwrap();
        let instructions: Vec<char> = opcode.chars().collect();
        let opcode = u16::from_str_radix(opcode, 16).unwrap();
        let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8
        );
        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;

        match nibbles {
            (0x00, 0x00, 0x0e, 0x00) => {
                display_buffer_front = [[0; 64]; 32];
            },
            (0x00, 0x00, 0x0e, 0x0e) => {
                let addr = stack[47];
                program_counter = addr as usize;
                stack_counter -= 1;
            },
            (0x01, _, _, _) => {
                program_counter = nnn;
            },
            (0x2, _, _, _) => {
                stack_counter += 1;
                stack[stack_counter] = program_counter as u8;
                program_counter = nnn;
            },
            (0x03, _, _, _) => {
                if registers[x] == kk {
                    program_counter += 2;
                }
            },
            (0x06, _, _, _) => println!("{} {}", x, kk),
            _ => println!("OpCode Not Found!"),
        }
    }

    // let mut events_loop = glium::glutin::EventsLoop::new();
    // let resolution = glium::glutin::dpi::LogicalSize::new(640.0, 320.0);
    // let wb = glium::glutin::WindowBuilder::new()
    //     .with_dimensions(resolution)
    //     .with_title("NES");
    // let cb = glium::glutin::ContextBuilder::new();
    // let display = glium::Display::new(wb, cb, &events_loop).unwrap();
    // let mut closed = false;
    // while !closed {
    //     events_loop.poll_events(|ev| match ev {
    //         glium::glutin::Event::WindowEvent { event, .. } => match event {
    //             glium::glutin::WindowEvent::CloseRequested => closed = true,
    //             _ => (),
    //         },
    //         _ => (),
    //     });

    //     let mut target = display.draw();
    //     target.clear_color(1.0, 0.0, 1.0, 1.0);
    //     target.finish().unwrap();
    // }
}
