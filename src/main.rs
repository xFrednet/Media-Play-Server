#![warn(clippy::pedantic, clippy::nursery)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

mod http_server;
mod ws_server;

use std::thread::spawn;

fn main() {
    spawn(|| http_server::start());
    ws_server::start();
}
