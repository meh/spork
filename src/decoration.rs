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

use palette;

#[derive(PartialEq, Copy, Clone, Default, Debug)]
pub struct Decoration {
	pub border: Option<Border>,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Border {
	pub width: u8,
	pub color: palette::Rgb,
}

impl Default for Border {
	fn default() -> Self {
		Border {
			width: 1,
			color: palette::Rgb::new(1.0, 1.0, 1.0),
		}
	}
}
