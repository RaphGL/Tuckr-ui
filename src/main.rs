#![warn(clippy::all, rust_2018_idioms, future_incompatible)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod app;
mod cmd;
mod filepicker;
mod groups;

// When compiling natively:
fn main() -> eframe::Result {
	let native_options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default()
			.with_transparent(true)
			.with_inner_size([600.0, 540.0])
			.with_min_inner_size([590.0, 520.0])
			.with_icon(
				// Icon
				eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
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
