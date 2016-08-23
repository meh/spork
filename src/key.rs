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
use unicode_segmentation::UnicodeSegmentation;

use error;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Key {
	modifiers: Modifiers,
	value:     Value,
}

bitflags! {
	pub flags Modifiers: u8 {
		const NONE  = 0x00,
		const SHIFT = 0x01,
		const META  = 0x02,
		const CTRL  = 0x04,
		const ZUPER = 0x08,
	}
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Value {
	Code(u32),
	Char(String),
	Special(Special),
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Special {
	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,

	Right,
	Left,
	Up,
	Down,

	Delete,
	Tab,
}

impl FromStr for Key {
	type Err = error::Error;

	fn from_str(s: &str) -> error::Result<Self> {
		let mut modifiers = s.split('-').collect::<Vec<&str>>();
		let     value     = modifiers.pop().ok_or(error::Error::Parse)?;

		let modifiers = modifiers.iter().fold(Modifiers::empty(), |r, m|
			r | match *m {
				"S" => SHIFT,
				"M" => META,
				"C" => CTRL,
				"Z" => ZUPER,
				_   => NONE,
			});

		let value = if UnicodeSegmentation::graphemes(value, true).count() == 1 {
			Value::Char(value.to_lowercase())
		}
		else if value.chars().all(|c| c.is_digit(10)) {
			Value::Code(value.parse().unwrap())
		}
		else {
			Value::Special(match &*value.to_lowercase() {
				"f1"  => Special::F1,
				"f2"  => Special::F2,
				"f3"  => Special::F3,
				"f4"  => Special::F4,
				"f5"  => Special::F5,
				"f6"  => Special::F6,
				"f7"  => Special::F7,
				"f8"  => Special::F8,
				"f9"  => Special::F9,
				"f10" => Special::F10,
				"f11" => Special::F11,
				"f12" => Special::F12,

				"left"  => Special::Left,
				"right" => Special::Right,
				"up"    => Special::Up,
				"down"  => Special::Down,

				"tab" => Special::Tab,
				"del" | "delete" => Special::Delete,

				_ =>
					return Err(error::Error::Parse),
			})
		};

		Ok(Key {
			modifiers: modifiers,
			value:     value,
		})
	}
}
