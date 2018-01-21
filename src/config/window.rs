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
use failure::Error;

use util;
use decoration::{Border, Decoration};

#[derive(Clone, Default, Debug)]
pub struct Windows(pub(super) Arc<RwLock<Data>>);

#[derive(Default, Debug)]
pub(super) struct Data {
	pub decoration: Decoration,
	pub windows:    Vec<Window>,
}

#[derive(Clone, Default, Debug)]
pub struct Window {
	instance:   Option<String>,
	class:      Option<String>,
	desktop:    Option<Desktop>,
	grid:       Option<Grid>,
	decoration: Option<Decoration>,
	level:      i8,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Desktop {
	Name(String),
	Index(isize),
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Grid {
	Name(String),
	Index(isize),
}

impl Windows {
	pub fn load(&self, table: &toml::value::Table) -> Result<(), Error> {
		if let Some(table) = table.get("default") {
			if let Some(table) = table.get("decoration").and_then(|v| v.as_table()) {
				self.0.write().unwrap().decoration = decoration(table);
			}
		}

		if let Some(slice) = table.get("window").and_then(|v| v.as_array()) {
			let mut windows = Vec::with_capacity(slice.len());

			for table in slice {
				let mut window = Window::default();

				if let Some(value) = table.get("instance").and_then(|v| v.as_str()) {
					window.instance = Some(value.into());
				}

				if let Some(value) = table.get("class").and_then(|v| v.as_str()) {
					window.class = Some(value.into());
				}

				if let Some(value) = table.get("desktop") {
					window.desktop = match *value {
						toml::Value::Integer(value) =>
							Some(Desktop::Index(value as isize)),

						toml::Value::String(ref value) =>
							Some(Desktop::Name(value.clone())),

						_ =>
							None
					}
				}

				if let Some(value) = table.get("grid") {
					window.grid = match *value {
						toml::Value::Integer(value) =>
							Some(Grid::Index(value as isize)),

						toml::Value::String(ref value) =>
							Some(Grid::Name(value.clone())),

						_ =>
							None
					}
				}

				if let Some(value) = table.get("decoration") {
					if let Some(table) = value.as_table() {
						window.decoration = Some(decoration(table));
					}
					else {
						window.decoration = Some(Default::default());
					}
				}

				if let Some(value) = table.get("level").and_then(|v| v.as_integer()) {
					window.level = value as i8;
				}

				windows.push(window);
			}

			self.0.write().unwrap().windows = windows;
		}

		Ok(())
	}
}

fn decoration(table: &toml::value::Table) -> Decoration {
	let mut decoration = Decoration::default();

	if let Some(table) = table.get("border").and_then(|v| v.as_table()) {
		let mut border = Border::default();

		if let Some(value) = table.get("width").and_then(|v| v.as_integer()) {
			border.width = value as u8;
		}

		if let Some(value) = table.get("color").and_then(|v| v.as_str()).and_then(|v| util::to_color(v)) {
			border.color = value;
		}

		decoration.border = Some(border);
	}

	decoration
}
