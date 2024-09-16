#![warn(clippy::all, rust_2018_idioms, future_incompatible)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod app;
mod cmd;
mod filepicker;
mod groups;

// Only compile natively:
fn main() -> eframe::Result {
	let native_options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default()
			.with_transparent(true)
			.with_inner_size([700.0, 540.0])
			.with_min_inner_size([685.0, 530.0])
			.with_icon(
				eframe::icon_data::from_png_bytes(&include_bytes!("../assets/256x256.png")[..])
					.expect("Failed to load icon"),
			),
		..Default::default()
	};
	eframe::run_native(
		"Tuckr UI",
		native_options,
		Box::new(|cc| {
			// This gives us image support:
			egui_extras::install_image_loaders(&cc.egui_ctx);
			Ok(Box::new(app::TemplateApp::new(cc)))
		}),
	)
}
