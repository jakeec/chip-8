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

fn display(buffer: [[u8; 64]; 32]) {
    let mut vram = String::from("");
    for column in &buffer {
        for row in column.iter() {
            match row {
                0 => vram.push(' '),
                1 => vram.push('*'),
                _ => panic!("Invalid vram"),
            }
        }
        vram.push('\n');
    }

    println!("{}", vram);
}

fn main() {
    let mut memory: [usize; 0x1000] = [0; 0x1000];
    let mut registers: [u8; 16] = [0; 16];
    let mut I: usize = 0;
    let mut stack: [u8; 48] = [0; 48];
    let mut delay_timer: usize = 0;
    let mut sound_timer: usize = 0;
    let mut program_counter: usize = 0;
    let mut stack_counter: usize = 0;
    let mut display_buffer_front: [[u8; 64]; 32] = [[0; 64]; 32];
    let mut sprites: [[u8; 5]; 16] = [[0; 5]; 16];
    let sprite0 = [0xF0, 0x90, 0x90, 0x90, 0xF0];
    let sprite1 = [
        0x20,
        0x60,
        0x20,
        0x20,
        0x70,
    ];
    let sprite2 = [
        0xF0,
        0x10,
        0xF0,
        0x80,
        0xF0,
    ];
    let sprite3 = [
        0xF0,
        0x10,
        0xF0,
        0x10,
        0xF0,
    ];
    let sprite4 = [
        0x90,
        0x90,
        0xF0,
        0x10,
        0x10,
    ];
    let sprite5 = [
        0xF0,
        0x80,
        0xF0,
        0x10,
        0xF0,
    ];
    let sprite6 = [
        0xF0,
        0x80,
        0xF0,
        0x90,
        0xF0,
    ];
    let sprite7 = [
        0xF0,
        0x10,
        0x20,
        0x40,
        0x40,
    ];
    let sprite8 = [
        0xF0,
        0x90,
        0xF0,
        0x90,
        0xF0,
    ];
    let sprite9 = [
        0xF0,
        0x90,
        0xF0,
        0x10,
        0xF0,
    ];
    let spriteA = [
        0xF0,
        0x90,
        0xF0,
        0x90,
        0x90,
    ];
    let spriteB = [
        0xE0,
        0x90,
        0xE0,
        0x90,
        0xE0,
    ];
    let spriteC = [
        0xF0,
        0x80,
        0x80,
        0x80,
        0xF0,
    ];
    let spriteD = [
        0xE0,
        0x90,
        0x90,
        0x90,
        0xE0,
    ];
    let spriteE = [
        0xF0,
        0x80,
        0xF0,
        0x80,
        0xF0,
    ];
    let spriteF = [
        0xF0,
        0x80,
        0xF0,
        0x80,
        0x80,
    ];
    sprites[0x00] = sprite0;
    sprites[0x01] = sprite1;
    sprites[0x02] = sprite2;
    sprites[0x03] = sprite3;
    sprites[0x04] = sprite4;
    sprites[0x05] = sprite5;
    sprites[0x06] = sprite6;
    sprites[0x07] = sprite7;
    sprites[0x08] = sprite8;
    sprites[0x09] = sprite9;
    sprites[0x0A] = spriteA;
    sprites[0x0B] = spriteB;
    sprites[0x0C] = spriteC;
    sprites[0x0D] = spriteD;
    sprites[0x0E] = spriteE;
    sprites[0x0F] = spriteF;

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
            (0x04, _, _, _) => {
                if registers[x] != kk {
                    program_counter += 2;
                }
            },
            (0x05, _, _, 0x00) => {
                if registers[x] == registers[y] {
                    program_counter += 2;
                }
            }
            (0x06, _, _, _) => {
                registers[x] = kk;
            },
            (0x07, _, _, _) => {
                registers[x] = registers[x] + kk;
            },
            (0x08, _, _, 0x00) => {
                registers[x] = registers[y];
            },
            (0x08, _, _, 0x01) => {
                registers[x] = registers[x] | registers[y];
            },
            (0x08, _, _, 0x02) => {
                registers[x] = registers[x] & registers[y];
            },
            (0x08, _, _, 0x03) => {
                registers[x] = registers[x] ^ registers[y];
            },
            (0x08, _, _, 0x04) => {
                if registers[x] + registers[y] > 255 { 
                    registers[0x0F] = 1;
                } else {
                    registers[0x0F] = 0;
                }
                registers[x] = registers[x] + registers[y] as u8;
            },
            (0x08, _, _, 0x05) => {
                if registers[x] > registers[y] { 
                    registers[0x0F] = 1;
                    registers[x] = registers[x] - registers[y] as u8;
                } else {
                    registers[0x0F] = 0;
                }
            },
            (0x08, _, _, 0x06) => {
                registers[0x0F] = registers[x] & 0x000F;
                registers[x] = registers[x] / 2;
            },
            (0x08, _, _, 0x07) => {
                if registers[y] > registers[x] {
                    registers[0x0F] = 1;
                } else {
                    registers[0x0F] = 0;
                }

                registers[x] = registers[y] - registers[x];
            }, 
            (0x08, _, _, 0x0E) => {
                registers[0x0F] = registers[x] & 0x08;
                registers[x] = registers[x] * 2;
            },
            (0x09, _, _, 0) => {
                if registers[x] != registers[7] {
                    program_counter += 2;
                }
            },
            (0x0A, _, _, _) => {
                I = nnn;
            },
            (0x0D, _, _, _) => {
                let bytes = &memory[I..I+n];
                println!("{:?}", bytes);
                let mut xr = x;
                let mut yr = y;
                let mut xoffset = 0;
                for byte in bytes {
                    let sprite = sprites[*byte];
                    for row in &sprite {
                        let r = format!("{:b}", row);
                        let r = format!("{:0>8}", r);
                        let pixels: Vec<_> = r.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
                        for pixel in pixels {
                            display_buffer_front[yr][xr] = pixel as u8;
                            // increment pixel x axis
                            xr = xr + 1;
                        }
                        // move down to next row
                        yr = yr + 1;
                        // reset x axis to start of sprite
                        xr = xoffset;
                    }
                    // reset y axis
                    yr = y;
                    // move along to next sprite position
                    xoffset = xoffset + 8;
                }
                display(display_buffer_front);
            },
            (0x0F, _, 0x03, 0x03) => {
                let digits: Vec<_> = registers[x].to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
                match digits.as_slice() {
                    [h, t, o] => {
                        memory[I] = *h as usize;
                        memory[I+1] = *t as usize;
                        memory[I+2] = *o as usize;
                    },
                    [t, o] => {
                        memory[I+1] = *t as usize;
                        memory[I+2] = *o as usize;
                    }, 
                    [o] => {
                        memory[I+2] = *o as usize;
                    }
                    _ => panic!("This should never happen!"),
                } 
            },
            (0x0F, _, 0x06, 0x05) => {
                for i in 0..x {
                    registers[i] = memory[I+i] as u8;
                }
            },
            (0x0F, _, 0x02, 0x09) => {
                I = registers[x] as usize;
            }
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
