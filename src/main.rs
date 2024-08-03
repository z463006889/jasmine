use clap::Parser;
use rcli::{process_csv,process_password, Opts, Subcommand};


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
        }
    }
    Ok(())
}

