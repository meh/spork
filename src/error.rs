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

use std::fmt;
use std::error;
use std::io;

pub use failure::Error;
pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum X {
	#[fail(display = "missing extension: {}", name)]
	MissingExtension {
		name: String
	},

	#[fail(display = "another window manager is already running")]
	HasWindowManager,
}

#[derive(Debug, Fail)]
pub enum DBus {
	#[fail(display = "the name has already been registered")]
	AlreadyRegistered,
}

#[derive(Debug, Fail)]
pub enum Key {
	#[fail(display = "key failed to parse")]
	Parse,
}

#[derive(Debug, Fail)]
pub enum Action {
	#[fail(display = "key failed to parse")]
	Parse,
}
