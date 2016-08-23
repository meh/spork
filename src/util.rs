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

use palette::Rgb;
use regex::Regex;

lazy_static! {
	static ref HEX_RGB: Regex = Regex::new(r"#([:xdigit:]{2})([:xdigit:]{2})([:xdigit:]{2})").unwrap();
}

pub fn color<T: AsRef<str>>(value: T) -> Option<Rgb> {
	HEX_RGB.captures(value.as_ref()).map(|captures| {
		Rgb::new_u8(
			u8::from_str_radix(captures.at(1).unwrap_or("0"), 16).unwrap_or(0),
			u8::from_str_radix(captures.at(2).unwrap_or("0"), 16).unwrap_or(0),
			u8::from_str_radix(captures.at(3).unwrap_or("0"), 16).unwrap_or(0),
		)
	})
}
