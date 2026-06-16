use egui_plot::{Line, PlotPoints};

use crate::{data_entry::DataEntry, flight_data::FlightData};

#[derive(Default)]
pub struct PlotData<'a>
{
	pub lines: Vec<Line<'a>>,
}

impl From<&FlightData> for PlotData<'static>
{
	fn from(value: &FlightData) -> Self
	{
		let mut data = Self { ..Self::default() };

		data.lines.push(create_line(&value.altitude, "Altitude"));
		data.lines.push(create_line(&value.apoapsis, "Apoapsis"));
		data.lines.push(create_line(&value.preiapsis, "Periapsis"));
		data.lines.push(create_line(&value.airspeed, "Airspeed"));
		data.lines.push(create_line(&value.aoa, "AoA"));
		data.lines.push(create_line(&value.mass, "Mass"));
		data.lines.push(create_line(&value.eta_to_apoapsis, "Eta To Apo"));
		data.lines.push(create_line(&value.target_pitch, "Steering"));
		data.lines.push(create_line(&value.pitch, "Pitch"));
		data.lines.push(create_line(&value.isp, "ISP"));
		data.lines.push(create_line(&value.thrust, "Thrust"));
		data.lines
			.push(create_line(&value.thrust_available, "Thrust Available"));
		data.lines.push(create_line(&value.twr, "TWR"));
		data.lines.push(create_line(&value.q, "Q"));
		data.lines.push(create_line(&value.orbital_speed, "Orbital Speed"));
		data
	}
}

pub fn create_line(data: &[DataEntry<f64>], name: impl Into<String>) -> Line<'static>
{
	let points: PlotPoints = data.iter().map(|e| [e.timestamp, e.value]).collect();
	Line::new(name, points)
}

pub fn create_line_relative(data: &[DataEntry<f64>], name: impl Into<String>, start_time: f64) -> Line<'static>
{
	let points: PlotPoints = data.iter().map(|e| [e.timestamp - start_time, e.value]).collect();
	Line::new(name, points)
}
