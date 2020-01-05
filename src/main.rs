#[macro_use]
extern crate glium;
extern crate hex;

use glium::Surface;
use std::env;
use std::fs;
use std::io;
use std::str;

type Memory = [u8; 0x1000];
type Register = u8;
type AddressRegister = u16;
type Stack = [u8; 48];

fn main() {
    let mut memory: Memory = [0; 0x1000];
    let mut V0: Register = 0;
    let mut V1: Register = 0;
    let mut V2: Register = 0;
    let mut V3: Register = 0;
    let mut V4: Register = 0;
    let mut V5: Register = 0;
    let mut V6: Register = 0;
    let mut V7: Register = 0;
    let mut V8: Register = 0;
    let mut V9: Register = 0;
    let mut VA: Register = 0;
    let mut VB: Register = 0;
    let mut VC: Register = 0;
    let mut VD: Register = 0;
    let mut VE: Register = 0;
    let mut VF: Register = 0;
    let mut I: AddressRegister = 0;
    let mut stack: Stack = [0; 48];
    let mut delay_timer: u64 = 0;
    let mut sound_timer: u64 = 0;

    let mut program = fs::read("./example-programs/picture.ch8")
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

        match instructions[0] {
            '0' => print!("CLS, RET, SYS addr; "),
            '1' => print!("JP addr; "),
            '2' => print!("CALL addr; "),
            '3' => print!("SE Vx, byte; "),
            '4' => print!("SNE Vx, byte; "),
            '5' => print!("SE Vx, Vy; "),
            '6' => print!("LD Vx, byte; "),
            '7' => print!("ADD Vx, byte; "),
            '8' => print!("Main Meat; "),
            '9' => print!("SNE Vx, Vy; "),
            'a' => print!("LD I, addr; "),
            'b' => print!("JP V0, addr; "),
            'c' => print!("RND Vx, byte; "),
            'd' => print!("DRW Vx, Vy, nibble; "),
            'e' => print!("SKP Vx, SKNP Vx; "),
            'f' => print!("Second Main Meat; "),
            _ => panic!("Not a valid OpCode!"),
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
