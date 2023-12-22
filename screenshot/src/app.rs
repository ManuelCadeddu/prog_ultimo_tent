use serde::{Serialize, Deserialize};
use eframe::egui;
use egui::*;
use screenshots::Screen;
use std::time::Instant;

use winit::{
    event::{Event, WindowEvent, MouseButton, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

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

#[derive(Debug, Clone, Copy)]
struct MousePosition {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Copy)]
struct SelectionArea {
    start: MousePosition,
    end: MousePosition,
}

impl Default for SelectionArea {
    fn default() -> Self {
        Self {
            start: MousePosition { x: 0.0, y: 0.0 },
            end: MousePosition { x: 0.0, y: 0.0 },
        }
    }
}

struct AppState {
    selection_area: Option<SelectionArea>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            selection_area: None,
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
                        //screenshot_full();
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot);
                    };

                    ui.add_space(20.0);

                    if ui.button("AREA").clicked() {
                        screenshot_area();
                        
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
/*
        egui::CentralPanel::default().show(ctx, |ui| {
        
            ui.horizontal(|ui| {

                ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                    
                    if ui.button("take screenshot!").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot);
                    }
                });
            });
        });

        if let Some(texture) = self.texture.as_ref() {
            ui.image((texture.id(), ui.available_size()));
        } else {
            ui.spinner();
        }

        // Check for returned screenshot:
        ui.input(|i| {
            for event in &i.raw.events {
                if let egui::Event::Screenshot { image, .. } = event {
                    if self.save_to_file {
                        let pixels_per_point = i.pixels_per_point();
                        let region = egui::Rect::from_two_pos(
                            egui::Pos2::ZERO,
                            egui::Pos2 { x: 100., y: 100. },
                        );
                        let top_left_corner = image.region(&region, Some(pixels_per_point));
                        image::save_buffer(
                            "top_left.png",
                            top_left_corner.as_raw(),
                            top_left_corner.width() as u32,
                            top_left_corner.height() as u32,
                            image::ColorType::Rgba8,
                        )
                        .unwrap();
                        self.save_to_file = false;
                    }
                    self.screenshot = Some(image.clone());
                }
            }
        });


        ctx.request_repaint();*/
    }
}

fn screenshot_full() {

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

fn screenshot_area() {

    //let screens = Screen::all().unwrap();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

   // let capturer = screenshots::Capturer::new(screenshots::Screen::Main).unwrap();
    let mut app_state = AppState::default();

    event_loop.run(move |event, _, control_flow| {

        *control_flow = ControlFlow::Wait;

        match event {

            Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, .. } => {
                
                // `position` contiene la posizione del cursore
                MousePosition.x = position.x;
                MousePosition.y = position.y;
            }

            Event::WindowEvent { event, .. } => match event {

                // input con trascinamento del mouse
                WindowEvent::MouseInput { state, button, .. } => {

                    if state == ElementState::Pressed && button == MouseButton::Left {

                        app_state.selection_area = Some(SelectionArea {

                            start: MousePosition {
                                x: window.cursor_position().unwrap().x,
                                y: window.cursor_position().unwrap().y,
                            },
                            end: MousePosition { x: 0.0, y: 0.0 },
                        });

                    } else if state == ElementState::Released && button == MouseButton::Left {

                        app_state.selection_area.as_mut().map(|area| {

                            area.end = MousePosition {
                                x: window.cursor_position().unwrap().x,
                                y: window.cursor_position().unwrap().y,
                            };
                        });

                        // Cattura lo screenshot dell'area selezionata
                        if let Some(area) = app_state.selection_area {

                            let screens = Screen::all().unwrap();

                            for screen in screens {

                                let mut image = screen.capture_area(300, 300, 300, 300).unwrap();
                        
                                image
                                    .save(format!("target/area_{}.png", screen.display_info.id))
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
                        }

                        // Resetta l'area di selezione
                        app_state.selection_area = None;
                    }
                }
                _ => (),
            },
            _ => (),
        }
    });
}


