mod data;
mod data_entry;
mod flight_data;

use std::{f32::consts::FRAC_PI_3, path::PathBuf};

use nannou::{color::IntoLinSrgba, draw::properties::ColorScalar, prelude::*};

use crate::{
	data::{Data, MetaDta},
	data_entry::DataEntry,
	flight_data::FlightData,
};

#[derive(Debug)]
struct Model
{
	_window: WindowId,
	res: Vec2,
	data: Option<Data>,
	flight_data: Option<FlightData>,
}

fn main()
{
	nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model
{
	let win = app
		.new_window()
		.resizable(true)
		.resized(resized)
		.title("Madoi")
		.dropped_file(file_dropped)
		.clear_color(rgb8(10, 10, 10))
		.size(1920, 1080)
		.view(view)
		.build()
		.expect("Failed to create window");
	let mut model = Model {
		_window: win,
		data: None,
		res: Vec2::new(1920.0, 1080.0),
		flight_data: None,
	};
	#[cfg(feature = "dev")]
	{
		if let Ok(result) = Data::load_file("Q:/KSP/Script/data/mct/Year 25 Day 313 5hr - Rukako 1.json")
		{
			model.data = Some(result.clone());
			model.flight_data = Some(result.into());
		}
	}
	model
}

fn file_dropped(_app: &App, model: &mut Model, path: PathBuf)
{
	match Data::load_file(path)
	{
		Ok(result) =>
		{
			model.data = Some(result.clone());
			model.flight_data = Some(result.into());
			println!("File parsed");
		}
		Err(err) =>
		{
			println!("{:?}", err);
		}
	}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame)
{
	let draw = app.draw();
	draw.background().color(BLACK);
	match &model.flight_data
	{
		Some(data) =>
		{
			draw_graph(&draw, &data, model.res);
		}
		None =>
		{
			draw.text("No Data loaded").color(BLUE);
		}
	}

	draw.to_frame(app, &frame).unwrap();
}

fn draw_graph(draw: &Draw, flight_data: &FlightData, res: Vec2)
{
	let rect = Rect::from_xy_wh(Vec2::ZERO, res - Vec2::splat(20.0));
	draw_points(rect, draw, &flight_data.pitch, &flight_data.meta, RED);
	draw_points(rect, draw, &flight_data.target_pitch, &flight_data.meta, PALEVIOLETRED);
	draw_points(rect, draw, &flight_data.apoapsis, &flight_data.meta, GREEN);
	draw_points(rect, draw, &flight_data.thrust, &flight_data.meta, DARKORANGE);
	draw_points(rect, draw, &flight_data.preiapsis, &flight_data.meta, ORANGE);
	draw_points(rect, draw, &flight_data.altitude, &flight_data.meta, YELLOWGREEN);
	draw_points(rect, draw, &flight_data.airspeed, &flight_data.meta, PINK);
	draw_sections(rect, draw, &flight_data.stage, &flight_data.meta, GRAY);
	draw_sections(rect, draw, &flight_data.program, &flight_data.meta, DARKCYAN);
	draw_sections(rect, draw, &flight_data.status, &flight_data.meta, DARKGREEN);
}

fn draw_sections<C, T>(bounds: Rect, draw: &Draw, points: &Vec<DataEntry<T>>, meta: &MetaDta, color: C)
where
	C: IntoLinSrgba<ColorScalar> + Clone,
{
	let duration = (meta.end_time - meta.start_time) as f32;
	for point in points
	{
		let t = (point.timestamp - meta.start_time) as f32;
		let scaled_time = map_range(t, 0.0, duration, bounds.left(), bounds.right());
		draw.line()
			.start(Vec2::new(scaled_time, bounds.bottom()))
			.end(Vec2::new(scaled_time, bounds.top()))
			.color(color.clone());
	}
}
fn draw_points<C>(bounds: Rect, draw: &Draw, points: &Vec<DataEntry<f64>>, meta: &MetaDta, color: C)
where
	C: IntoLinSrgba<ColorScalar> + Clone,
{
	let (min, max) = get_range(points);
	let duration = (meta.end_time - meta.start_time) as f32;
	for point in points
	{
		let t = (point.timestamp - meta.start_time) as f32;
		let scaled_time = map_range(t, 0.0, duration, bounds.left(), bounds.right());
		let scaled_value = map_range_64(point.value, min, max, bounds.bottom() as f64, bounds.top() as f64) as f32;
		// draw.text(format!("{:?}", point.value).as_str())
		// 	.x_y(scaled_time, scaled_value - 10.0)
		// 	.rotate(FRAC_PI_3);
		draw.ellipse()
			.x_y(scaled_time, scaled_value)
			.radius(2.0)
			.color(color.clone());
	}
}

fn map_range(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32
{
	out_min + (out_max - out_min) * ((value - in_min) / (in_max - in_min))
}

fn map_range_64(value: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64
{
	out_min + (out_max - out_min) * ((value - in_min) / (in_max - in_min))
}

fn get_range(data: &Vec<DataEntry<f64>>) -> (f64, f64)
{
	let mut min = f64::MAX;
	let mut max = f64::MIN;
	for e in data
	{
		if e.value < min
		{
			min = e.value;
		}
		if e.value > max
		{
			max = e.value;
		}
	}
	(min, max)
}
fn resized(_app: &App, model: &mut Model, size: Vec2)
{
	model.res = size;
}
