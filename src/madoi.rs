use std::{path::PathBuf, sync::Arc};

use eframe::egui;
use egui::{Id, Ui, Vec2, Vec2b, WidgetText, vec2};
use egui_plot::{Corner, HLine, Legend, Line, Plot, PlotPoint, Span, Text, VLine};

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
	load_error: Option<String>,
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

	#[cfg(feature = "dev")]
	fn with_data(mut self, path: &PathBuf) -> Self
	{
		self.load_data(path);
		self
	}

	fn load_data(&mut self, path: &PathBuf)
	{
		self.reset_data();
		match Data::load_file(path)
		{
			Ok(loaded) =>
			{
				self.data = Some(loaded.clone());
				let flight_data: FlightData = loaded.into();
				self.plot_data = Some((&flight_data).into());
				self.flight_data_normalized = Some(flight_data.normalized());
				self.flight_data = Some(flight_data);
			}
			Err(err) =>
			{
				self.load_error = Some(err);
			}
		}
	}

	fn load_data_raw(&mut self, bytes: &Arc<[u8]>)
	{
		self.reset_data();
		match Data::load_file_raw(bytes.iter().as_slice())
		{
			Ok(loaded) =>
			{
				self.data = Some(loaded.clone());
				let flight_data: FlightData = loaded.into();
				self.plot_data = Some((&flight_data).into());
				self.flight_data_normalized = Some(flight_data.normalized());
				self.flight_data = Some(flight_data);
			}
			Err(err) =>
			{
				self.load_error = Some(err);
			}
		}
	}

	fn reset_data(&mut self)
	{
		self.load_error = None;
		self.data = None;
		self.flight_data = None;
		self.flight_data_normalized = None;
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

#[derive(PartialEq, Debug, Default, Clone, Copy)]
enum TimeMode
{
	Ut,
	#[default]
	Elapsed,
}

impl From<TimeMode> for bool
{
	fn from(value: TimeMode) -> Self
	{
		match value
		{
			TimeMode::Ut => false,
			TimeMode::Elapsed => true,
		}
	}
}

impl eframe::App for Madoi
{
	fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame)
	{
		ui.input(|i| {
			if let Some(file) = i.raw.dropped_files.first()
			{
				if let Some(path) = &file.path
				{
					self.load_data(path);
				}
				else if let Some(bytes) = &file.bytes
				{
					self.load_data_raw(bytes);
				}
			}
		});
		let data_src = match self.display
		{
			DisplayMode::Absolute => &self.flight_data,
			DisplayMode::Normalized => &self.flight_data_normalized,
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
				if let Some(data) = data_src
				{
					ui.heading(format!("Flight Data - {} at {}", data.meta.vessel_name, data.meta.body));
				}
				else
				{
					ui.heading("No data Loaded");
				}
			});

		egui::CentralPanel::default_margins().show_inside(ui, |ui| {
			let Some(data) = data_src
			else
			{
				ui.vertical_centered_justified(|ui| {
					ui.heading("Drop a file here to start visualization!");
					if let Some(err_msg) = &self.load_error
					{
						ui.label("Failed to load file");
						ui.label(err_msg.to_string());
					}
				});
				return;
			};
			let Some(_flight_data) = &self.flight_data
			else
			{
				return;
			};
			ui.take_available_width();
			let screen = ui.available_size();
			ui.vertical(|ui| {
				ui.horizontal(|ui| {
					//Graphs
					let graph_size = vec2(screen.x, screen.y);
					let cell_size = graph_size / 2.0;
					let spans = get_spans(&self.section, data, self.time_mode.into());
					let max_q = data.max_q_line(self.time_mode.into());
					ui.vertical(|ui| {
						ui.columns(2, |ui| {
							let ascent_lines = match self.time_mode
							{
								TimeMode::Ut => data.ascent_lines(),
								TimeMode::Elapsed => data.ascent_lines_relative(),
							};
							add_graph(
								"Ascent",
								&mut ui[0],
								GraphData {
									lines: ascent_lines,
									spans: spans.clone(),
									v_lines: vec![max_q.clone()],
									..Default::default()
								},
								GraphDisplay {
									x_label: "Time",
									y_label: "Altitude",
									legend_corner: Corner::LeftTop,
									cell_size,
									tooltip_builder: None::<fn(&mut Ui, PlotPoint)>,
								},
							);
							let speed_lines = match self.time_mode
							{
								TimeMode::Ut => data.speed_lines(),
								TimeMode::Elapsed => data.speed_lines_relative(),
							};
							add_graph(
								"Speed",
								&mut ui[1],
								GraphData {
									lines: speed_lines,
									spans: spans.clone(),
									v_lines: vec![max_q.clone()],
									..Default::default()
								},
								GraphDisplay {
									x_label: "Time",
									y_label: "Speed",
									legend_corner: Corner::LeftTop,
									cell_size,
									tooltip_builder: None::<fn(&mut Ui, PlotPoint)>,
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
								&mut ui[0],
								GraphData {
									lines: attitude_lines,
									spans: spans.clone(),
									v_lines: vec![max_q.clone()],
									..Default::default()
								},
								GraphDisplay {
									x_label: "Time",
									y_label: "Degrees",
									cell_size,
									legend_corner: Corner::RightTop,
									tooltip_builder: None::<fn(&mut Ui, PlotPoint)>,
								},
							);
							let thrust_lines = match self.time_mode
							{
								TimeMode::Ut => data.thrust_lines(),
								TimeMode::Elapsed => data.thrust_lines_relative(),
							};
							add_graph(
								"Thrust",
								&mut ui[1],
								GraphData {
									lines: thrust_lines,
									spans: spans.clone(),
									v_lines: vec![max_q.clone()],
									..Default::default()
								},
								GraphDisplay {
									cell_size,
									x_label: "Time",
									y_label: "Thrust",
									legend_corner: Corner::RightTop,
									tooltip_builder: None::<fn(&mut Ui, PlotPoint)>,
								},
							);
						});
					});
				});
			});
		});
	}
}

