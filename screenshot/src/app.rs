use serde::{Serialize, Deserialize};
use eframe::egui;
use egui::*;
use screenshots::Screen;
use std::time::Instant;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Screenshot!".to_owned(),
            value: 1.0,
        }
    }
}


impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        Default::default()
    }
}


impl eframe::App for TemplateApp {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        /*
       let my_frame = egui::containers::Frame {

           fill: egui::Color32::from_rgba_premultiplied(116, 109, 113, 200),
           ..Default::default()
       };
       egui::CentralPanel::default().me(my_frame).show(ctx, |ui| {});
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
       });*/


        egui::TopBottomPanel::top("My TopPanel")
            .frame(Frame::none().fill(Color32::TRANSPARENT))
            .show_separator_line(false)
            .show(&ctx, |ui| {

                ui.horizontal(|ui| {

                    ui.spacing_mut().item_spacing.x = 0.0; // Riduci lo spazio tra i widget

                    if ui.button("FULL").clicked() {
                        screenshot();
                    };

                    ui.add_space(20.0);

                    if ui.button("AREA").clicked() {
                        screenshot();
                    };

                    ui.add_space(20.0);

                    ui.image(egui::include_image!("icon_screen.png"));
                });

        /*
                    ScrollArea::vertical()
                        .auto_shrink(false)
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                            ui.bel(&self.label);
                        });

                    if ctx.input(|i| i.key_pressed(Key::A)) {
                        self.label.push_str("\nPressed");
                    }
                    if ctx.input(|i| i.key_down(Key::A)) {
                        self.label.push_str("\nHeld");
                        ui.ctx().request_repaint(); // make sure we note the holding.
                    }
                    if ctx.input(|i| i.key_released(Key::A)) {
                        self.label.push_str("\nReleased");
                    }*/
                });
    }
}

fn screenshot() {

    let start = Instant::now();
    let screens = Screen::all().unwrap();

    for screen in screens {

        let mut image = screen.capture().unwrap();
        image
            .save(format!("target/{}.png", screen.display_info.id))
            .unwrap();
        /*
                image
                    .save(format!("target/{}.jpeg", screen.display_info.id))
                    .unwrap();

                image
                    .save(format!("target/{}.gif", screen.display_info.id))
                    .unwrap();
         */
    }
    /*
        image = screen.capture_area(300, 300, 300, 300).unwrap();
        image
            .save(format!("target/{}-2.png", screen.display_info.id))
            .unwrap();

        let screen = Screen::from_point(100, 100).unwrap();
        println!("capturer {screen:?}");

        let image = screen.capture_area(300, 300, 300, 300).unwrap();
        image.save("target/capture_display_with_point.png").unwrap();
        println!("运行耗时: {:?}", start.elapsed());*/
}

