use std::range::RangeInclusive;

use egui_plot::{Line, PlotPoint, Span, Text, VLine};

use crate::{
	data::{Data, MetaDta},
	data_entry::DataEntry,
	plot_data::{create_line, create_line_relative},
};

#[derive(Debug, Clone)]
pub struct FlightData
{
	pub meta: MetaDta,
	pub altitude: Vec<DataEntry<f64>>,
	pub apoapsis: Vec<DataEntry<f64>>,
	pub preiapsis: Vec<DataEntry<f64>>,
	pub eta_to_apoapsis: Vec<DataEntry<f64>>,
	pub isp: Vec<DataEntry<f64>>,
	pub thrust: Vec<DataEntry<f64>>,
	pub thrust_available: Vec<DataEntry<f64>>,
	pub stage: Vec<DataEntry<i32>>,
	pub status: Vec<DataEntry<String>>,
	pub program: Vec<DataEntry<String>>,
	pub rcs: Vec<DataEntry<bool>>,
	pub aoa: Vec<DataEntry<f64>>,
	pub pitch: Vec<DataEntry<f64>>,
	pub target_pitch: Vec<DataEntry<f64>>,
	pub twr: Vec<DataEntry<f64>>,
	pub throttle: Vec<DataEntry<f64>>,
	pub q: Vec<DataEntry<f64>>,
	pub mass: Vec<DataEntry<f64>>,
	pub airspeed: Vec<DataEntry<f64>>,
	pub orbital_speed: Vec<DataEntry<f64>>,
}

impl From<Data> for FlightData
{
	fn from(value: Data) -> Self
	{
		FlightData {
			meta: value.meta.clone(),
			altitude: value.get_data::<f64>("Altitude"),
			apoapsis: value.get_data::<f64>("Apoapsis"),
			preiapsis: value.get_data::<f64>("Airspeed"),
			eta_to_apoapsis: value.get_data::<f64>("Eta to Ap"),
			isp: value.get_data::<f64>("ISP"),
			thrust: value.get_data::<f64>("Thrust"),
			thrust_available: value.get_data::<f64>("Available Thrust"),
			stage: value.get_data::<i32>("Stage"),
			status: value.filter_data("Status"),
			program: value.filter_data("Program"),
			rcs: value.get_data::<bool>("RCS"),
			aoa: value.get_data::<f64>("AoA"),
			pitch: value.get_data::<f64>("Pitch"),
			target_pitch: value.get_data::<f64>("Target Pitch"),
			twr: value.get_data::<f64>("TWR"),
			throttle: value.get_data::<f64>("Throttle"),
			q: value.get_data::<f64>("Q"),
			mass: value.get_data::<f64>("Mass"),
			airspeed: value.get_data::<f64>("Airspeed"),
			orbital_speed: value.get_data::<f64>("Orbital Speed"),
		}
	}
}

impl FlightData
{
	pub fn normalized(&self) -> Self
	{
		Self {
			airspeed: normalize(&self.airspeed),
			mass: normalize(&self.mass),
			altitude: normalize(&self.altitude),
			apoapsis: normalize(&self.apoapsis),
			preiapsis: normalize(&self.preiapsis),
			aoa: normalize(&self.aoa),
			eta_to_apoapsis: normalize(&self.eta_to_apoapsis),
			isp: normalize(&self.isp),
			orbital_speed: normalize(&self.orbital_speed),
			pitch: normalize(&self.pitch),
			q: normalize(&self.q),
			target_pitch: normalize(&self.target_pitch),
			throttle: normalize(&self.throttle),
			thrust: normalize(&self.thrust),
			thrust_available: normalize(&self.thrust_available),
			twr: normalize(&self.twr),
			..self.clone()
		}
	}

	pub fn ascent_lines<'a>(&self) -> Vec<Line<'a>>
	{
		vec![
			create_line(&self.altitude, "Altitude"),
			create_line(&self.apoapsis, "Apoapsis"),
			create_line(&self.preiapsis, "Periapsis"),
		]
	}
	pub fn speed_lines<'a>(&self) -> Vec<Line<'a>>
	{
		vec![
			create_line(&self.airspeed, "Airspeed"),
			create_line(&self.orbital_speed, "Orbital Speed"),
		]
	}
	pub fn thrust_lines<'a>(&self) -> Vec<Line<'a>>
	{
		vec![
			create_line(&self.thrust, "Thrust"),
			create_line(&self.thrust_available, "Available Thrust"),
		]
	}
	pub fn attitude_lines<'a>(&self) -> Vec<Line<'a>>
	{
		vec![
			create_line(&self.aoa, "AoA"),
			create_line(&self.pitch, "Pitch"),
			create_line(&self.target_pitch, "Target Pitch"),
		]
	}

