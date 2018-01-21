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

use std::str::FromStr;
use xcb;
use shlex;

use error;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Action {
	/// Execute the command in the shell.
	Execute(String, Vec<String>),

	/// Change the current desktop.
	Desktop(Desktop),

	/// Kill the given window.
	Kill(Window),
	
	/// Change the grid for the given window.
	Grid(Window, String),

	/// Change the current focus.
	Focus(Focus),

	/// Change the decorations for the given window.
	Decoration(Window, Decoration),
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Window {
	/// The currently active window.
	Active,

	/// The specific window ID.
	Id(xcb::Window),

	/// A window matching the name.
	Name(String),
}

impl FromStr for Window {
	type Err = error::Error;

	fn from_str(s: &str) -> error::Result<Self> {
		if s == "$WINDOW" {
			Ok(Window::Active)
		}
		else if s.chars().all(|c| c.is_digit(10)) {
			Ok(Window::Id(s.parse().unwrap()))
		}
		else {
			Ok(Window::Name(s.into()))
		}
	}
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Desktop {
	/// The next desktop.
	Next,

	/// The previous desktop.
	Previous,

	/// The desktop with the given name.
	Name(String),

	/// The desktop at the given index.
	Index(isize),
}

impl FromStr for Desktop {
	type Err = error::Error;

	fn from_str(s: &str) -> error::Result<Self> {
		Ok(match s {
			"next" =>
				Desktop::Next,

			"prev" | "previous" =>
				Desktop::Previous,

			name =>
				if let Ok(index) = name.parse() {
					Desktop::Index(index)
				}
				else {
					Desktop::Name(name.into())
				}
		})
	}
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Focus {
	/// The next window.
	Next,

	/// The previous window.
	Previous,

	/// The specific window.
	Window(Window),
}

impl FromStr for Focus {
	type Err = error::Error;

	fn from_str(s: &str) -> error::Result<Self> {
		match s {
			"next" =>
				Ok(Focus::Next),

			"prev" | "previous" =>
				Ok(Focus::Previous),

			name =>
				Ok(Focus::Window(Window::from_str(name)?))
		}
	}
}


#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Decoration {
	/// Toggle decorations on and off.
	Toggle,
}

impl FromStr for Decoration {
	type Err = error::Error;

	fn from_str(s: &str) -> error::Result<Self> {
		match s {
			"toggle" =>
				Ok(Decoration::Toggle),

			_ =>
				Err(error::Action::Parse.into()),
		}
	}
}

impl FromStr for Action {
	type Err = error::Error;

	fn from_str(s: &str) -> error::Result<Self> {
		if !s.starts_with('!') {
			let mut name      = shlex::split(s).ok_or(error::Action::Parse)?;
			let     arguments = name.split_off(1);

			return Ok(Action::Execute(name.pop().ok_or(error::Action::Parse)?, arguments));
		}

		let mut parts = s[1..].split(' ');

		match parts.next() {
			Some("desktop") => match parts.next() {
				Some(value) =>
					Ok(Action::Desktop(Desktop::from_str(value)?)),

				_ =>
					Err(error::Action::Parse.into())
			},

			Some("grid") => match parts.next() {
				Some(window) => match parts.next() {
					Some(name) => {
						Ok(Action::Grid(Window::from_str(window)?, name.into()))
					}

					_ =>
						Err(error::Action::Parse.into())
				},

				_ =>
					Err(error::Action::Parse.into())
			},

			Some("focus") => match parts.next() {
				Some(value) =>
					Ok(Action::Focus(Focus::from_str(value)?)),

				_ =>
					Err(error::Action::Parse.into())
			},

			Some("kill") => match parts.next() {
				Some(value) =>
					Ok(Action::Kill(Window::from_str(value)?)),

				_ =>
					Err(error::Action::Parse.into())
			},

			Some("decoration") => match parts.next() {
				Some(window) => match parts.next() {
					Some(what) =>
						Ok(Action::Decoration(Window::from_str(window)?, Decoration::from_str(what)?)),

					_ =>
						Err(error::Action::Parse.into())
				},

				_ =>
					Err(error::Action::Parse.into())
			},

			_ =>
				Err(error::Action::Parse.into())
		}
	}
}
