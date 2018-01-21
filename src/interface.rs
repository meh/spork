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

use std::thread;
use std::sync::Arc;
use std::ops::Deref;
use channel;

use dbus;

use error;
use config::Config;

pub struct Interface {
	receiver: channel::Receiver<Request>,
	sender:   channel::Sender<Response>,
	signals:  channel::Sender<Signal>,
}

#[derive(Debug)]
pub enum Request {
	/// Reload the configuration file.
	Reload(Option<String>),
	Execute,
}

#[derive(Debug)]
pub enum Response {
	/// Whether the reload was successful or not.
	Reload(bool),
	Execute,
}

#[derive(Debug)]
pub enum Signal {

}

impl Interface {
	/// Spawn a DBus interface with the given configuration.
	pub fn spawn(config: Config) -> Result<Interface, dbus::Error> {
		let (sender, i_receiver)   = channel::unbounded();
		let (i_sender, receiver)   = channel::unbounded();
		let (s_sender, signals)    = channel::unbounded();
		let (g_sender, g_receiver) = channel::unbounded::<Result<(), dbus::Error>>();

		macro_rules! dbus {
			(connect) => (
				match dbus::Connection::get_private(dbus::BusType::Session) {
					Ok(value) => {
						value
					}

					Err(error) => {
						g_sender.send(Err(error)).unwrap();
						return;
					}
				}
			);

			(register $conn:expr, $name:expr) => (
				match $conn.register_name($name, dbus::NameFlag::DoNotQueue as u32) {
					Err(error) => {
						g_sender.send(Err(error)).unwrap();
						return;
					}

					Ok(value) => {
						value
					}
				}
			);

			(watch $conn:expr, $filter:expr) => (
				if let Err(error) =  $conn.add_match($filter) {
					g_sender.send(Err(error)).unwrap();
					return;
				}
			);

			(ready) => (
				g_sender.send(Ok(())).unwrap();
			);

			(check) => (
				g_receiver.recv().unwrap()
			)
		}

		thread::spawn(move || {
			let c = dbus!(connect);
			let f = dbus::tree::Factory::new_fn::<()>();

			dbus!(register c, "meh.rust.WindowManager");
			dbus!(ready);

			let tree = f.tree(())
				.add(f.object_path("/meh/rust/WindowManager", ()).introspectable().add(f.interface("meh.rust.WindowManager", ())
					.add_m(f.method("Reload", (), move |m| {
						sender.send(Request::Reload(m.msg.get1())).unwrap();

						if let Response::Reload(value) = receiver.recv().unwrap() {
							Ok(vec![m.msg.method_return().append1(value)])
						}
						else {
							unreachable!();
						}
					}).inarg::<String, _>("path").outarg::<bool, _>("success"))));

			tree.set_registered(&c, true).unwrap();

			for item in tree.run(&c, c.iter(500)) {
				match item {
					dbus::ConnectionItem::Nothing => {
						while let Ok(signal) = signals.try_recv() {
							// TODO: fill this in
						}
					}

					dbus::ConnectionItem::Signal(m) => {
						match (&*m.interface().unwrap(), &*m.member().unwrap()) {
							_ => ()
						}
					}

					_ => ()
				}
			}
		});

		dbus!(check)?;

		Ok(Interface {
			receiver: i_receiver,
			sender:   i_sender,
			signals:  s_sender,
		})
	}

	pub fn response(&self, value: Response) -> Result<(), channel::SendError<Response>> {
		self.sender.send(value)
	}

	pub fn signal(&self, value: Signal) -> Result<(), channel::SendError<Signal>> {
		self.signals.send(value)
	}
}

impl Deref for Interface {
	type Target = channel::Receiver<Request>;

	fn deref(&self) -> &channel::Receiver<Request> {
		&self.receiver
	}
}
