use screenshot::TemplateApp;
use eframe::egui;

fn main() -> eframe::Result<()> {

    let native_options = eframe::NativeOptions {

        viewport: egui::ViewportBuilder::default()
            .with_title("Screenshot")
            //.with_decorations(false)
            .with_maximized(true)
            .with_transparent(true)
            .with_always_on_top()
            .with_resizable(false)
            //.with_title_shown(false)
            .with_titlebar_buttons_shown(false),
            //.with_titlebar_shown(false)
            //.with_active(true)
            //.with_visible(true)
            //.with_always_on_top(),
        ..Default::default()  // il resto della struttura viene riempito con i valori di default
    };

    eframe::run_native(
        "screenshot",
        native_options,
        Box::new(|cc| {

            Box::new(TemplateApp::new(cc))

        }),    // applicazione che verrà eseguita all'interno di eframe
    )
}