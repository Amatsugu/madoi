use std::path::PathBuf;

use eframe::egui;
use egui::{Id, Ui, Vec2, Vec2b, WidgetText, vec2};
use egui_plot::{Corner, Legend, Line, Plot, Span};

use crate::{data::Data, flight_data::FlightData, plot_data::PlotData};

#[derive(Default)]
pub struct Madoi
{
	data: Option<Data>,
	flight_data: Option<FlightData>,
	flight_data_normalized: Option<FlightData>,
	plot_data: Option<PlotData<'static>>,
	section: SpanDisplay,
	display: DisplayMode,
	time_mode: TimeMode,
}

impl Madoi
{
	pub fn new(_cc: &eframe::CreationContext<'_>) -> Self
	{
		#[cfg(feature = "dev")]
		return Self::default().with_data(&PathBuf::from(
			"Q:/KSP/Script/data/mct/Year 30 Day 136 5hr - Anna 1.json",
		));
		#[cfg(not(feature = "dev"))]
		return Self::default();
	}

	fn with_data(mut self, path: &PathBuf) -> Self
	{
		self.load_data(path);
		self
	}

	fn load_data(&mut self, path: &PathBuf)
	{
		if let Ok(loaded) = Data::load_file(path)
		{
			self.data = Some(loaded.clone());
			let flight_data: FlightData = loaded.into();
			self.plot_data = Some((&flight_data).into());
			self.flight_data_normalized = Some(flight_data.normalized());
			self.flight_data = Some(flight_data);
		}
		else
		{
			println!("Failed to load data")
		}
	}
}

const HEADER_HEIGHT: f32 = 24.0;
const SIDEBAR_WIDTH: f32 = 200.0;

#[derive(PartialEq, Debug, Default)]
enum SpanDisplay
{
	#[default]
	Staging,
	Status,
	Program,
	Rcs,
}

#[derive(PartialEq, Debug, Default)]
enum DisplayMode
{
	#[default]
	Absolute,
	Normalized,
}

#[derive(PartialEq, Debug, Default)]
enum TimeMode
{
	#[default]
	Ut,
	Elapsed,
}

impl eframe::App for Madoi
{
	fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame)
	{
		ui.input(|i| {
			if let Some(file) = i.raw.dropped_files.first()
				&& let Some(path) = &file.path
			{
				self.load_data(path);
			}
		});
		let data_src = match self.display
		{
			DisplayMode::Absolute => &self.flight_data,
			DisplayMode::Normalized => &self.flight_data_normalized,
		};
		let Some(data) = data_src
		else
		{
			return;
		};
		egui::Panel::left("sidebar")
			.default_size(SIDEBAR_WIDTH)
			.show_inside(ui, |ui| {
				ui.take_available_width();
				ui.vertical(|ui| {
					ui.heading("Sections");
					ui.radio_value(&mut self.section, SpanDisplay::Staging, "Staging");
					ui.radio_value(&mut self.section, SpanDisplay::Status, "Status");
					ui.radio_value(&mut self.section, SpanDisplay::Program, "Program");
					ui.radio_value(&mut self.section, SpanDisplay::Rcs, "RCS");
					ui.heading("Time Mode");
					ui.radio_value(&mut self.time_mode, TimeMode::Ut, "UT");
					ui.radio_value(&mut self.time_mode, TimeMode::Elapsed, "Elapsed");
					ui.heading("Display Mode");
					ui.radio_value(&mut self.display, DisplayMode::Absolute, "Absolute");
					ui.radio_value(&mut self.display, DisplayMode::Normalized, "Normalized");
				});
			});
		egui::Panel::top("titlebar")
			.default_size(HEADER_HEIGHT)
			.show_inside(ui, |ui| {
				ui.heading(format!("Flight Data - {} at {}", data.meta.vessel_name, data.meta.body));
			});

		egui::CentralPanel::default_margins().show_inside(ui, |ui| {
			ui.take_available_width();
			let screen = ui.available_size();
			ui.vertical(|ui| {
				ui.horizontal(|ui| {
					//Graphs
					let graph_size = vec2(screen.x, screen.y);
					let cell_size = graph_size / 2.0;
					let spans = get_spans(&self.section, data, &self.display);
					ui.vertical(|ui| {
						ui.columns(2, |ui| {
							let ascent_lines = match self.time_mode
							{
								TimeMode::Ut => data.ascent_lines(),
								TimeMode::Elapsed => data.ascent_lines_relative(),
							};
							add_graph(
								"Ascent",
								cell_size,
								&mut ui[0],
								GraphData {
									lines: ascent_lines,
									spans: spans.clone(),
								},
								GraphDisplay {
									x_label: "Time",
									y_label: "Altitude",
									legend_corner: Corner::LeftTop,
								},
							);
							let speed_lines = match self.time_mode
							{
								TimeMode::Ut => data.speed_lines(),
								TimeMode::Elapsed => data.speed_lines_relative(),
							};
							add_graph(
								"Speed",
								cell_size,
								&mut ui[1],
								GraphData {
									lines: speed_lines,
									spans: spans.clone(),
								},
								GraphDisplay {
									x_label: "Time",
									y_label: "Speed",
									legend_corner: Corner::LeftTop,
								},
							);
						});
						ui.columns(2, |ui| {
							let attitude_lines = match self.time_mode
							{
								TimeMode::Ut => data.attitude_lines(),
								TimeMode::Elapsed => data.attitude_lines_relative(),
							};
							add_graph(
								"Attitude",
								cell_size,
								&mut ui[0],
								GraphData {
									lines: attitude_lines,
									spans: spans.clone(),
								},
								GraphDisplay {
									x_label: "Time",
									y_label: "Degrees",
									legend_corner: Corner::RightTop,
								},
							);
							let thrust_lines = match self.time_mode
							{
								TimeMode::Ut => data.thrust_lines(),
								TimeMode::Elapsed => data.thrust_lines_relative(),
							};
							add_graph(
								"Thrust",
								cell_size,
								&mut ui[1],
								GraphData {
									lines: thrust_lines,
									spans: spans.clone(),
								},
								GraphDisplay {
									x_label: "Time",
									y_label: "Thrust",
									legend_corner: Corner::RightTop,
								},
							);
						});
					});
				});
			});
		});
	}
}

