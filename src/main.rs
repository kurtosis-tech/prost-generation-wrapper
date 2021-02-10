use std::{env, fs::canonicalize, path::PathBuf, process::exit};
use anyhow::{Context, Result};
use log::error;
extern crate pretty_env_logger;

fn main() {
    pretty_env_logger::init();
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        error!("Usage: {} input_dirpath output_dirpath input_filepath1 input_filepath2...", &args[0]);
        exit(1);
    }

    let raw_input_dirpath = &args[1];
    let raw_output_dirpath = &args[2];
    let raw_input_filepaths = &args[3..];

    let result = generate_protos(raw_input_dirpath, raw_output_dirpath, raw_input_filepaths);
    if result.is_err() {
        error!("An error occurred generating the bindings: {:#}", result.unwrap_err());
        exit(1);
    }
}

fn generate_protos(raw_input_dirpath: &str, raw_output_dirpath: &str, raw_input_proto_filepaths: &[String]) -> Result<()> {
    let input_dirpath = canonicalize(raw_input_dirpath)
        .context(format!("Could not canonicalize raw input dirpath '{}'", raw_input_dirpath))?;
    let output_dirpath = canonicalize(raw_output_dirpath)
        .context(format!("Could not canonicalize raw output dirpath '{}'", raw_output_dirpath))?;

    let mut input_proto_filepaths: Vec<PathBuf> = Vec::new();
    for raw_filepath in raw_input_proto_filepaths {
        let can_filepath = canonicalize(raw_filepath)
            .context(format!("Could not canonicalize raw proto filepath '{}'", raw_filepath))?;
        input_proto_filepaths.push(can_filepath);
    }

    tonic_build::configure()
        .out_dir(&output_dirpath)
        .compile(&input_proto_filepaths, &[input_dirpath])
        .context("An error occurred generating the .proto files")?;
    return Ok(());
}
