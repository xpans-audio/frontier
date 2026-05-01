use std::{
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
    str::FromStr,
    thread,
};

use tauri::{DragDropEvent, Manager, WindowEvent};
use xpans_projectfile::ProjectFile;

use crate::{AppState, events};

pub fn open_on_start(app: &tauri::AppHandle, args: &[String]) {
    for arg in args.iter() {
        let Ok(arg) = PathBuf::from_str(&arg);
        for extension in VALID_PROJECT_EXTENSIONS.iter() {
            if arg.ends_with(extension) {
                load_project_from_file_inner(&app, &arg);
            }
        }
    }
}

pub(crate) const VALID_PROJECT_EXTENSIONS: &[&str] = &["json", "toml"];

pub fn load_project_from_file_inner<P>(app: &tauri::AppHandle, path: P)
where
    P: AsRef<Path>,
{
    let file = File::open(&path).unwrap();
    let mut file = BufReader::new(file);
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    let project_file = read_projectfile(&string).unwrap();
    let project = project_file.into_project(path.as_ref().parent().unwrap());
    if let Some(project) = project {
        let state = app.state::<AppState>();
        state.lock().unwrap().loaded_project = Some(project.clone());
        let _ = events::project_loaded(app, project);
        let _ = events::project_loaded_from_path(app, path.as_ref().to_path_buf());
    }
}
#[tauri::command]
pub fn get_default_render_dir(project_path: PathBuf) -> PathBuf {
    project_path.parent().unwrap().join("Renders")
}

#[tauri::command]
pub async fn load_project_from_file(app: tauri::AppHandle, project_path: PathBuf) {
    load_project_from_file_inner(&app, project_path);
}

fn read_projectfile(str: &str) -> Option<ProjectFile> {
    let project_file = toml::from_str::<ProjectFile>(&str);
    if let Ok(project_file) = project_file {
        return Some(project_file);
    }
    let project_file = serde_json::from_str::<ProjectFile>(&str);
    if let Ok(project_file) = project_file {
        return Some(project_file);
    }
    return None;
}

pub fn handle_file_dragged_in(window: &tauri::Window, window_event: &tauri::WindowEvent) {
    let window = window.clone();
    let window_event = window_event.clone();
    thread::spawn(move || {
        let app = window.app_handle();
        if let WindowEvent::DragDrop(DragDropEvent::Drop {
            paths,
            position: _position,
        }) = window_event
        {
            load_project_from_file_inner(app, paths.first().unwrap());
        }
    });
}
