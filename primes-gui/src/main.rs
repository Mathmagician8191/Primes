use eframe::egui;
use primes::number_info;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Instant;

type PrimeInput = u128;

struct PrimeGUI {
  input: String,
  ready: bool,
  message: Option<String>,
  result: Option<Receiver<String>>,
  instant: Instant,
  frames: u32,
  seconds: u64,
}

impl Default for PrimeGUI {
  fn default() -> Self {
    Self {
      input: String::new(),
      ready: true,
      message: None,
      result: None,
      instant: Instant::now(),
      frames: 0,
      seconds: 0,
    }
  }
}

impl eframe::App for PrimeGUI {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Enter a number to factorise");
      ui.text_edit_singleline(&mut self.input);
      if self.ready && ui.button("Factorise").clicked() {
        let trimmed = self.input.trim();
        match trimmed.parse::<PrimeInput>() {
          Ok(i) => {
            self.message = Some("Calculating".to_string());
            self.ready = false;
            let (tx, rx) = channel();
            let ctx = ctx.clone();
            thread::spawn(move || {
              tx.send(number_info(i)).expect("Sending calculation failed");
              ctx.request_repaint();
            });
            self.result = Some(rx);
          }
          Err(_) => {
            self.message = Some("You must enter a positive integer".to_string());
          }
        }
      }
      if let Some(receiver) = &self.result {
        if let Ok(message) = receiver.try_recv() {
          self.message = Some(message);
          self.ready = true;
          self.result = None;
        }
      }
      if let Some(string) = &self.message {
        ui.heading(string);
      }
    });
    if cfg!(debug_assertions) {
      self.frames += 1;
      let duration = self.instant.elapsed().as_secs();
      if duration - self.seconds > 0 {
        self.seconds = duration;
        println!("{} FPS", self.frames);
        self.frames = 0;
      }
    }
  }
}

fn main() {
  eframe::run_native(
    "Prime Factoriser",
    eframe::NativeOptions::default(),
    Box::new(|_cc| Box::<PrimeGUI>::default()),
  );
}
