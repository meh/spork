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

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::Read;
use std::sync::{Arc, RwLock};

use xdg;
use toml;
use failure::Error;

use super::{Desktops, Windows, Grids, Actions};

#[derive(Clone, Default, Debug)]
pub struct Config {
	path: Arc<RwLock<Option<PathBuf>>>,

	actions:  Actions,
	desktops: Desktops,
	windows:  Windows,
	grids:    Grids,
}

impl Config {
	pub fn load<T: AsRef<Path>>(path: Option<T>) -> Result<Config, Error> {
		let config = Config::default();
		config.reload(path)?;

		Ok(config)
	}

	pub fn reload<T: AsRef<Path>>(&self, path: Option<T>) -> Result<(), Error> {
		let path = if let Some(path) = path {
			*self.path.write().unwrap() = Some(path.as_ref().into());
			path.as_ref().into()
		}
		else if let Some(path) = self.path.read().unwrap().clone() {
			path
		}
		else {
			xdg::BaseDirectories::with_prefix("spork").unwrap()
				.place_config_file("config.toml").unwrap()
		};

		let table = if let Ok(mut file) = File::open(path) {
			let mut content = String::new();
			file.read_to_string(&mut content)?;

			content.parse::<toml::Value>()?.try_into()?
		}
		else {
			toml::value::Table::new()
		};

		self.actions.load(&table)?;
		self.desktops.load(&table)?;
		self.windows.load(&table)?;
		self.grids.load(&table)?;

		Ok(())
	}

	pub fn resize(&self, screen: i32, width: u32, height: u32) {
		self.grids.resize(screen, width, height);
	}

	pub fn actions(&self) -> Actions {
		self.actions.clone()
	}

	pub fn desktops(&self) -> Desktops {
		self.desktops.clone()
	}

	pub fn windows(&self) -> Windows {
		self.windows.clone()
	}

	pub fn grids(&self) -> Grids {
		self.grids.clone()
	}
}
