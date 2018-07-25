#![feature(ord_max_min)]
#![feature(drain_filter)]
#![feature(nll)]

             extern crate backtrace;
             extern crate bincode;
             extern crate chrono;
             extern crate crypto;
             extern crate dirs;
             extern crate enum_traits;
             extern crate env_logger;
             extern crate gilrs;
             extern crate os_type;
             extern crate rand;
             extern crate reqwest;
             extern crate serde;
             extern crate tempdir;
             extern crate toml;
             extern crate treeflection;
             extern crate uuid;
             extern crate winit;
             extern crate zip;
#[macro_use] extern crate enum_traits_macros;
#[macro_use] extern crate log;
#[macro_use] extern crate matches;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate treeflection_derive;

pub mod command_line;
pub mod config;
pub mod fighter;
pub mod files;
pub mod geometry;
pub mod input;
pub mod json_upgrade;
pub mod logger;
pub mod network;
pub mod os_input;
pub mod package;
pub mod panic_handler;
pub mod rules;
pub mod stage;
