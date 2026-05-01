use std::{collections::BTreeMap, fs::File, path::PathBuf};

use pathdiff::diff_paths;
use tauri::Emitter;
use tauri_plugin_dialog::DialogExt;
use xpans_projectfile::{Project, ProjectFile};

use crate::events;

#[tauri::command]
pub fn create_project(
    app: tauri::AppHandle,
    project: Project,
    project_path: PathBuf,
) -> Result<(), String> {
    let project_path_parent = project_path.parent().unwrap();
    let audio_relative = diff_paths(&project.audio, &project_path_parent).unwrap();
    let spatial_relative = diff_paths(&project.spatial, &project_path_parent).unwrap();
    let project_file = ProjectFile {
        title: project.title,
        audio: audio_relative,
        spatial: spatial_relative,
        source_names: BTreeMap::new(),
    };
    let file = File::create_new(&project_path).map_err(|e| e.to_string())?;
    let _ = serde_json::to_writer_pretty(file, &project_file);
    let _ = app.emit("project_created", &project_path);
    Ok(())
}

#[tauri::command]
pub fn choose_scene_audio_file(app: tauri::AppHandle) {
    app.dialog()
        .file()
        .set_title("Set audio file for new scene:")
        .add_filter("Audio Files", &["wav", "aiff", "caf", "flac", "m4a"])
        .pick_file(move |path| {
            if let Some(path) = path {
                let path = path.into_path().unwrap();
                let _ = events::scene_audio_file_chosen(&app, &path);
            }
        });
}

#[tauri::command]
pub fn choose_scene_spatial_file(app: tauri::AppHandle) {
    app.dialog()
        .file()
        .set_title("Set spatial file for new scene:")
        .add_filter("xpans Spatial Record", &["xsr"])
        .pick_file(move |path| {
            if let Some(path) = path {
                let path = path.into_path().unwrap();
                let _ = events::scene_spatial_file_chosen(&app, &path);
            }
        });
}

#[tauri::command]
pub fn choose_scene_project_file(app: tauri::AppHandle) {
    app.dialog()
        .file()
        .set_title("Save project file:")
        .add_filter("xpans Scene File", &["json"])
        .save_file(move |path| {
            if let Some(path) = path {
                let path = path.into_path().unwrap();
                let _ = events::scene_project_file_chosen(&app, &path);
            }
        });
}
