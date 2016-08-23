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

use config;
use error;

pub struct Grid {
	name:   String,
	config: config::Grids,
}

impl Grid {
	pub fn new<S: Into<String>>(name: S, config: config::Grids) -> error::Result<Self> {
		Ok(Grid {
			name:   name.into(),
			config: config,
		})
	}
}
