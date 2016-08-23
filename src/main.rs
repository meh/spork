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

#![feature(mpsc_select, question_mark, pub_restricted)]

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
use error::Error;

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

fn main() {
	env_logger::init().unwrap();

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

	let config    = Config::load(matches.value_of("config")).unwrap();
	let display   = Display::open(matches.value_of("display"), config.clone()).unwrap();
	let interface = Interface::spawn(config.clone()).unwrap();
}