	pub fn ascent_lines_relative<'a>(&self) -> Vec<Line<'a>>
	{
		vec![
			create_line_relative(&self.altitude, "Altitude", self.meta.start_time),
			create_line_relative(&self.apoapsis, "Apoapsis", self.meta.start_time),
			create_line_relative(&self.preiapsis, "Periapsis", self.meta.start_time),
		]
	}
	pub fn speed_lines_relative<'a>(&self) -> Vec<Line<'a>>
	{
		vec![
			create_line_relative(&self.airspeed, "Airspeed", self.meta.start_time),
			create_line_relative(&self.orbital_speed, "Orbital Speed", self.meta.start_time),
		]
	}
	pub fn thrust_lines_relative<'a>(&self) -> Vec<Line<'a>>
	{
		vec![
			create_line_relative(&self.thrust, "Thrust", self.meta.start_time),
			create_line_relative(&self.thrust_available, "Available Thrust", self.meta.start_time),
		]
	}
	pub fn attitude_lines_relative<'a>(&self) -> Vec<Line<'a>>
	{
		vec![
			create_line_relative(&self.aoa, "AoA", self.meta.start_time),
			create_line_relative(&self.pitch, "Pitch", self.meta.start_time),
			create_line_relative(&self.target_pitch, "Target Pitch", self.meta.start_time),
		]
	}

	pub fn staging_spans(&self, relative: bool) -> Vec<Span>
	{
		create_span(
			&self.stage,
			"Stage".to_string(),
			self.meta.start_time,
			self.meta.end_time,
			relative,
		)
	}

	pub fn program_spans(&self, relative: bool) -> Vec<Span>
	{
		create_span(
			&self.program,
			"".to_string(),
			self.meta.start_time,
			self.meta.end_time,
			relative,
		)
	}

	pub fn status_spans(&self, relative: bool) -> Vec<Span>
	{
		create_span(
			&self.status,
			"".to_string(),
			self.meta.start_time,
			self.meta.end_time,
			relative,
		)
	}

	pub fn rcs_spans(&self, relative: bool) -> Vec<Span>
	{
		create_span(
			&self.rcs,
			"".to_string(),
			self.meta.start_time,
			self.meta.end_time,
			relative,
		)
	}

	pub fn max_q_line(&self, relative: bool) -> (VLine, Text)
	{
		let max = self
			.q
			.iter()
			.fold(&self.q[0], |a, b| if b.value > a.value { b } else { a });
		let t = if relative
		{
			max.timestamp - self.meta.start_time
		}
		else
		{
			max.timestamp
		};
		(
			VLine::new("Max Q", t),
			Text::new("maxq-label", PlotPoint::new(t, max.value), "Max Q"),
		)
	}
}

fn create_span<T>(data: &[DataEntry<T>], label: String, start_time: f64, end_time: f64, relative: bool) -> Vec<Span>
where
	T: ToString,
{
	let mut result = Vec::with_capacity(data.len());
	for (i, e) in data.iter().enumerate()
	{
		let name = if label.is_empty()
		{
			e.value.to_string()
		}
		else
		{
			format!("{} {}", label, e.value.to_string())
		};
		let t = match relative
		{
			true => e.timestamp - start_time,
			false => e.timestamp,
		};
		if i < data.len() - 1
		{
			let next = &data[i + 1];

			let t2 = match relative
			{
				true => next.timestamp - start_time,
				false => next.timestamp,
			};
			result.push(Span::new(name, RangeInclusive { start: t, last: t2 }));
		}
		else
		{
			let t2 = match relative
			{
				true => end_time - start_time,
				false => end_time,
			};
			result.push(Span::new(name, RangeInclusive { start: t, last: t2 }));
		}
	}
	result
}

fn normalize(data: &Vec<DataEntry<f64>>) -> Vec<DataEntry<f64>>
{
	let (min, max) = get_range(data);
	let m = max - min;
	data.iter()
		.map(|e| DataEntry {
			value: (e.value - min) / m,
			..e.clone()
		})
		.collect()
}

fn get_range(data: &Vec<DataEntry<f64>>) -> (f64, f64)
{
	let mut min = f64::MAX;
	let mut max = f64::MIN;
	for entry in data
	{
		if entry.value > max
		{
			max = entry.value;
		}
		if entry.value < min
		{
			min = entry.value;
		}
	}
	(min, max)
}
