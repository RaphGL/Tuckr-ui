use std::fmt::Display;
use tuckr::{dotfiles::Dotfile};
/// the tuckr state 
use tuckr::Cli;
use egui::Color32;

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

/// enum for each page/command
#[derive(Default, serde::Deserialize, serde::Serialize)]
pub enum Page {
    #[default]
    Help,
    Status,
    // (exclude, force, adopt)
    Add(Option<Vec<String>>, bool, bool),
    // exclude
    Rm(Option<Vec<String>>),
    // (exclude, force, adopt)
    Set(Option<Vec<String>>, bool, bool),
    // files
    Push(Vec<String>),
    Pop,
    Init,
}

impl Page {
/// Prepare for runing a command, push will only use the first group\
/// `\*` is all groups
pub fn try_into_cli(self, groups: Vec<String>) -> Option<Cli> {
    match self {
        Page::Help => None,
        Page::Status => Some(Cli::Status { groups: Some(groups) }),
        // use combobox for exclude and something groups
        Page::Add(exclude, force, adopt) => Some(Cli::Add { groups, exclude: exclude.unwrap_or_default(), force, adopt }),
        Page::Rm(exclude) => Some(Cli::Rm { groups, exclude: exclude.unwrap_or_default() }),
        Page::Set(exclude, force, adopt) => Some(Cli::Set { groups, exclude: exclude.unwrap_or_default(), force, adopt }),
        Page::Push(f) => Some(Cli::Push { group: groups[0].clone(), files: f }),
        Page::Pop => Some(Cli::Pop { groups }),
        // on macos create $HOME/.dotfiles insted of using init
        Page::Init => Some(Cli::Init)
    }
}
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    /// The page with command data incoded
    page: Page,
    /// if the force flag is used on add and set
    #[serde(skip)]
    force: bool,
    /// if the adopt flag is used on add and set
    #[serde(skip)]
    adopt: bool,
    label: String,
    value: f32,
}

const PANEL_FILL: Color32 = Color32::from_rgba_premultiplied(5, 18, 29, 245);

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        let mut visuals = egui::Visuals::default();

        visuals.window_stroke = egui::Stroke::NONE;
        visuals.extreme_bg_color = Color32::from_hex("#CBE0F0FA").unwrap();
        visuals.code_bg_color = Color32::from_hex("#CBE0F0").unwrap();
        visuals.faint_bg_color = Color32::from_hex("#01243360").unwrap();
        visuals.panel_fill = PANEL_FILL;
        visuals.override_text_color = Color32::from_hex("#14AAA9").ok();
        cc.egui_ctx.set_visuals(visuals);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
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

        egui::TopBottomPanel::top("top_panel").frame(egui::Frame::default().fill(PANEL_FILL).inner_margin(3.0)).show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().frame(egui::Frame::default().fill(PANEL_FILL).inner_margin(3.0)).show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
