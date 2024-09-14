use crate::cmd::run;
use egui::{Color32, Ui};
use egui::{Context, Image, Modifiers, KeyboardShortcut, include_image};
use std::fmt::Display;
use std::process::ExitCode;
use std::thread::{self, JoinHandle};
use tuckr::dotfiles::{Dotfile, ReturnCode};
use egui_file::FileDialog;
use std::{ffi::OsStr, path::{Path, PathBuf}};
/// the tuckr state
use tuckr::Cli;

#[derive(thiserror::Error, Debug)]
pub enum CmdError {
	Help,
	Io(#[from] std::io::Error),
}

impl Display for CmdError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			CmdError::Io(e) => write!(f, "Io Error: {}", e),
			_ => write!(f, "See Help"),
		}
	}
}
// todo use new thred to check dotfiles

/// enum for each page/command
#[derive(Default, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
pub enum Page {
	/// help page
	#[default]
	Help,
	/// the simlink status
	Status,
	/// (exclude, force, adopt)
	Add(Option<Vec<String>>, bool, bool),
	/// exclude
	Rm(Option<Vec<String>>),
	/// (exclude, force, adopt)
	Set(Option<Vec<String>>, bool, bool),
	/// files
	Push(Vec<String>),
	Pop,
	Init,
	/// create and edit hooks
	Hooks,
}

impl Page {
	/// Prepare for runing a command, push will only use the first group\
	/// `\*` is all groups, None if page is help
	pub fn into_cli(self, groups: Vec<String>) -> Result<Cli, String> {
		match self {
			Page::Help => Err(include_str!("../assets/help.txt").to_string()),
			Page::Status => Ok(Cli::Status { groups: None }),
			// use combobox for exclude and something groups
			Page::Add(exclude, force, adopt) => Ok(Cli::Add {
				groups,
				exclude: exclude.unwrap_or_default(),
				force,
				adopt,
			}),
			Page::Rm(exclude) => Ok(Cli::Rm {
				groups,
				exclude: exclude.unwrap_or_default(),
			}),
			Page::Set(exclude, force, adopt) => Ok(Cli::Set {
				groups,
				exclude: exclude.unwrap_or_default(),
				force,
				adopt,
			}),
			Page::Push(f) => Ok(Cli::Push {
				group: groups[0].clone(),
				files: f,
			}),
			Page::Pop => Ok(Cli::Pop { groups }),
			Page::Init => Ok(Cli::Init),
			Page::Hooks => Err("editer".into()),
		}
	}
}

impl Display for Page {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Page::Add(_, _, _) => write!(f, "Add"),
			Page::Help => write!(f, "Help"),
			Page::Init => write!(f, "Init"),
			Page::Pop => write!(f, "Pop"),
			Page::Push(_) => write!(f, "Push"),
			Page::Rm(_) => write!(f, "Rm"),
			Page::Set(_, _, _) => write!(f, "Set"),
			Page::Status => write!(f, "Status"),
			Page::Hooks => write!(f, "Hooks"),
		}
	}
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
	/// if the adopt flag is used on add and set
	#[serde(skip)]
	adopt: bool,
	/// timer between checking groups
	#[serde(skip)]
	check_count: u32,
	/// exclude
	exclude: Option<Vec<String>>,
	/// if the force flag is used on add and set
	#[serde(skip)]
	force: bool,
	/// Avalibule groups
	#[serde(skip)]
	found_groups: Option<Vec<String>>,
	/// The selected groups
	#[serde(skip)]
	groups: Option<Vec<String>>,
	label: String,
	#[serde(skip)]
	output: String,
	/// The page with command data incoded
	page: Page,
	/// The last opened hook script
	#[serde(skip)]
	code: String,
}
// todo add code block for hooks with the egui syntax_highlighting feature

const PANEL_FILL: Color32 = Color32::from_rgba_premultiplied(5, 18, 29, 247);
const BUTTON_SIZE: f32 = 7.0;
const ROUNDING: f32 = 5.5;
// colors
const INACTIVE: Color32 = Color32::from_rgb(62, 62, 110);
const HOVERED: Color32 = Color32::from_rgb(72, 72, 115);
const ACTIVE: Color32 = Color32::from_rgb(82, 82, 125);
const OPEN: Color32 = Color32::from_rgb(74, 74, 115);

