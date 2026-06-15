use crate::{
	data::{Data, MetaDta},
	data_entry::DataEntry,
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
