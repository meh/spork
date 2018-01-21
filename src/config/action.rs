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
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use toml;

use action::{self, Action};
use key::{self, Key};
use error;

#[derive(Clone, Default, Debug)]
pub struct Actions(pub(super) Arc<RwLock<Data>>);

#[derive(Default, Debug)]
pub(super) struct Data {
	pub actions: HashMap<Key, Vec<Action>>,
}

impl Actions {
	pub fn load(&self, table: &toml::value::Table) -> error::Result<()> {
		if let Some(table) = table.get("actions").and_then(|v| v.as_table()) {
			let mut actions = HashMap::new();

			for (key, action) in table {
				if let Some(definition) = action.as_str() {
					let key    = Key::from_str(key)?;
					let action = Action::from_str(definition)?;

					if !actions.contains_key(&key) {
						actions.insert(key.clone(), Vec::new());
					}

					actions.get_mut(&key).unwrap().push(action);
				}
			}

			self.0.write().unwrap().actions = actions;
		}

		Ok(())
	}
}
