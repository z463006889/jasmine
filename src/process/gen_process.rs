use anyhow::Ok;
use rand::{seq::SliceRandom, thread_rng, Rng};
use crate::cli::GenPassOpts;
use zxcvbn::{zxcvbn, Score};

const UPPER:&[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER:&[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBER:&[u8] = b"123456789";
const SYMBOL:&[u8] = b"!@#$%^&*-_"; //

pub fn process_password(opts:&GenPassOpts)->anyhow::Result<(std::string::String, Score)> {

    let mut rng = rand::thread_rng();
    let mut password=String::new();
    let mut chars=Vec::new();
    if opts.uppercase {
        chars.extend_from_slice(UPPER);
        password.push(find_random_char(UPPER).unwrap() as char);
    }
    if opts.lowercase {
        chars.extend_from_slice(LOWER);
        password.push(find_random_char(LOWER).unwrap() as char);
    }
    if opts.number {
        chars.extend_from_slice(NUMBER);
        password.push(find_random_char(NUMBER).unwrap() as char);
    }
    if opts.symbols {
        chars.extend_from_slice(SYMBOL);
        password.push(find_random_char(SYMBOL).unwrap() as char);
    }

    for _ in 0..opts.length-4 {
        let c = chars.choose(&mut rng).expect("Invalid character");
        password.push(*c as char);
    }
    let result = String::from_utf8(password.into())?;

    let v = zxcvbn(&result,&[]);
    let ret = (result, v.score());
    Ok(ret)
}

fn find_random_char(c:&[u8]) -> anyhow::Result<u8> {
    let mut rng = thread_rng();
    let choc = c.choose(&mut rng);
    match choc {
        Some(r)=>Ok(*r),
        _=>{Err(anyhow::anyhow!("Invalid Data"))}
    }
}