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

use std::sync::Arc;
use xcb;

use error;

pub struct Window {
	conn: Arc<xcb::Connection>,
	id:   xcb::Window,
}

impl Window {
	pub fn new(conn: Arc<xcb::Connection>, id: xcb::Window) -> error::Result<Self> {
		Ok(Window {
			conn: conn,
			id:   id,
		})
	}
}
