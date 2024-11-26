use std::{
    thread, 
    time::Duration, 
    sync::{Arc, Mutex},
    path::PathBuf,
};
use rfd::{FileDialog, MessageDialog};
use capture::{check_paths, sstest};


#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    interval: f32,
    save_path: Arc<Mutex<PathBuf>>,

    #[serde(skip)]
    dark_mode: bool,
    recording: Arc<Mutex<bool>>,
}


impl Default for TemplateApp {
    fn default() -> Self {
        let mut default_path: PathBuf = std::env::current_dir().unwrap();
        default_path.push("capture");

        Self {
            interval: 5.0,
            save_path: Arc::new(Mutex::new(default_path)),
            dark_mode: false,
            recording: Arc::new(Mutex::new(false)),
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        
        Default::default()
    }
}



impl eframe::App for TemplateApp {
    
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default()
            .frame(new_frame(self.dark_mode))
            .show(ctx, |ui| {

            self.dark_mode = ui.style_mut().visuals.dark_mode;

            egui::Grid::new("uis")
                //.num_columns(5)
                //.max_col_width(80.0)
                .show(ui, |ui| {

                    // select folder
                    let ib = ui.add_sized([24.0, 24.0],
                        egui::ImageButton::new(egui::Image::new(egui::include_image!("img/folder_24.png")))
                    );
                    if ib.clicked() {
                        let res = FileDialog::new()
                            .set_directory(&*self.save_path.lock().unwrap())
                            .set_title("Select a directry for captures")
                            .pick_folder();

                        if res != None {
                            *self.save_path.lock().unwrap() = res.unwrap();
                        }
                    }

                    // recording 
                    if *self.recording.lock().unwrap() == false {
                        let ib = ui.add_sized([24.0, 24.0],
                            egui::ImageButton::new(egui::Image::new(egui::include_image!("img/rec_24.png")))
                        );
                        if ib.clicked() {
                            if *self.recording.lock().unwrap() == false {
                                *self.recording.lock().unwrap() = true;
                                let rec = Arc::clone(&self.recording);
                                let inter = (self.interval * 1000.0) as u64;


                                let save_path = self.save_path.lock().unwrap();
                                let mut save_path = save_path.clone();
                                check_paths(&mut save_path).unwrap();


                                let save_path = Arc::clone(&self.save_path);

                                thread::spawn(move || {

                                    // recording loop
                                    while *rec.lock().unwrap() {
                                        thread::sleep(Duration::from_millis(inter));    
                                        sstest(&save_path.lock().unwrap());
                                    }

                                });
                            }
                        }
                    } else {
                        let ib = ui.add_sized([24.0, 24.0],
                            egui::ImageButton::new(egui::Image::new(egui::include_image!("img/stop_24.png")))
                        );
                        if ib.clicked() {
                            *self.recording.lock().unwrap() = false;
                        }
                    }


                    //ui.add_sized([200.0, 24.0],
                    ui.add( egui::Slider::new(&mut self.interval, 1.0..=60.0).text("Interval") );

                });
        });
    }
}

fn new_frame(dark_mode: bool) -> egui::containers::Frame {
    let style = match dark_mode {
        true => egui::style::Visuals::dark(),
        false => egui::style::Visuals::light(),
    };

    let bg: egui::Color32 = style.widgets.noninteractive.bg_fill;
    let fg: egui::Color32 = style.widgets.noninteractive.fg_stroke.color;
    egui::containers::Frame {
        inner_margin: egui::Margin { left: 16.0, right: 16.0, top: 16.0, bottom: 16.0 },
        outer_margin: egui::Margin { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 },
        rounding: egui::Rounding { nw: 0.0, ne: 0.0, sw: 0.0, se: 0.0 },
        shadow: eframe::epaint::Shadow { color: egui::Color32::DARK_GRAY, offset: egui::vec2(0.0, 0.0), spread: 0.0, blur: 0.0 },
        fill: bg,
        stroke: egui::Stroke::new(0.0, fg),
    }
}

