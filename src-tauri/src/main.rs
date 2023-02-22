#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::fs::Metadata;
use std::path::PathBuf;
use std::sync::mpsc::{channel};
use std::sync::{Arc, mpsc, Mutex};
use std::sync::atomic::{Ordering, AtomicBool};
use std::thread;
use jwalk::WalkDir;
use tauri::{Manager, Window, State};

mod files;


#[derive(Debug)]
pub struct AppState {
  directory: Arc<Mutex<String>>,
  running: Arc<Mutex<bool>>
}

impl AppState {
  fn new() -> Self {
    Self {
      directory: Arc::new(Mutex::new("".to_string())),
      running: Arc::new(Mutex::new(false)),
    }
  }
}

#[derive(Debug)]
pub enum Instruction {
  AddEntryToBaseFolder((Metadata, PathBuf)),
  IncrementFailedToRead
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

#[tauri::command]
fn set_directory(directory: &str, state: State<AppState>) {
  let mut dir = state.directory.lock().unwrap();
  *dir = String::from(directory);
}

#[tauri::command]
fn start_scan(state: State<AppState>) {
  let directory = state.directory.lock().unwrap();
  let mut running = state.running.lock().unwrap();

  if directory.is_empty() || *running {
    return;
  }

  *running = true;
  thread::spawn({
    let folder = PathBuf::from(directory.as_str());
    let mut running = state.running.clone();
    move || {
      for entry in WalkDir::new(&folder)
        .skip_hidden(false)
        .follow_links(false)
        .into_iter()
      {
        match entry {
          Ok(entry) => {
            match entry.metadata() {
              Ok(file_metadata) => {
                let entry_path = entry.path();
                // println!("fileName: {:?}, metadata: {:?}", entry_path, file_metadata);
              },
              Err(_) => panic!("Error in metadata"),
            }
          },
          Err(_) => panic!("Error in WalkDir"),
        }
      }
      *running.lock().unwrap() = false;
    }
  });
}

#[tauri::command]
fn init_process(window: Window, state: State<AppState>) {
  println!("状态：{:?}", state.running);
  std::thread::spawn(move || {
    window.emit("event-name", Payload { message: "Tauri is awesome!".into() }).unwrap();
  });
}

fn main() {
  tauri::Builder::default()
    .manage(AppState::new())
    .invoke_handler(tauri::generate_handler![set_directory, start_scan])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
