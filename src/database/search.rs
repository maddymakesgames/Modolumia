use serde::ser::{SerializeMap};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use serde_json::map::Entry;

use crate::database::Document;
use crate::database::DocumentType;


#[derive(Deserialize, Debug, Serialize)]
pub struct SearchResult<S> where S: DocumentType {
	#[serde(bound(deserialize = "Document<S>: Deserialize<'de>"))]
	pub docs: Option<Vec<Document<S>>>,
	pub bookmark: Option<String>,
	pub warning: Option<String>,
	pub execution_stats: Option<ExecutionStats> 
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ExecutionStats {
	pub total_keys_examined: u16,
	pub total_docs_examined: u16,
	pub total_quorum_docs_examined: u16,
	pub results_returned: u16,
	pub execution_time_ms: f32
}

pub struct SearchBuilder {
	curr_search: SearchInfo
}

impl SearchBuilder {
	pub fn new() -> Self {
		SearchBuilder {
			curr_search: SearchInfo::new()
		}
	}

	pub fn build(self) -> SearchInfo {
		self.curr_search
	}

	pub fn filter(mut self, term: SearchTerm) -> Self {
		self.curr_search.selector = term;
		self
	}

	pub fn limit(mut self, limit: u32) -> Self {
		self.curr_search.limit = Some(limit);
		self
	}

	pub fn skip(mut self, skip: u32) -> Self {
		self.curr_search.skip = Some(skip);
		self
	}

	pub fn sort(mut self, sort: Vec<SortTerm>) -> Self {
		self.curr_search.sort = Some(sort);
		self
	}

	pub fn fields(mut self, fields: Vec<String>) -> Self {
		self.curr_search.fields = Some(fields);
		self
	}

	pub fn r(mut self, r: u32) -> Self {
		self.curr_search.r = Some(r);
		self
	}

	pub fn bookmark(mut self, bookmark: String) -> Self {
		self.curr_search.bookmark = Some(bookmark);
		self
	}

	pub fn update(mut self, update: bool) -> Self {
		self.curr_search.update = Some(update);
		self
	}

	pub fn stable(mut self, stable: bool) -> Self {
		self.curr_search.stable = Some(stable);
		self
	}

	pub fn stale(mut self, stale: bool) -> Self {
		self.curr_search.stale = Some(stale);
		self
	}

	pub fn execution_stats(mut self, execution_stats: bool) -> Self {
		self.curr_search.execution_stats = Some(execution_stats);
		self
	}
}
#[derive(Serialize, Debug)]
pub struct SearchInfo {
	selector: SearchTerm,
	#[serde(skip_serializing_if = "Option::is_none")]
	limit: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	skip: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	sort: Option<Vec<SortTerm>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	fields: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	r: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	bookmark: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	update: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	stable: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	stale: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	execution_stats: Option<bool>
}

impl SearchInfo {
	pub(super) fn new() -> Self {
		SearchInfo {
			selector: SearchTerm::null(),
			limit: None,
			skip: None,
			sort: None,
			fields: None,
			r: None,
			bookmark: None,
			update: None,
			stable: None,
			stale: None,
			execution_stats: None
		}
	}
}

#[derive(Debug)]
pub struct SortTerm {
	asc: bool,
	property: String
}

impl SortTerm {
	/// Creates a new SortTerm sorting by ascending values of prop
	pub fn ascending(prop: String) -> Self {
		SortTerm {
			asc: true,
			property: prop
		}
	}

	/// Create a new SortTerm sorting by descending values of prop
	pub fn descending(prop: String) -> Self {
		SortTerm {
			asc: false,
			property: prop
		}
	}
}

impl Serialize for SortTerm {
    fn serialize<S : serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		let mut s_map = serializer.serialize_map(Some(1))?;
		s_map.serialize_entry(&self.property, if self.asc {"asc"} else {"desc"})?;
		s_map.end()
    }
}

pub fn val_to_str(val: &Value) -> String {
	match val {
		Value::String(str) => str.clone(),
		Value::Bool(bo) => bo.to_string(),
		Value::Number(n) => n.to_string(),
		Value::Array(arr) => arr.iter().map(|v| v.to_string().clone()).collect(),
		Value::Null => "Null".to_owned(),
		Value::Object(_) => panic!("attempting to convert Value::Object to a string")
	}
}

#[derive(Clone, Debug)]
pub struct SearchTerm {
	children: Option<Vec<SearchTerm>>,
	value: Value,
	is_arr: bool
}

