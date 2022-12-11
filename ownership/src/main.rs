fn main() { 
    
    str();
    string();
    let x = 5.0;
    copies_params(x);
    println!("{x}");

    let y = Box::new(20.0);
    head_procedure(y);
    //println!("{y}"); Value borrowed after 'move'
    let z = Box::new(10.0);
    borrow_heap_procedure(&z);
    println!("{z}");
    borrowing();

    let obj = Object {
        x: 3, 
        y: 8
    };
    use_object(obj);
    // println!("{:?}", obj);  Use after move, objects are moved by default 
}

fn copies_params(param: f64) {
    println!("{param}");
}

fn head_procedure(param: Box<f64>) {
    println!("{param}");
}

fn borrow_heap_procedure(param: &Box<f64>) {
    println!("{param}");
}

fn str()  { // s is not valid
    // s is valid here
    let s = "Hello";
} // s is dropped and no longer valid

fn string() {
    let mut string = String::new();
    string.push_str("Hello World!");
    println!("{string}");
}

fn borrowing() {
    let mut a = String::from("Hello");
    let mut b = &a;
  //  b.push_str("d"); Now allow to mutate immutable references

    let c = &a;
    // a.push('a'); // The vector might be reallocated with a new push
    // thus the current refence would be invalid
    println!("{}{}{}", a, b, c);
}
#[derive(Debug, Clone)]
struct Object {
    x: i32,
    y: i32
}
fn use_object(param: Object) {
    println!("{:#?}", param);
}