fn visuals() -> egui::Visuals {
	let mut visuals = egui::Visuals::default();

	visuals.dark_mode = true;
	// Background
	visuals.window_stroke = egui::Stroke::NONE;
	visuals.extreme_bg_color = Color32::from_hex("#11304390").unwrap();
	visuals.code_bg_color = Color32::from_hex("#01243390").unwrap();
	visuals.faint_bg_color = Color32::from_hex("#01243360").unwrap();
	visuals.panel_fill = PANEL_FILL;
	visuals.override_text_color = Color32::from_hex("#14AAA9").ok();
	visuals.widgets.noninteractive.bg_stroke.color = Color32::from_hex("#113443").unwrap();
	// Widget size
	visuals.widgets.inactive.expansion = BUTTON_SIZE;
	visuals.widgets.hovered.expansion = BUTTON_SIZE;
	visuals.widgets.active.expansion = BUTTON_SIZE;
	visuals.widgets.open.expansion = BUTTON_SIZE;
	// Widget rounding
	visuals.widgets.inactive.rounding = ROUNDING.into();
	visuals.widgets.hovered.rounding = ROUNDING.into();
	visuals.widgets.active.rounding = ROUNDING.into();
	visuals.widgets.open.rounding = ROUNDING.into();
	// Widget color
	visuals.widgets.inactive.bg_fill = INACTIVE;
	visuals.widgets.hovered.bg_fill = HOVERED;
	visuals.widgets.active.bg_fill = ACTIVE;
	visuals.widgets.open.bg_fill = OPEN;

	visuals.widgets.inactive.weak_bg_fill = INACTIVE;
	visuals.widgets.hovered.weak_bg_fill = HOVERED;
	visuals.widgets.active.weak_bg_fill = ACTIVE;
	visuals.widgets.open.weak_bg_fill = OPEN;

	visuals
}

fn fonts() -> egui::FontDefinitions {
	let mut font = egui::FontDefinitions::default();
	font.font_data.insert(
		"FiraCode".to_string(),
		egui::FontData::from_static(include_bytes!("../assets/FiraCode-Regular.ttf")),
	);
	font.families.get_mut(&egui::FontFamily::Proportional).unwrap()
	.insert(0, "FiraCode".to_string());

	font
}

fn format_vec_str(vstr: &mut Vec<String>) -> String {
	let mut formated_str  = String::with_capacity(vstr.len() * 9);
	formated_str.push_str(&vstr.remove(0));
	for str in vstr {
		formated_str.push_str(", ");
		formated_str.push_str(str);
	}
	formated_str
}

impl TemplateApp {
	/// Called once before the first frame.
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		// This is also where we customize the look and feel of egui
		cc.egui_ctx.set_visuals(visuals());
		cc.egui_ctx.set_fonts(fonts());
		// ! look the thing for blur
		// cc.gl

		cc.egui_ctx.style_mut(|style| {
			for (text_style, font_id) in style.text_styles.iter_mut() {
				font_id.size = match text_style {
					egui::TextStyle::Small => 12.0,
					egui::TextStyle::Body => 14.0,
					egui::TextStyle::Monospace => 14.0,
					egui::TextStyle::Button => 15.0,
					egui::TextStyle::Heading => 23.0,
					egui::TextStyle::Name(_) => 16.0,
				}
			}
		});

		// Load previous app state (if any).
		// Note that you must enable the `persistence` feature for this to work.
		if let Some(storage) = cc.storage {
			return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
		}

		TemplateApp::default()
	}
}

impl Default for TemplateApp {
	fn default() -> Self {
		Self {
			force: false,
			adopt: false,
			check_count: 980,
			page: Page::default(),
			groups: None,
			exclude: None,
			found_groups: None,
			label: String::new(),
			output: String::new(),
			code: String::new(),
		}
	}
}

