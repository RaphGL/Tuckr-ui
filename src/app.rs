use std::fmt::Display;
use tuckr::dotfiles::Dotfile;
/// the tuckr state 
use tuckr::Cli;
use egui::Color32;
use crate::cmd::run;

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
#[derive(Default, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
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
    /// `\*` is all groups, None if page is help
    pub fn try_into_cli(self, groups: Vec<String>) -> Option<Cli> {
        match self {
            Page::Help => None,
            Page::Status => Some(Cli::Status { groups: None }),
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
    /// The selected groups
    #[serde(skip)]
    groups: Option<Vec<String>>,
    /// exclude
    exclude: Option<Vec<String>>,
    label: String,
    value: f32,
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

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_visuals(visuals());

        cc.egui_ctx.style_mut(|style|{
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

        Default::default()
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

        egui::CentralPanel::default().frame(egui::Frame::default().fill(PANEL_FILL).inner_margin(egui::Margin::symmetric(12.0, 12.0))).show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Tuckr UI");

            ui.style_mut().spacing.item_spacing = egui::vec2(15.0, 15.0);
            ui.separator();

            ui.vertical_centered(|ui| {

                ui.horizontal(|ui| {
                    egui::ComboBox::from_id_source(4)
                        .selected_text(format!("{}", self.page))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.page, Page::Add(self.exclude.clone(), self.force, self.adopt), "Add");
                            ui.selectable_value(&mut self.page, Page::Help, "Help");
                            ui.selectable_value(&mut self.page, Page::Init, "Init");
                            ui.selectable_value(&mut self.page, Page::Pop, "Pop");
                            ui.selectable_value(&mut self.page, Page::Push(self.groups.clone().unwrap_or(vec![self.label.clone()])), "Push");
                            ui.selectable_value(&mut self.page, Page::Rm(self.exclude.clone()), "Rm");
                            ui.selectable_value(&mut self.page, Page::Set(self.exclude.clone(), self.force, self.adopt), "Set");
                            ui.selectable_value(&mut self.page, Page::Status, "Status");
                        });

                    ui.add_space(10.0);
                    // todo use maiti selcte radio buttons
                    // ui.label("Groups");
                    ui.text_edit_singleline(&mut self.label);
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

                if ui.button("Exacute").clicked() {
                    let _ = match self.page.clone().try_into_cli(self.groups.clone().unwrap_or(vec![r"\*".into()])) {
                        Some(cli) => run(cli),
                        None => {Ok(self.label = "select a group".into())},
                    };
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    egui::warn_if_debug_build(ui);
                });
            });
        });
    }
}
