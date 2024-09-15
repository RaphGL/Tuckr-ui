#![warn(clippy::all, rust_2018_idioms, future_incompatible)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod app;
mod cmd;
mod groups;
mod filepicker;

// When compiling natively:
fn main() -> eframe::Result {
	let native_options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default()
			.with_transparent(true)
			.with_inner_size([500.0, 460.0])
			.with_min_inner_size([500.0, 420.0])
			.with_icon(
				// NOTE: Adding an icon is optional
				eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
					.expect("Failed to load icon"),
			),
		..Default::default()
	};
	eframe::run_native(
		"eframe template",
		native_options,
		Box::new(|cc|{
			// This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
			Ok(Box::new(app::TemplateApp::new(cc)))
		}),
	)
}
