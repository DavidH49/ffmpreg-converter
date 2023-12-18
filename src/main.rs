mod consts;

use crate::consts::*;
use std::{env, process, process::Command};
use once_cell::sync::Lazy;
use regex::Regex;


fn main() {
    let args = get_args()
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
    });

    let src_path = &args[1];
    let dst_path = parse_path(src_path);

    let command_args = ["-hwaccel","cuda", "-i", src_path, "-c:v", "h264_nvenc", &dst_path];

    println!("ffmpeg {:?}", &command_args);

    let cmd = Command::new("ffmpeg")
        .args(command_args)
        .output()
        .unwrap_or_else(|err| panic!("{err}"));

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
    static RE_FILEEXT: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX_FILEEXT).unwrap());
    static RE_FILEPATH: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX_FILEPATH).unwrap());

    // Replace file extension
    let dst_path = RE_FILEEXT.replace(path, EXTENSION);

    // Replace file path
    let dst_path = RE_FILEPATH.replace(&dst_path, DST_PATH);

    dst_path.to_string()
}