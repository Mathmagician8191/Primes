//! A GUI to check the prime factors of a number
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use eframe::{egui, App};
use primes::{number_info, PrimeInput, ERROR};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Instant;

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

impl App for PrimeGUI {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Enter a number to factorise");
      ui.text_edit_singleline(&mut self.input);
      if self.ready && ui.button("Factorise").clicked() {
        let trimmed = self.input.trim();
        match trimmed.parse::<PrimeInput>() {
          Ok(i) => {
            self.message = Some("Calculating".to_owned());
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
            self.message = Some(ERROR.to_owned());
          }
        }
      }
      if let Some(ref receiver) = self.result {
        if let Ok(message) = receiver.try_recv() {
          self.message = Some(message);
          self.ready = true;
          self.result = None;
        }
      }
      if let Some(ref string) = self.message {
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