impl eframe::App for TemplateApp {
	/// make the window transparent
	fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
		[0.0, 0.0, 0.0, 0.0]
	}

	/// Called by the frame work to save state before shutdown.
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, eframe::APP_KEY, self);
	}

	/// Called each time the UI needs repainting, which may be many times per second.
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		// Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
		// For inspiration and more examples, go to https://emilk.github.io/egui
		let mut groups_handle: Option<JoinHandle<(Result<Vec<String>, ReturnCode>, String)>> = None;
		if self.check_count >= 1000 {
			groups_handle = Some(thread::spawn(|| {
				let mut output = "".to_string();
				(crate::groups::load_groups(&mut output), output)
			}));
			self.check_count = 0
		}
		self.check_count += 1;

		egui::CentralPanel::default().frame(
			egui::Frame::default().fill(PANEL_FILL)
			.inner_margin(egui::Margin::symmetric(12.0, 12.0)))
			.show(ctx, |ui| {
				// The central panel the region left after adding TopPanel's and SidePanel's
				ui.heading("Tuckr UI");

				ui.style_mut().spacing.item_spacing = egui::vec2(15.0, 15.0);
				ui.separator();

				ui.vertical_centered(|ui| {
					ui.label(&self.check_count.to_string());

					ui.horizontal(|ui| {
						egui::ComboBox::from_id_source(4)
							.selected_text(format!("{}", self.page))
							.show_ui(ui, |ui| {
								ui.selectable_value(
									&mut self.page,
									Page::Add(self.exclude.clone(), self.force, self.adopt),
									"Add",
								);
								ui.selectable_value(&mut self.page, Page::Help, "Help");
								ui.selectable_value(&mut self.page, Page::Init, "Init");
								ui.selectable_value(&mut self.page, Page::Pop, "Pop");
								ui.selectable_value(
									&mut self.page,
									Page::Push(self.groups.clone().unwrap_or(vec![self.label.clone()])),
									"Push",
								);
								ui.selectable_value(&mut self.page, Page::Rm(self.exclude.clone()), "Rm");
								ui.selectable_value(
									&mut self.page,
									Page::Set(self.exclude.clone(), self.force, self.adopt),
									"Set",
								);
								ui.selectable_value(&mut self.page, Page::Status, "Status");
								ui.selectable_value(&mut self.page, Page::Hooks, "Hooks")
							});

						ui.add_space(10.0);
						// todo use maiti selcte radio buttons
						// ui.label("Groups");
						ui.text_edit_singleline(&mut self.label);

						// icon for refresh button
						let refresh_icon = Image::new(include_image!("../assets/refresh.svg"));

						if ui.input_mut(|i| egui::InputState::consume_shortcut(i, &KeyboardShortcut { modifiers: Modifiers::COMMAND, logical_key: egui::Key::R }))
						|| ui.add(egui::Button::image(refresh_icon)).clicked() {
							self.check_count = 0;
							groups_handle = Some(thread::spawn(|| {
								let mut output = "".to_string();
								(crate::groups::load_groups(&mut output), output)
							}));
						}
					});

					// flags
					ui.horizontal(|ui| {
						ui.style_mut().spacing.item_spacing = egui::vec2(5.0, 5.0);

						ui.checkbox(&mut self.force, "");
						ui.label("Force");
						ui.add_space(10.0);
						ui.checkbox(&mut self.adopt, "");
						ui.label("Adopt");

						ui.style_mut().spacing.item_spacing = egui::vec2(15.0, 15.0);
					});

					// if the page is hooks list groups and hook files then open it in a editer
					if self.page == Page::Hooks {
						// todo file selector close on select
						// file_picker(self, ui);
						// todo load selected file in to editer
						code_editer(self, ui);
					}

					// groups
					ui.label(format_vec_str(&mut self.found_groups.clone().unwrap_or(vec!["".into()])));

					if ui.button("Exacute").clicked() {
						match self.page.clone()
						.into_cli(self.groups.clone().unwrap_or(vec![r"\*".into()])) {
							Ok(cli) => { self.output = run(cli).0 },
							Err(h) => { self.output = h; self.label = "select a group".into(); },
						};
					}

					ui.label(&self.output);
					// todo put in a popup then wight the ansouer to stdin
					// 	print!("Are you sure you want to override conflicts? (N/y) ");
					// } else if adopt {
					// 	print!("Are you sure you want to adopt conflicts? (N/y) ");
					// });
					// todo same for from stow
					// print!("Are you sure you want to convert your dotfiles to tuckr? (y/N)");
					// todo cmd_push
					// print!(
					// 	"{} already exists. Do you want to override it? (y/N) ",
					// 	target_file.to_str().unwrap()
					// );
					// todo pop_cmd
					// print!("\nDo you want to proceed? (y/N) ");
					ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
						egui::warn_if_debug_build(ui);
					});
				});
			});

		match groups_handle {
			Some(groups) => match groups.join() {
				Ok(g) => { self.output.push_str(&g.1); self.found_groups = g.0.ok() },
				Err(_) => (),
			},
			None => (),
		}
	}
}

fn code_editer(app: &mut TemplateApp, ui: &mut Ui) {
	let theme =
	egui_extras::syntax_highlighting::CodeTheme::from_style(ui.style()); //from_memory(ui.ctx());
	// ui.collapsing("Theme", |ui| {
	// 	ui.group(|ui| {
	// 		theme.ui(ui);
	// 		theme.clone().store_in_memory(ui.ctx());
	// 	});
	// });

	let mut layouter = |ui: &egui::Ui, code: &str, wrap_width: f32| {
		let mut layout_job = egui_extras::syntax_highlighting::highlight(
			ui.ctx(),
			&theme,
			&code,
			"bash",
		);
		layout_job.wrap.max_width = wrap_width;
		ui.fonts(|f| f.layout_job(layout_job))
	};

	egui::ScrollArea::vertical().show(ui, |ui| {
		ui.add(
			egui::TextEdit::multiline(&mut app.code)
				.font(egui::TextStyle::Monospace) // for cursor height
				.code_editor()
				.desired_rows(10)
				.lock_focus(true)
				.desired_width(f32::INFINITY)
				.layouter(&mut layouter),
		);
	});
}

	// opened_file: Option<PathBuf>,
	// open_file_dialog: Option<FileDialog>,

// fn file_picker(app: &mut TemplateApp, ui: &mut Ui) {
// 	if (ui.button("Open")).clicked() {
// 		// Show only files with the extension "txt".
// 		let filter = Box::new({
// 			let ext = Some(OsStr::new("txt"));
// 			move |path: &Path| -> bool { path.extension() == ext }
// 		});
// 		let mut dialog = FileDialog::open_file(app.opened_file.clone()).show_files_filter(filter);
// 		dialog.open();
// 		app.open_file_dialog = Some(dialog);
// 		}

// 		if let Some(dialog) = &mut app.open_file_dialog {
// 		if dialog.show(ctx).selected() {
// 			if let Some(file) = dialog.path() {
// 			app.opened_file = Some(file.to_path_buf());
// 			}
// 		}
// 		}
// }