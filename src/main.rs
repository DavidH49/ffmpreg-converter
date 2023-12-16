mod consts;

use crate::consts::*;
use std::{env, process, process::Command};
use regex::Regex;


fn main() {
    let args = get_args()
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
    });

    let src_path = &args[1];
    let dst_path = parse_path(src_path);

    let cmd = Command::new("ffmpeg")
        .args(["-i", src_path, &dst_path])
        .output();

    println!("{:#?}", cmd);
}


fn get_args() -> Result<Vec<String>, &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2usize {
        return Err("not enough arguments!")
    }

    Ok(args)
}


fn parse_path(path: &str) -> String {
    // Replace file extension
    let dst_path = Regex::new(REGEX_FILEEXT)
        .unwrap()
        .replace(path, EXTENSION);

    // Replace file path
    let dst_path = Regex::new(REGEX_FILEPATH)
        .unwrap()
        .replace(&dst_path, DST_PATH);

    dst_path.to_string()
}