impl Serialize for SearchTerm {
	fn serialize<S : serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		match &self.children {
			Some(children) => {
				if self.is_arr {
					let mut vec = serializer.serialize_map(Some(children.len()))?;
					vec.serialize_entry(&self.value, &children)?;
					vec.end()
				} else {
					match &self.children {
						Some(children) => {
							let mut map =  serde_json::Map::new();
							if children.len() == 1 {
								map.insert(val_to_str(&self.value), children.get(0).unwrap().gen_object());
							} else {
								map.insert(val_to_str(&self.value), self.gen_object());
							}

							map.serialize(serializer)
						},
						None => self.value.serialize(serializer)
					}
				}
			},
			None => self.value.serialize(serializer)
		}
	}
}


impl SearchTerm {
	fn gen_object(&self) -> Value {
		match &self.children {
			Some(children) => {
				if self.is_arr {
					let mut vec: Vec<Value> = Vec::new();
					for child in children {
						vec.push(child.gen_object())
					}
					return Value::Array(vec);
				}

				let mut map = serde_json::Map::new();
				for child in children {
					let mut child_obj = child.gen_object();
					match &mut child_obj {
						Value::Object(child_map) =>
							match child_map.entry(val_to_str(&child.value)) {
								Entry::Occupied(entry) => map.insert(entry.key().clone(), entry.get().clone()),
								Entry::Vacant(_) => map.insert(val_to_str(&child.value), child_obj)
							},
						_ => map.insert(val_to_str(&self.value), child_obj)
					};
				}
				Value::Object(map)
			},
			None => self.value.clone()
		}
	}
}

macro_rules! st_builder {
	($fnname: ident, $name: expr, $array: expr, $doc: expr) => {
		#[doc($doc)]
		pub fn $fnname() -> Self {
		SearchTerm {
				children: None,
				is_arr: $array,
				value: Value::String($name.to_owned())
			}
		}
	};
}

impl SearchTerm {
	/// Add a child to the current search term
	pub fn child(mut self, value: SearchTerm) -> Self {
		match &mut self.children {
			Some(children) => {
				children.push(value);
			},
			None => {
				self.children = Some(vec![value]);
			}
		}

		self
	}

	/// A string key value pair
	pub fn pair(key: &str, value: &str) -> Self {
		let value_term = SearchTerm {
			children: None,
			is_arr: false,
			value: Value::String(value.to_owned())
		};

		SearchTerm {
			children: Some(vec![value_term]),
			is_arr: false,
			value: Value::String(key.to_owned())
		}
	}

	// Value types

	/// A key / string value
	pub fn string(val: &str) -> Self {
		SearchTerm {
			children: None,
			is_arr: false,
			value: Value::String(val.to_owned())
		}
	}

	/// A integer value
	pub fn int(val: u64) -> Self {
		SearchTerm {
			children: None,
			is_arr: false,
			value: Value::Number(serde_json::value::Number::from(val))
		}
	}

	/// A float value
	pub fn float(val: f64) -> Self {
		SearchTerm {
			children: None,
			is_arr: false,
			value: Value::Number(serde_json::value::Number::from_f64(val).expect("float has to be finite idiot"))
		}
	}

	/// A null
	pub fn null() -> Self {
		SearchTerm {
			children: None,
			is_arr: false,
			value: Value::Null
		}
	}

	// Meta-Conditionals
	st_builder!(and, "$and", true, "all conditions match");
	st_builder!(or, "$or", true, "any of the conditions match");
	st_builder!(nor, "$nor", true, "none of conditions match");
	st_builder!(all, "$all", true, "array contains all values");
	st_builder!(not, "$not", false, "condition is not true");



	// Value Conditionals
	st_builder!(lt, "$lt", false, "value is less than");
	st_builder!(lte, "$lte", false, "value is less than or equal to");
	st_builder!(eq, "$eq", false, "value is equal to");
	st_builder!(ne, "$ne", false, "value is not equal to");
	st_builder!(gt, "$gt", false, "value is greator than");
	st_builder!(gte, "$gte", false, "value is greator than or equal to");
	st_builder!(r#in, "$in", true, "value is in the provided array");
	st_builder!(nin, "$nin", true, "value is not in the provided array");
	st_builder!(size, "$size", false, "array is of specified size");
	st_builder!(r#mod, "$mod", true, "requires a [Divisor, Remaindor]\nvalue matches remaindor");
	st_builder!(regex, "$regex", false, "string matches regex\nregex used is ERLANG regex");
}