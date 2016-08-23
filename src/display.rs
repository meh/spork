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

use std::ops::Deref;

use xcb;
use xcbu;

use error;
use config::Config;

/// Handles the X11 display.
pub struct Display {
	connection: xcbu::ewmh::Connection,
	randr:      xcb::QueryExtensionData,
}

unsafe impl Send for Display { }
unsafe impl Sync for Display { }

impl Display {
	/// Open the default display.
	pub fn open(name: Option<&str>, config: Config) -> error::Result<Self> {
		let (connection, screen) = xcb::Connection::connect(name)?;
		let connection           = xcbu::ewmh::Connection::connect(connection).map_err(|(e, _)| e)?;

		// Randr is used for the backlight and screen configuration changes events.
		let randr = {
			let extension = connection.get_extension_data(xcb::randr::id()).ok_or(error::X::MissingExtension("RANDR".into()))?;
			let version   = xcb::randr::query_version(&connection, 1, 2).get_reply()?;

			if version.major_version() != 1 || version.minor_version() < 2 {
				return Err(error::X::MissingExtension("RANDR".into()).into());
			}

			extension
		};

		Ok(Display {
			connection: connection,
			randr:      randr,
		})
	}

	/// Get the XRandr extension details.
	pub fn randr(&self) -> &xcb::QueryExtensionData {
		&self.randr
	}
}

impl Deref for Display {
	type Target = xcbu::ewmh::Connection;

	fn deref(&self) -> &Self::Target {
		&self.connection
	}
}
