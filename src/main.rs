extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate clap;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate tempdir;

include!(concat!(env!("OUT_DIR"), "/main.rs"));
