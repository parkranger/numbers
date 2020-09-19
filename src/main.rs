//#![allow(unused_imports)]
//#![allow(unused_variables)]
#![allow(dead_code)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate log;

mod app;
mod calc;
mod prime;

fn main() {
    env_logger::init();

    if let Err(ref errors) = app::run() {
        eprintln!("Error level - description");
        errors
            .iter()
            .enumerate()
            .for_each(|(index, error)| eprintln!("└> {} - {}", index, error));

        if let Some(backtrace) = errors.backtrace() {
            eprintln!("{:?}", backtrace);
        }

        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}
