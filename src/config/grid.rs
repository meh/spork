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
use std::collections::HashMap;
use petgraph::{Graph, DfsIter};
use regex::Regex;
use toml;
use meval;

use error;

#[derive(Clone, Default, Debug)]
pub struct Grids(pub(super) Arc<RwLock<Data>>);

#[derive(Default, Debug)]
pub(super) struct Data {
	grids:   HashMap<String, Grid>,
	screens: HashMap<i32, HashMap<String, f64>>,
}

#[derive(Clone, Debug)]
pub struct Grid {
	pub name: String,

	pub x: String,
	pub y: String,

	pub width:  String,
	pub height: String,
}

impl Default for Grid {
	fn default() -> Self {
		Grid {
			name: "anonymous".into(),

			x: "0".to_string(),
			y: "0".to_string(),

			width:  "screen.width".to_string(),
			height: "screen.height".to_string(),
		}
	}
}

impl Grids {
	pub fn load(&self, table: &toml::Table) -> error::Result<()> {
		if let Some(table) = table.get("grid").and_then(|v| v.as_table()) {
			let mut grids = HashMap::new();

			for (name, table) in table {
				let mut grid = Grid::default();
				grid.name = name.clone();

				if let Some(value) = table.lookup("x") {
					match *value {
						toml::Value::Integer(value) =>
							grid.x = value.to_string(),

						toml::Value::String(ref value) =>
							grid.x = value.clone(),

						_ => ()
					}
				}

				if let Some(value) = table.lookup("y") {
					match *value {
						toml::Value::Integer(value) =>
							grid.y = value.to_string(),

						toml::Value::String(ref value) =>
							grid.y = value.clone(),

						_ => ()
					}
				}

				if let Some(value) = table.lookup("width") {
					match *value {
						toml::Value::Integer(value) =>
							grid.width = value.to_string(),

						toml::Value::String(ref value) =>
							grid.width = value.clone(),

						_ => ()
					}
				}

				if let Some(value) = table.lookup("height") {
					match *value {
						toml::Value::Integer(value) =>
							grid.height = value.to_string(),

						toml::Value::String(ref value) =>
							grid.height = value.clone(),

						_ => ()
					}
				}

				grids.insert(grid.name.clone(), grid);
			}

			self.0.write().unwrap().grids = grids;
		}

		Ok(())
	}

	pub fn resize(&self, screen: i32, width: u32, height: u32) {
		let mut context = HashMap::new();
		let mut lock    = self.0.write().unwrap();

		// Fill implicit screen context.
		context.insert(variable("screen", "x"), 0.0);
		context.insert(variable("screen", "y"), 0.0);
		context.insert(variable("screen", "width"), width as f64);
		context.insert(variable("screen", "height"), height as f64);

		// Calculate dependency order and fill context.
		{
			let mut graph = Graph::<(&str, &str), ()>::new();
			let mut nodes = HashMap::new();
			let     root  = graph.add_node(("screen", "root"));

			for name in lock.grids.keys() {
				for &field in &["x", "y", "width", "height"] {
					let node = graph.add_node((name, field));
					nodes.insert((&**name, field), node);
					graph.add_edge(root, node, Default::default());
				}
			}

			for (name, grid) in &lock.grids {
				for &(field, value) in &[("x", &grid.x), ("y", &grid.y), ("width", &grid.width), ("height", &grid.height)] {
					let node = *nodes.get(&(&**name, field)).unwrap();

					if let Some(value) = as_number(value) {
						context.insert(variable(name, field), value);
					}
					else {
						for m in VARIABLES.captures_iter(&value) {
							// The screen is not an actual dependency.
							if &m[1] == "screen" {
								continue;
							}

							graph.add_edge(*nodes.get(&(&m[1], &m[2])).unwrap(), node, Default::default());
						}
					}
				}
			}

			// Iterate over the graph to calculate fields in the proper order.
			for node in DfsIter::new(&graph, root) {
				let (name, field) = graph[node];

				// The screen is not an actual dependency.
				if name == "screen" {
					continue;
				}

				let value = meval::eval_str_with_context(
					expression(&lock.grids, name, field).unwrap(), &context).unwrap();

				context.insert(variable(name, field), value);
			}
		}

		lock.screens.insert(screen, context);
	}

	pub fn get<T: AsRef<str>>(&self, screen: i32, value: T) -> Option<u32> {
		self.0.read().unwrap().screens.get(&screen)
			.and_then(|v| v.get(&value.as_ref().replace('.', "_")))
			.map(|&v| v.round() as u32)
	}
}

lazy_static! {
	static ref VARIABLES: Regex = Regex::new(r"(\w+)\.(\w+)").unwrap();
}

fn as_number<T: AsRef<str>>(value: T) -> Option<f64> {
	meval::eval_str(value.as_ref()).ok()
}

fn variable<T: AsRef<str>, T2: AsRef<str>>(name: T, field: T2) -> String {
	format!("{}_{}", name.as_ref().replace("-", "_"), field.as_ref())
}

fn expression<T: AsRef<str>>(table: &HashMap<String, Grid>, name: T, field: T) -> Option<String> {
	table.get(name.as_ref()).and_then(|grid|
		match field.as_ref() {
			"x"      => Some(grid.x.replace('.', "_")),
			"y"      => Some(grid.y.replace('.', "_")),
			"width"  => Some(grid.width.replace('.', "_")),
			"height" => Some(grid.height.replace('.', "_")),
			_        => None,
		})
}
