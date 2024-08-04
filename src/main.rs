use std::os::windows::process;

use clap::Parser;
use rcli::{process_csv, process_encode,process_decode, process_password, Base64Subcommand, Opts, Subcommand};


fn main()->anyhow::Result<()>{
    let opts= Opts::parse();
    match opts.cmd {
        Subcommand::Trans(opts)=>{
            let output = if let Some(outs)=opts.output{
                outs.clone()
            }else{
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input,output,opts.format)?;
        },
        Subcommand::GenPass(genopts)=>{
            process_password(&genopts)?;
        },
        Subcommand::Base64(base64)=>{
            match base64 {
                Base64Subcommand::Encode(s) =>{
                    println!("input{}",s.input);
                    process_encode(&s.input, s.format)?;
                },
                Base64Subcommand::Decode(d)=>{
                    process_decode(&d.output, d.format)?;
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod hashtests {
    #[test]
    fn hash_test() {
        let sd = std::io::stdin();
        let mut buf = String::new();
        sd.read_line(&mut buf);
        let hash = blake3::hash(buf.as_bytes());
        println!("{}", hash);

    }

    #[test]
    fn test_ed25519() {
        
    }


}