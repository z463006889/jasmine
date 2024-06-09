
fn add(x:i32,y:i32) ->i32{
    x+y
}

fn main() {
    let mut v=vec![1,2,3];
    v.push(4);
    add(4, 7);
    println!("Hello, Rust!");
    println!("{:?}",v);
}

