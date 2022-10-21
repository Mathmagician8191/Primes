use eframe::egui;
use poll_promise::Promise;
use primes::number_info;
use std::time::Instant;

type PrimeInput = u128;

struct PrimeGUI {
  input: String,
  ready: bool,
  message: Option<String>,
  result: Option<Promise<String>>
}

impl Default for PrimeGUI {
  fn default() -> Self {
    Self {
      input : String::new(),
      ready: true,
      message : None,
      result : None
    }
  }
}

impl eframe::App for PrimeGUI {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    let time = Instant::now();
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Enter a number to factorise");
      ui.text_edit_singleline(&mut self.input);
      if self.ready && ui.button("Factorise").clicked() {
        let trimmed = self.input.trim();
        match trimmed.parse::<PrimeInput>() {
          Ok(i) => {
            self.message = Some("Calculating".to_string());
            self.ready = false;
            self.result = Some(Promise::spawn_thread("factorise", move || {
              number_info(i)
            }));
          }
          Err(_) => {
            self.message = Some("You must enter a positive integer".to_string());
          }
        }
      }
      if let Some(promise) = &self.result {
        if let Some(message) = promise.ready() {
          self.message = Some(message.to_string());
          self.ready = true;
          self.result = None;
        }
      }
      match &self.message {
        Some(string) => {
          ui.heading(string);
        }
        None => {}
      }
    });
    let micros = time.elapsed().as_micros();
    if micros > 50 {
      println!("Frame took {} μs", micros);
    }
  }
}

fn main() {
  let options = eframe::NativeOptions::default();
  eframe::run_native("Prime Factoriser", options, Box::new(|_cc| Box::new(PrimeGUI::default())))
}
