use std::collections::BTreeMap;

use serde_json::value::Value;

use rustc_serialize::json::ToJson;
use rustc_serialize::json::Json;

fn value_array_to_json(array: Vec<Value>) -> Json {
	let mut ret: Vec<Json> = Vec::with_capacity(array.len());
	for element in array {
		ret.push(value_to_json(element));
	}
	return ret.to_json();
}

fn value_object_to_json(object: BTreeMap<String, Value>) -> Json {
	let mut ret: BTreeMap<String, Json> = BTreeMap::new();
	for (key, value) in object {
		ret.insert(key, value_to_json(value));
	}
	return ret.to_json();
}

pub fn value_to_json(value: Value) -> Json {
	match value {
		Value::Null         => value.as_null().to_json(),
		Value::Bool(data)   => data.to_json(),
		Value::I64(data)    => data.to_json(),
		Value::U64(data)    => data.to_json(),
		Value::F64(data)    => data.to_json(),
		Value::String(data) => data.to_json(),
		Value::Array(data)  => value_array_to_json(data), // Somehow need to recursively go through and generate the array from the source
		Value::Object(data) => value_object_to_json(data), // Somehow need to recursively go through and generate the array from the source
	}
}
