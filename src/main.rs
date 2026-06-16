mod data;
mod data_entry;
mod flight_data;
mod madoi;
mod plot_data;

use egui::Vec2;

use crate::madoi::Madoi;

fn main()
{
	let native_options = eframe::NativeOptions {
		persist_window: false,
		window_builder: Some(Box::new(|b| {
			b.with_inner_size(Vec2::new(1920.0, 1080.0))
				.with_clamp_size_to_monitor_size(true)
				.with_drag_and_drop(true)
		})),
		..Default::default()
	};
	eframe::run_native("Madoi", native_options, Box::new(|cc| Ok(Box::new(Madoi::new(cc)))))
		.expect("Failed to create app");
}
