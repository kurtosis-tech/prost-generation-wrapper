use std::{env, fs::canonicalize, process::exit};
use anyhow::{Context, Result};
use log::error;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

fn main() {
    pretty_env_logger::init();
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    if args.len() != 3 {
        error!("Takes two args: input_proto_filepath output_dirpath");
        exit(1);
    }

    let result = generate_protos(&args[1], &args[2]);
    if result.is_err() {
        error!("An error occurred generating the bindings: {:#}", result.unwrap_err());
        exit(1);
    }
}

fn generate_protos(raw_input_proto_filepath: &str, raw_output_dirpath: &str) -> Result<()> {
    let input_proto_filepath = canonicalize(raw_input_proto_filepath)
        .context("Could not canonicalize input proto filepath")?;
    let output_dirpath = canonicalize(raw_output_dirpath)
        .context("Could not canonicalize output dirpath")?;
    let input_proto_dirpath = input_proto_filepath.join("..").canonicalize()
        .context("Could not canonicalize input proto dirpath")?;

    tonic_build::configure()
        .out_dir(&output_dirpath)
        .compile(&[input_proto_filepath], &[input_proto_dirpath])
        .context("An error occurred generating the .proto files")?;
    return Ok(());
}
