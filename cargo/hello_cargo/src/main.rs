struct Object
{
    data: usize,
}

fn add(a: usize, b: usize) -> usize {
    a + b
}

fn main() {
    println!("Initializing...");
    let obj = Object {
        data: 29
    };
    println!("Object data is: {}", add(obj.data, 6));
    
}
