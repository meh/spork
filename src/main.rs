// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// This file is part of spork.
//
// spork is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// spork is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with spork.  If not, see <http://www.gnu.org/licenses/>.

#![feature(mpsc_select)]

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate clap;
use clap::{Arg, App};

extern crate xdg;
extern crate toml;
extern crate dbus;

#[macro_use] extern crate bitflags;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;
#[macro_use] extern crate crossbeam_channel as channel;

extern crate palette;
extern crate petgraph;

extern crate meval;
extern crate regex;
extern crate unicode_segmentation;
extern crate shlex;

extern crate libc;
extern crate xcb;
extern crate xcb_util as xcbu;
extern crate xkbcommon;

#[macro_use]
mod util;

mod error;

mod key;
use key::Key;

mod action;
use action::Action;

mod decoration;
use decoration::Decoration;

mod window;
use window::Window;

mod grid;
use grid::Grid;

mod display;
use display::Display;

mod config;
use config::Config;

mod interface;
use interface::Interface;

mod manager;
use manager::Manager;

use std::process;
use std::io::{self, Write};
use std::fmt;

macro_rules! exit {
	($body:expr) => (
		match $body {
			Ok(value) =>
				value,

//			Err(error) =>
//				if let Some(err) = error.downcast_ref::<error::X>() {
//					match *err {
//						error::X::MissingExtension { ref name }
//					}
//				}
//
//			Err(Error::Parse) =>
//				exit(10, "Could not load configuration file."),
//
//			Err(e@Error::X(error::X::Connection(..))) =>
//				exit(20, e),
//
//			Err(e@Error::X(error::X::HasWindowManager)) =>
//				exit(21, e),
//
//			Err(Error::X(error::X::MissingExtension(ref name))) =>
//				exit(22, format!("Missing extension {}", name)),
//
//			Err(e@Error::DBus(error::DBus::AlreadyRegistered)) =>
//				exit(30, e),

			Err(err) =>
				exit(255, err)
		}
	);
}

fn exit<T: fmt::Display>(code: i32, message: T) -> ! {
	writeln!(&mut io::stderr(), "spork: {}", message).unwrap();
	process::exit(code);
}

fn main() {
	env_logger::init();

	let matches = App::new("spork")
		.version(env!("CARGO_PKG_VERSION"))
		.author("meh. <meh@schizofreni.co>")
		.arg(Arg::with_name("display")
			.short("d")
			.long("display")
			.takes_value(true)
			.help("The display to connect to."))
		.arg(Arg::with_name("config")
			.short("c")
			.long("config")
			.takes_value(true)
			.help("Path to the configuration file."))
		.get_matches();

	let config    = exit!(Config::load(matches.value_of("config")));
	let display   = exit!(Display::open(matches.value_of("display"), config.clone()));
	let interface = exit!(Interface::spawn(config.clone()));
	let manager   = exit!(Manager::new(config.clone(), display.clone()));

	// XXX: select! is icky, this works around shadowing the outer name
	let x = display::sink(&display);
	let i = &*interface;

	loop {
		select_loop! {
			recv(x, event) => {
				match event.response_type() {
					e => {
						println!("{:?}", e);
					}
				}
			},

			recv(i, event) => {
				match event {
					interface::Request::Reload(ref path) => (),
					interface::Request::Execute => (),
				}
			}
		}
	}
}
