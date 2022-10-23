use eframe::egui;
use poll_promise::Promise;
use primes::number_info;
use std::time::Duration;
use std::time::Instant;
type PrimeInput = u128;

struct PrimeGUI {
  input: String,
  ready: bool,
  message: Option<String>,
  result: Option<Promise<String>>,
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
            self.result = Some(Promise::spawn_thread("factorise", move || number_info(i)));
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
    ctx.request_repaint_after(Duration::from_millis(200));
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
    Box::new(|_cc| Box::new(PrimeGUI::default())),
  )
}
