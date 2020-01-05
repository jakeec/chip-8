// Set Vx = Vy.
// Stores the value of register Vy in register Vx.
fn LD(x: &char, y: &char) {
    println!("\x1b[0;32mPutting value of V{} into V{}\x1b[0m", y, x);
}

// Set Vx = Vx OR Vy.
// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
// A bitwise OR compares the corrseponding bits from two values, and if either
// bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
fn OR(x: &char, y: &char) {
    println!("\x1b[0;32mSetting V{} = V{} OR V{}\x1b[0m", x, x, y);
}

// Set Vx = Vx AND Vy.
// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
// A bitwise AND compares the corrseponding bits from two values, and if both
// bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
fn AND(x: &char, y: &char) {
    println!("\x1b[0;32mSetting V{} = V{} AND V{}\x1b[0m", x, x, y);
}

// Set Vx = Vx XOR Vy.
// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the
// result in Vx. An exclusive OR compares the corrseponding bits from two values,
// and if the bits are not both the same, then the corresponding bit in the
// result is set to 1. Otherwise, it is 0.
fn XOR(x: &char, y: &char) {
    println!("\x1b[0;32mSetting V{} = V{} XOR V{}\x1b[0m", x, x, y);
}

// Set Vx = Vx + Vy, set VF = carry.
// The values of Vx and Vy are added together. If the result is greater than
// 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits
// of the result are kept, and stored in Vx.
fn ADD(x: &char, y: &char) {
    println!("\x1b[0;32mSetting V{} = V{} + V{}\x1b[0m", x, x, y);
}

// Set Vx = Vx - Vy, set VF = NOT borrow.
// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
fn SUB(x: &char, y: &char) {
    println!("\x1b[0;32mSetting V{} = V{} - V{}\x1b[0m", x, x, y);
}

// Set Vx = Vx SHR 1.
// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
fn SHR(x: &char, y: &char) {
    println!("\x1b[0;32mSetting V{} = V{} SHR V{}\x1b[0m", x, x, y);
}

// Set Vx = Vy - Vx, set VF = NOT borrow.
// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
fn SUBN(x: &char, y: &char) {
    println!("\x1b[0;32mSetting V{} = V{} - V{}\x1b[0m", x, x, y);
}

// Set Vx = Vx SHL 1.
// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
fn SHL(x: &char, y: &char) {
    println!("\x1b[0;32mSetting V{} = V{} SHL 1\x1b[0m", x, x);
}

pub fn RegisterOperation(instructions: &Vec<char>) {
    let rator = instructions[3];
    let rand1 = instructions[1];
    let rand2 = instructions[2];
    match rator {
        '0' => LD(&rand1, &rand2),
        '1' => OR(&rand1, &rand2),
        '2' => AND(&rand1, &rand2),
        '3' => XOR(&rand1, &rand2),
        '4' => ADD(&rand1, &rand2),
        '5' => SUB(&rand1, &rand2),
        '6' => SHR(&rand1, &rand2),
        '7' => SUBN(&rand1, &rand2),
        'e' => SHL(&rand1, &rand2),
        _ => println!("No Operation found for {}!", rator),
    }
}
