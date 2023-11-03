use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};

use model::models::{Project, Task, PatchProject};
use crate::api::action::Action;
use crate::api::Update;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    edit_project_name: String,
    #[serde(skip)]
    task_text: String,
    #[serde(skip)]
    edit_project: Option<Rc<RefCell<Project>>>,
    #[serde(skip)]
    project_name: String,
    #[serde(skip)]
    promise: Option<poll_promise::Promise<String>>,
    // Receiver of messages from network
    #[serde(skip)]
    receiver: Option<Receiver<Update>>,
    #[serde(skip)]
    action: Option<Action>,
    #[serde(skip)] 
    tasks: Vec<Task>,
    #[serde(skip)] 
    selected_project: Option<Rc<RefCell<Project>>>,
    #[serde(skip)] 
    projects: Vec<Rc<RefCell<Project>>>,
}

async fn fetch(url: &str) -> String {
    let client = reqwest::Client::new();
    let resp = client.get(url).send().await.unwrap();
    resp.text().await.unwrap()
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>, server_url: String) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }
        let (sender, receiver) = std::sync::mpsc::channel::<Update>();
        let action = Action { sender: Some(sender), server_url: server_url.into() };
        action.get_projects();
        TemplateApp { 
            action: Some(action),
            promise: None,
            task_text: String::new(),
            selected_project: None,
            tasks: Vec::new(),
            edit_project: None,
            edit_project_name: String::new(),
            receiver: Some(receiver),
            projects: Vec::new(),
            project_name: String::new()
        }
    }
    fn action(&self) -> &Action {
        self.action.as_ref().unwrap()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(ref mut r) = self.receiver {
            if let Ok(m) = r.try_recv() {
                match m {
                    Update::ProjectList(project_list) => {
                        self.projects = project_list.into_iter().map(|p| Rc::new(RefCell::new(p))).collect();
                    }
                    Update::TaskList(tasks) => {
                        self.tasks = tasks;
                    }
                    Update::ProjectCreated(project) => {
                        self.projects.push(Rc::new(RefCell::new(project)));
                    }
                    Update::TaskDeleted(task) => {
                        let pos = self.tasks.iter().position(|t| t.id == task.id);
                        if let Some(pos) = pos {
                            self.tasks.remove(pos);
                        }
                    }
                    Update::ProjectChanged(project) => {
                        if let Some(changed_project) = self.projects.iter().find(|p| p.borrow().id == project.id) {
                            project.patch(&mut changed_project.borrow_mut());
                        }
                    }
                    Update::TaskCreated(task) => {
                        self.tasks.push(task);
                    }
                    Update::ProjectDeleted(project) => {
                        if let Some(ref selected) = self.selected_project {
                            if selected.borrow().id == project.id {
                                self.selected_project = None;
                            }
                        }
                        let pos = self.projects.iter().position(|p| p.borrow().id == project.id);
                        if let Some(pos) = pos {
                            self.projects.remove(pos);
                        }
                    }
                }
            }
        }
        
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }
            });
        });

        egui::SidePanel::new(egui::panel::Side::Left, "left").show(ctx, |ui| {
            ui.label("Enter new project name:");
            ui.text_edit_singleline(&mut self.project_name);
            let create_project_button = egui::Button::new("Create New Project")
                .fill(egui::Color32::from_rgb(50, 100, 150));
            if ui.add(create_project_button).clicked() {
                let name = self.project_name.trim();
                if !name.is_empty() {
                    self.action().create_project(name);
                    self.project_name.clear();
                }
            }
            ui.separator();
            ui.label("Project list:");
            let mut id = None;
            if let Some(ref edit_project) = self.edit_project {
                id = Some(edit_project.borrow().id);
            }
            let action: Action = self.action().clone();
            for p in &self.projects {
                ui.horizontal(|ui| {
                    let x_button = egui::Button::new("X")
                        .fill(egui::Color32::from_rgb(90, 20, 20));
                    if ui.add(x_button).clicked() {
                        action.delete_project(p.borrow().id);
                    }
                    let edit_button = egui::Button::new("Edit")
                        .fill(egui::Color32::from_rgb(90, 90, 20));
                    if ui.add(edit_button).clicked() {
                        {
                            self.edit_project_name = p.borrow().title.clone();
                            self.edit_project = Some(p.clone());
                        }
                    }
                    
                    
                    if id == Some(p.borrow().id) {
                        let response = ui.text_edit_singleline(&mut self.edit_project_name);
                        if ui.button("cancel").clicked() {
                            self.edit_project = None;
                        }
                        if ui.button("ok").clicked() || response.lost_focus() && response.ctx.input(|r|{r.key_pressed(egui::Key::Enter)}) {
                            self.edit_project = None;
                            if !p.borrow().title.eq(&self.edit_project_name) {
                                let patched = PatchProject {
                                    id: p.borrow().id,
                                    title: self.edit_project_name.clone()
                                };
                                action.edit_project(patched);
                            }
                        };
                    } else {
                        let mut project_button = egui::Button::new(&p.borrow().title);
                        if let Some(ref selected_project) = self.selected_project {
                            if selected_project.borrow().id == p.borrow().id {
                                project_button = project_button.fill(egui::Color32::from_rgb(20, 79, 20));
                            }
                        }
                            
                        if ui.add(project_button).clicked() {
                            self.selected_project = Some(p.clone());
                            action.get_tasks(p.borrow().id);
                        };
                    }
                });
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(ref selected_project) = self.selected_project {
                ui.heading(&format!("Project:    '{}'", selected_project.borrow().title));
                ui.separator();
                ui.label("Enter task description:");
                ui.text_edit_multiline(&mut self.task_text);
                let mut create_task_button = egui::Button::new(
                    egui::RichText::new("Create New Task")
                        .color(egui::Color32::from_rgb(255, 255, 255))
                    ).fill(egui::Color32::from_rgb(20, 150, 20));
                if ui.add(create_task_button).clicked() && !self.task_text.trim().is_empty() {
                    self.action().create_task(&self.task_text, selected_project.borrow().id);
                    self.task_text.clear();
                }
            }
            
            ui.separator();
            ui.label("Tasks:");
            ui.separator();
            // The central panel the region left after adding TopPanel's and SidePanel's
            for (i, t) in self.tasks.iter().enumerate() {
                ui.horizontal(|ui| {
                    let delete_task_button = egui::Button::new("X")
                        .fill(egui::Color32::from_rgb(90, 20, 20));
                    if ui.add(delete_task_button).clicked() {
                        self.action().delete_task(t.id);
                    }
                    ui.label(&format!("{:<2}. ", i + 1));
                    ui.label(&t.title);
                });
            }

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
