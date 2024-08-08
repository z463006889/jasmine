use std::{fs::File, io::Read};

use anyhow::Ok;
use base64::{engine::general_purpose::{STANDARD,URL_SAFE}, prelude::*};
use clap::builder::Str;

use crate::{get_reader, Base64Type};
pub fn process_encode(input: &str,format:Base64Type) -> anyhow::Result<String>{
    let mut reader = get_reader(input)?;
    let mut buf=Vec::new();
    reader.read_to_end(&mut buf)?;
    let encode_ = match format {
        Base64Type::Standard=>STANDARD.encode(buf),
        Base64Type::Urlsafe=>URL_SAFE.encode(buf),
    };
    Ok(encode_)
}


pub fn process_decode(output: &str,format:Base64Type) -> anyhow::Result<String>{
    
    let mut reader = get_reader(output)?;
    let mut buf:String=String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    println!("decode: {:?}",buf);
    let decode_r = match format {
        Base64Type::Standard=>{
            let decode_ = STANDARD.decode(buf)?;
            String::from_utf8(decode_)?
        },
        Base64Type::Urlsafe=>{
            let decode_ = URL_SAFE.decode(buf)?;
            String::from_utf8(decode_)?
        }
    };
    Ok(decode_r.trim().to_string())
}

