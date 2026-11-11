// https://docs.rs/eframe/latest/eframe/

use eframe::egui;
use chrono::{DateTime, Local, Duration, NaiveDate};

fn main() -> eframe::Result<()> 
{
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Clockin", native_options, Box::new(|cc| Ok(Box::new(Clockin::new(cc)))))
}

#[derive(Default)]
struct Clockin 
{
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
    elapsed: Duration,
    running: bool,
    sessions: Vec<Session>
}

struct Session
{
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
    duration: Duration,
    date: NaiveDate,
    produc_rate: u8 //u8 is so cool, its so memory efficient literally just stores values from 0-255 so peak
}

impl Clockin 
{
    fn new(_cc: &eframe::CreationContext<'_>) -> Self 
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
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) 
    {
        if self.running 
        {
            if let Some(start) = self.start_time
            {
                self.elapsed = Local::now() - start;
            }

            ui.ctx().request_repaint();
        }

        egui::CentralPanel::default().show(ui, |ui| 
        {
            let mil = self.elapsed.num_milliseconds();
            
            let hur = mil / 3_600_000;
            let min = (mil / 60_000) % 60;
            let sec = (mil / 1000) % 60;
            
            ui.label(format!("{:02}:{:02}:{:02}", hur, min, sec));

            ui.label(format!("Running: {}", self.running));

            let clock_in_out = if self.running
            {
                "Clockout"
            }
            else
            {
                "Clockin"
            };


            if ui.add_enabled(self.end_time.is_none(), egui::Button::new(clock_in_out)).clicked()
            {
                self.running = !self.running;

                if self.running
                {
                    self.start_time = Some(Local::now());
                    self.elapsed = Duration::zero();
                }
                else
                {
                    if let Some(start) = self.start_time
                    {
                        let end = Local::now();

                        self.elapsed = end - start;

                        if self.elapsed > Duration::seconds(10)
                        {
                            self.end_time = Some(end);
                        }
                        else
                        {
                            self.start_time = None;
                        }
                    }
                }
            }

            if let (Some(start), Some(end)) = (self.start_time, self.end_time)
            {
                ui.label("How productive were you? (1 = low, 5 = high)");

                for rating in 1..=5
                {
                    if ui.button(rating.to_string()).clicked()
                    {
                        let session = Session
                        {
                            start_time: start,
                            end_time: end,
                            duration: end - start,
                            date: start.date_naive(),
                            produc_rate: rating
                        };

                        self.sessions.push(session);

                        self.start_time = None;
                        self.end_time = None;
                    }
                }
            }


            // Testing
            ui.label("History");

            for session in &self.sessions
            {
                let total_sec = session.duration.num_seconds();

                let hur = total_sec / 3600;
                let min = (total_sec / 60) % 60;
                let sec = total_sec % 60;

                ui.label(format!("Duration: {:02}:{:02}:{:02}, Date: {}, Productivity: {}", hur, min, sec, session.date, session.produc_rate));
            }
        });
    }
}