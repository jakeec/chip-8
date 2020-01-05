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

type Memory = [u8; 0x1000];
type Register = u8;
type AddressRegister = u16;
type Stack = [u8; 48];

fn CLS(instructions: &Vec<char>) {
    println!("Called CLS with parameters: {:?}", instructions);
}

fn RET(instructions: &Vec<char>) {
    println!("Called RET with parameters: {:?}", instructions);
}

fn JP(instructions: &Vec<char>) {
    println!("Called JP with parameters: {:?}", instructions);
}

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
    let mut registers: Vec<u8> = Vec::new();
    registers.push(V0);
    registers.push(V1);
    registers.push(V2);
    registers.push(V3);
    registers.push(V4);
    registers.push(V5);
    registers.push(V6);
    registers.push(V7);
    registers.push(V8);
    registers.push(V9);
    registers.push(VA);
    registers.push(VB);
    registers.push(VC);
    registers.push(VD);
    registers.push(VE);
    registers.push(VF);
    let mut I: AddressRegister = 0;
    let mut stack: Stack = [0; 48];
    let mut delay_timer: u64 = 0;
    let mut sound_timer: u64 = 0;
    let mut program_counter: u64 = 0;
    let mut stack_counter: u64 = 0;

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

        match instructions[0] {
            '0' => CLS(&instructions),
            '1' => JP(&instructions),
            '2' => JP(&instructions),
            '3' => println!("SE Vx, byte; "),
            '4' => println!("SNE Vx, byte; "),
            '5' => println!("SE Vx, Vy; "),
            '6' => println!("LD Vx, byte; "),
            '7' => println!("ADD Vx, byte; "),
            '8' => RegisterOperation(&instructions),
            '9' => println!("SNE Vx, Vy; "),
            'a' => println!("LD I, addr; "),
            'b' => println!("JP V0, addr; "),
            'c' => {
                let mut rng = rand::thread_rng();
                let mut rnd: u8 = rng.gen_range(0, 255);
                let mut a = String::from("0");
                let mut b = String::from("0");
                a.push(instructions[2]);
                b.push(instructions[3]);
                let kk = format!("{}{}", instructions[2], instructions[3]);
                let x = hex::decode(&kk).unwrap();
                let result = (rnd & x[0]) as u8;
                match instructions[1] {
                    '0' => V0 = result,
                    '1' => V1 = result,
                    '2' => V2 = result,
                    '3' => V3 = result,
                    '4' => V4 = result,
                    '5' => V5 = result,
                    '6' => V6 = result,
                    '7' => V7 = result,
                    '8' => V8 = result,
                    '9' => V9 = result,
                    'a' => VA = result,
                    'b' => VB = result,
                    'c' => VC = result,
                    'd' => VD = result,
                    'e' => VE = result,
                    'f' => VF = result,
                    _ => panic!("Register V{} does not exist!", instructions[1]),
                }
            }
            'd' => println!("DRW Vx, Vy, nibble; "),
            'e' => println!("SKP Vx, SKNP Vx; "),
            'f' => IOOperation(&instructions, &memory, &registers),
            _ => panic!("Not a valid OpCode!"),
        }
    }

    println!("V0 value was set to {}", V0);

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
