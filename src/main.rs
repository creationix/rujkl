#[derive(Debug, Copy, Clone)]
enum Value {
    Nil,
    Bool(bool),
    Integer(i32),
    Cell(u32) // Index into pair heap
}

fn main() {
    let a = Value::Nil;
    let b = Value::Bool(true);
    let c = Value::Integer(42);
    let d = Value::Cell(0);
    let heap = [(a, b), (c, d)];
    println!("a={:?} b={:?} c={:?} heap={:?}", a, b, c, &heap);
}
