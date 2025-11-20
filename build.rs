// use clap::builder::PossibleValue;
// use clap::{ArgAction, Command, ValueEnum, arg, value_parser};
use clap::ValueEnum;
use clap_complete::{Shell, generate_to};
use clap_mangen::Man;
use std::env;
use std::io::Error;
use std::path::PathBuf;

include!("./src/commands.rs");

fn main() -> Result<(), Error> {
    let Some(outdir) = env::var_os("OUT_DIR") else {
        return Ok(());
    };

    let mut cmd = build();
    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, "todo", &outdir)?;
    }

    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(PathBuf::from(outdir).join("todo.1"), buffer)?;

    Ok(())
}
