extern crate clap;

mod png;

use std::path::Path;
use std::process;
use clap::{Arg, App};
use png::{compress_file, Options};

fn main() {
  let matches = App::new("tiny-image")
    .version("0.1.0")
    .author("hanje. hanjie@youzan.com")
    .about("A image compressor written in Rust")
    .arg(Arg::with_name("IMAGE").help("Image to compress.").empty_values(false))
    .get_matches();

  if let Some(file) = matches.value_of("IMAGE") {
    if Path::new(&file).exists() {
      compress_file(String::from(file), Options { add_ext: true });
    } else {
      eprintln!("[tiny-image Error] No such file or directory.");
      process::exit(1); // exit
    }
  }
}
