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
use std::sync::Arc;
use std::sync::mpsc::{Receiver, sync_channel};
use std::thread;

use xcb;
use xcbu;

use error;
use config::Config;

/// Handles the X11 display.
#[derive(Clone)]
pub struct Display(Arc<Data>);
pub(crate) struct Data {
	connection: xcbu::ewmh::Connection,

	randr: bool,
}

impl Display {
	/// Open the given display.
	pub fn open(name: Option<&str>, config: Config) -> error::Result<Self> {
		let (connection, screen) = xcb::Connection::connect(name)?;
		let connection           = xcbu::ewmh::Connection::connect(connection).map_err(|(e, _)| e)?;
		let root                 = connection.get_setup().roots().nth(screen as usize).unwrap().root();

		// RANDR is used for screen configuration events.
		let randr = if let Some(randr) = connection.get_extension_data(xcb::randr::id()) {
			let version = xcb::randr::query_version(&connection, 1, 2).get_reply()?;

			if version.major_version() != 1 || version.minor_version() < 2 {
				return Err(error::X::MissingExtension { name: "RANDR".into() }.into())
			}

			true
		}
		else {
			false
		};

		// Check if another window manager is running.
		if xcb::change_window_attributes_checked(&connection, root, &[
			(xcb::CW_EVENT_MASK, xcb::EVENT_MASK_SUBSTRUCTURE_REDIRECT)]).request_check().is_err()
		{
			return Err(error::X::HasWindowManager.into());
		}

		Ok(Display(Arc::new(Data {
			connection: connection,
			randr: randr,
		})))
	}

	/// Get the XRandr extension details.
	pub fn randr(&self) -> Option<xcb::QueryExtensionData> {
		if self.0.randr {
			Some(self.0.connection.get_extension_data(xcb::randr::id()).unwrap())
		}
		else {
			None
		}
	}
}

pub fn sink(display: &Display) -> Receiver<xcb::GenericEvent> {
	let (sender, receiver) = sync_channel(1);
	let display            = display.0.clone();

	// Drain events into a channel.
	thread::Builder::new().name("spork-x11-sink".into()).spawn(move || {
		while let Some(event) = display.connection.wait_for_event() {
			sender.send(event).unwrap();
		}
	}).unwrap();

	receiver
}

impl Deref for Display {
	type Target = xcbu::ewmh::Connection;

	fn deref(&self) -> &Self::Target {
		&self.0.connection
	}
}
