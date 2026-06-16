use std::{fs::File, io::BufReader, str::FromStr};

use crate::data_entry::DataEntry;

#[derive(Debug, Clone)]
pub struct Data
{
	pub raw: Vec<DataEntry<String>>,
	pub meta: MetaDta,
}

#[derive(Debug, Default, Clone)]
pub struct MetaDta
{
	pub vessel_name: String,
	pub vessel_type: String,
	pub body: String,
	pub start_time: f64,
	pub end_time: f64,
}

impl Data
{
	pub fn get_data<T>(&self, id: &str) -> Vec<DataEntry<T>>
	where
		T: FromStr,
	{
		let entries = self.filter_data(id);
		let mut result = Vec::with_capacity(entries.len());
		for entry in entries
		{
			match entry.parse::<T>()
			{
				Ok(val) =>
				{
					result.push(val);
				}
				Err(_err) =>
				{
					println!("Failed to parse value");
				}
			};
		}
		result
	}

	pub fn filter_data(&self, id: &str) -> Vec<DataEntry<String>>
	{
		self.raw.iter().filter(|e| e.id == id).cloned().collect()
	}

	pub fn parse_metadata(mut self) -> Self
	{
		if let Some(vessel_name) = self.raw.iter().find(|e| e.id == "Vessel")
		{
			self.meta.vessel_name = vessel_name.value.clone();
		}
		if let Some(vessel_type) = self.raw.iter().find(|e| e.id == "Vessel Type")
		{
			self.meta.vessel_type = vessel_type.value.clone();
		}
		if let Some(body) = self.raw.iter().find(|e| e.id == "Body")
		{
			self.meta.body = body.value.clone();
		}
		self.meta.start_time = self
			.raw
			.iter()
			.fold(f64::MAX, |a, b| if b.timestamp < a { b.timestamp } else { a });

		self.meta.end_time = self
			.raw
			.iter()
			.fold(0.0, |a, b| if b.timestamp > a { b.timestamp } else { a });
		self
	}

	pub fn load_file<P>(path: P) -> Result<Data, String>
	where
		P: AsRef<std::path::Path>,
	{
		let file = File::open(path).map_err(|err| err.to_string())?;
		let reader = BufReader::new(file);
		let data = serde_json::from_reader(reader).map_err(|err| err.to_string())?;
		Ok(Data {
			raw: data,
			meta: Default::default(),
		}
		.parse_metadata())
	}

	pub fn load_file_raw(bytes: &[u8]) -> Result<Data, String>
	{
		let data = serde_json::from_slice(bytes).map_err(|err| err.to_string())?;
		Ok(Data {
			raw: data,
			meta: Default::default(),
		}
		.parse_metadata())
	}
}
