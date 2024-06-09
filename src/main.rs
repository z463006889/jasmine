
fn add(x:i32,y:i32) ->i32{
    x+y
}

enum Types<T> {
    AniI32(T),
    Nothing,
}

fn main() {
    let mut v=vec![1,2,3];
    v.push(4);
    println!("Hello, Rust!");
    println!("{:?}",v);
    let t: Types<i32>=Types::AniI32(20);
    match t {
        Types::AniI32(f)=>println!("{}",f),
        Types::Nothing=>println!("Nothing"),
    };
}

