use std::str::FromStr;

use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct DataEntry<T>
{
	pub id: String,
	pub value: T,
	pub timestamp: f64,
}

impl<'de> Deserialize<'de> for DataEntry<String>
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		#[derive(Deserialize)]
		struct RawDataEntry
		{
			id: String,
			value: Value,
			timestamp: f64,
		}

		let raw = RawDataEntry::deserialize(deserializer)?;

		let string_value = match raw.value
		{
			Value::String(s) => s,
			Value::Null => "null".to_string(),
			Value::Bool(b) => b.to_string(),
			Value::Number(n) => n.to_string(),
			other => other.to_string(),
		};

		Ok(DataEntry {
			id: raw.id,
			value: string_value,
			timestamp: raw.timestamp,
		})
	}
}

impl DataEntry<String>
{
	pub fn parse<T>(&self) -> Result<DataEntry<T>, <T as FromStr>::Err>
	where
		T: FromStr,
	{
		Ok(DataEntry::<T> {
			id: self.id.clone(),
			value: self.value.parse()?,
			timestamp: self.timestamp,
		})
	}
}
