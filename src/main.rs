use std::fs;

use clap::Parser;
use rcli::{process_csv, process_decode, process_encode, process_password, process_sign,process_verify,process_key_gen, Base64Subcommand, Opts, Subcommand, TextSignFormat, TextSubcommand};


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
            let pw=process_password(&genopts)?;
            println!("{}",pw.0);
            println!("{}",pw.1);
        },
        Subcommand::Base64(base64)=>{
            match base64 {
                Base64Subcommand::Encode(s) =>{
                    let encode_ret= process_encode(&s.input, s.format)?;
                    println!("{}",encode_ret);
                },
                Base64Subcommand::Decode(d)=>{
                    let decode_ret = process_decode(&d.output, d.format)?;
                    println!("{}",decode_ret);
                }
            }
        },
        Subcommand::Text(subcom)=>{
            match subcom {
                TextSubcommand::Sign(opts)=>{
                    process_sign(&opts.input, &opts.key, opts.format)?;
                },
                TextSubcommand::Verify(opts)=>{
                    process_verify(&opts.input, &opts.key, opts.format, &opts.sign)?;
                },
                TextSubcommand::Generate(opts)=>{
                    let ret = process_key_gen(opts.format)?;
                    match opts.format {
                        TextSignFormat::Blake3=>{
                            let name= opts.output.join("blake3.txt");
                            fs::write(name, &ret[0])?;

                        },
                        TextSignFormat::Ed25519=>{
                            let dir_name = opts.output;
                            fs::write(dir_name.join("ed25519.sk"), &ret[0])?;
                            fs::write(dir_name.join("ed25519.pk"), &ret[1])?;
                        }
                    }
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
    fn test_ed25519_1() {
        
    }


}