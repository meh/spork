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

use std::sync::{Arc, RwLock};
use toml;

use error;

#[derive(Clone, Default, Debug)]
pub struct Desktops(pub(super) Arc<RwLock<Data>>);

#[derive(Default, Debug)]
pub(super) struct Data {
	pub desktops: Vec<Desktop>,
}

#[derive(Eq, PartialEq, Clone, Default, Debug)]
pub struct Desktop {
	pub name: Option<String>,
}

impl Desktops {
	pub fn load(&self, table: &toml::Table) -> error::Result<()> {
		if let Some(slice) = table.get("desktop").and_then(|v| v.as_slice()) {
			let mut desktops = Vec::with_capacity(slice.len());

			for table in slice {
				let mut desktop = Desktop::default();

				if let Some(value) = table.lookup("name").and_then(|v| v.as_str()) {
					desktop.name = Some(value.into());
				}

				desktops.push(desktop);
			}

			self.0.write().unwrap().desktops = desktops;
		}

		Ok(())
	}
}
