use std::thread;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::OutputStreamHandle;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

use egui::*;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
use eframe::{egui::{self, Ui, Resize}, epaint::tessellator::Path};


fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My SoundBoard Rust!", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

pub struct FileSys {
}

impl FileSys {
    fn create_folder(path: String) -> std::io::Result<()> {
        fs::create_dir(path)?;
        Ok(())
    }

    fn zip_folder(path: String) -> std::io::Result<()> {
        Ok(())
    }
    
 }

pub struct Player {
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    _sink: Sink,
}

impl Player {
    fn new() -> Player {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Player {
            _stream: stream,
            _stream_handle: stream_handle,
            _sink: sink
        }
    }


    fn play_a_sound(&mut self, path: String) {
        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();
        self._sink.append(source);
        self._sink.sleep_until_end();
    }

    fn stop_playing_sound(&mut self) {
        self._sink.stop();
    }
}

pub struct Sound {
    name: String,
    description: String,
    date: chrono::DateTime<Utc>,
}

impl Sound {
    fn print_info(&mut self) {
        println!("Name: {}", self.name);
        println!("Description {}", self.description);
        println!("Date {}", self.date.to_string());
    }
}

#[derive(Default)]
pub struct MyEguiApp {
    //_player: Player,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

    egui::CentralPanel::default().show(ctx, |ui| {     
        if ui.button("Play Sound").clicked() {
            thread::spawn(|| {
                let mut player = Player::new();
                player.play_a_sound(String::from("recordings/birds.mp3"));
            });
            }
        if ui.button("Stop Sound").clicked() {}
        
        });
        
    }
}
