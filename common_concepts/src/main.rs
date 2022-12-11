
fn main() {
   // Variables are immutable by default
    let x = 5;
    println!("Value of x: {x}");
   // x = 6; compile error
    let mut x = 2;
    x = x + 2;
    println!("Value of x: {x}");

    // Constants 
    const MAX_POINTS: usize = 100;
    const THREE_HOURS_IN_SECONDS: usize = 60 * 60 * 3;

    // Shadowing
    let health = 20;
    let health = health + 20;
    println!("Value of health: {health}");
    let spaces = "   ";
    let spaces = spaces.len();
    // Datatypes
    // byte 
    println!("U8 Min value: {}", std::u8::MIN);
    println!("U8 Max value: {}", std::u8::MAX);
    println!("I8 Min value: {}", std::i8::MIN);
    println!("I8 Max value: {}", std::i8::MAX);
    // 2 bytes
    println!("U16 Min value: {}", std::u16::MIN);
    println!("U16 Max value: {}", std::u16::MAX);
    println!("I16 Min value: {}", std::i16::MIN);
    println!("I16 Max value: {}", std::i16::MAX);
    // 4 bytes
    println!("U32 Min value: {}", std::u32::MIN);
    println!("U32 Max value: {}", std::u32::MAX);
    println!("I32 Min value: {}", std::i32::MIN);
    println!("I32 Max value: {}", std::i32::MAX);
    // 8 bytes
    println!("U64 Min value: {}", std::u64::MIN);
    println!("U64 Max value: {}", std::u64::MAX);
    println!("I64 Min value: {}", std::i64::MIN);
    println!("I64 Max value: {}", std::i64::MAX);
    // Architecture dependent
    println!("Usize Min value: {}", std::usize::MIN);
    println!("Isize Max value: {}", std::usize::MAX);

    //let overflow: u8 = 257;
        // addition
    let sum = 1 + 2;
    let substraction = 2 - 4;
    let product = 5 * 5;
    let remainder = 7 % 3;
    let division = 7 / 3;

    let tuple = (10, 20, 30);
    let head = tuple.0;

    // Fixed-size array , allocated on the stack
    let array = [1, 2, 3, 4, 5];
    let zeroes = [0; 9];
    let first = array[0];
    let second = array[1];
    another_function(4, 2);
    another_function(1, 7);

    let y = 6; // statement
    //2 * 2; // expression
    let four = multiply_by_2(2);
    let twelve = multiply_by_2(6);

    // This is a comment
    // And gets ignored by the compiler.

    // Control Flow
    // If expression

    let size = 7;
    if size >= 7 {
        println!("Number is greater or equal to 7");
    } else {
        println!("Number is less than 7");
    }
    // if in a let statement
    let condition = true;
    let do_task = if condition { 2 } else { 5 };

    // Loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 20 {
            break counter * 2;
        }
    };
    // Loop labels
    let mut c = 0;
    'counting_up: loop {}

    // for & while
    while true {
        break;
    }
    // indexed loop
    for n in 0..=3 {
        
    }
}   

fn another_function(x: i32, y: i32) {

}

fn multiply_by_2(x: isize) -> isize {
    x * 2
}