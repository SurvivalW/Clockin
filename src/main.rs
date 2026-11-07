// https://docs.rs/eframe/latest/eframe/

use eframe::egui;
use chrono::{DateTime, Local, Duration};

fn main() 
{
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Clockin", native_options, Box::new(|cc| Ok(Box::new(Clockin::new(cc)))));
}

#[derive(Default)]
struct Clockin 
{
    start_time: Option<DateTime<Local>>,
    elapsed: Duration,
    running: bool
}

impl Clockin 
{
    fn new(cc: &eframe::CreationContext<'_>) -> Self 
    {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for Clockin 
{
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) 
    {
        if self.running 
        {
            if let Some(start) = self.start_time
            {
                self.elapsed = Local::now() - start;
            }
        }

        egui::CentralPanel::default().show(ui, |ui| 
        {
            let mil = self.elapsed.num_milliseconds();
            
            let hur = mil / 3_600_000;
            let min = (mil / 60_000) % 60;
            let sec = (mil / 1000) % 60;
            
            ui.label(format!("Running: {}", self.running));

            let clock_in_out = if self.running
            {
                "Clockout"
            }
            else
            {
                "Clockin"
            };


            if ui.button(clock_in_out).clicked()
            {
                self.running = !self.running;
            }
        });
    }
}