fn get_spans(section: &SpanDisplay, data: &FlightData, display: &DisplayMode) -> Vec<Span>
{
	let relative = match display
	{
		DisplayMode::Absolute => false,
		DisplayMode::Normalized => true,
	};
	match section
	{
		SpanDisplay::Staging => data.staging_spans(relative),
		SpanDisplay::Status => data.status_spans(relative),
		SpanDisplay::Program => data.program_spans(relative),
		SpanDisplay::Rcs => data.rcs_spans(relative),
	}
}

#[derive(Default)]
struct GraphData<'a>
{
	lines: Vec<Line<'a>>,
	spans: Vec<Span>,
}

struct GraphDisplay<T>
where
	T: Into<egui::WidgetText>,
{
	x_label: T,
	y_label: T,
	legend_corner: Corner,
}

fn add_graph(
	name: impl std::hash::Hash,
	cell_size: Vec2,
	ui: &mut Ui,
	data: GraphData,
	display: GraphDisplay<impl Into<WidgetText>>,
)
{
	ui.allocate_ui(cell_size, |ui| {
		Plot::new(name)
			.link_cursor(Id::new("main"), Vec2b::new(true, true))
			.link_axis(Id::new("main"), Vec2b::new(true, true))
			.x_axis_label(display.x_label)
			.y_axis_label(display.y_label)
			.legend(
				Legend::default()
					.position(display.legend_corner)
					.follow_insertion_order(true),
			)
			.label_formatter(|name, value| {
				if name.is_empty()
				{
					"".to_owned()
				}
				else
				{
					format!("{} {:.*?}", name, 1, value.y)
				}
			})
			.show(ui, |plot_ui| {
				for span in data.spans
				{
					plot_ui.span(span);
				}
				for line in data.lines
				{
					plot_ui.line(line);
				}
			});
	});
}