fn get_spans(section: &SpanDisplay, data: &FlightData, relative: bool) -> Vec<Span>
{
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
	h_lines: Vec<(HLine, Text)>,
	v_lines: Vec<(VLine, Text)>,
	spans: Vec<Span>,
}

struct GraphDisplay<T, F>
where
	T: Into<egui::WidgetText>,
	F: FnMut(&mut Ui, PlotPoint),
{
	x_label: T,
	y_label: T,
	legend_corner: Corner,
	cell_size: Vec2,
	tooltip_builder: Option<F>,
}

fn add_graph(
	name: impl std::hash::Hash,
	ui: &mut Ui,
	data: GraphData,
	display: GraphDisplay<impl Into<WidgetText>, impl FnMut(&mut Ui, PlotPoint)>,
)
{
	ui.allocate_ui(display.cell_size, |ui| {
		let _response = Plot::new(name)
			.link_cursor(Id::new("main"), Vec2b::new(true, true))
			.link_axis(Id::new("main"), Vec2b::new(true, false))
			.x_axis_label(display.x_label)
			.y_axis_label(display.y_label)
			.grid_fade(1.0)
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
				for (hline, text) in data.h_lines
				{
					plot_ui.hline(hline);
					plot_ui.text(text);
				}
				for (vline, text) in data.v_lines
				{
					plot_ui.vline(vline);
					plot_ui.text(text);
				}
				for line in data.lines
				{
					plot_ui.line(line);
				}
				if let Some(m_pos) = plot_ui.response().hover_pos()
				{
					let plot_pos = plot_ui.plot_from_screen(m_pos);
					plot_ui.response().clone().on_hover_ui_at_pointer(|ui| {
						if let Some(mut builder) = display.tooltip_builder
						{
							builder(ui, plot_pos);
						}
					});
				}
			});
	});
}

#[allow(dead_code, unused_variables)]
fn build_tooltip(ui: &mut Ui, data: &FlightData, plot_pos: PlotPoint)
{
	todo!()
}
