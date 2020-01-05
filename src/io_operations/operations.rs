use std::char;
use std::fmt::Binary;
use std::str;

// Store BCD representation of Vx in memory locations I, I+1, and I+2.
// The interpreter takes the decimal value of Vx, and places the hundreds digit
// in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
fn fn33(x: &char) {}

pub fn IOOperation<'a>(instructions: &Vec<char>, memory: &[u8; 0x1000], registers: &Vec<u8>) {
    let rand = instructions[1];
    let mut rator = instructions[2].to_string();
    rator.push(instructions[3]);
    match &*rator {
        "07" => println!("07!"),
        "0a" => println!("OA!"),
        "15" => println!("15!"),
        "18" => println!("18!"),
        "1e" => println!("1e!"),
        "29" => println!("29!"),
        "33" => fn33(&rand),
        "55" => println!("55!"),
        "65" => {
            println!("65!");
            println!("RAND: {}", rand);
        }
        _ => println!("Operation not found {}", rator),
    }
}
