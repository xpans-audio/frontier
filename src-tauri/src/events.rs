use std::path::{Path, PathBuf};

use tauri::{AppHandle, Emitter, Error};
use xpans_projectfile::Project;

use crate::task::RenderTaskForExport;

pub fn project_loaded(app: &AppHandle, project: Project) -> Result<(), Error> {
    app.emit("project_loaded", project)
}
pub fn project_loaded_from_path(app: &AppHandle, path: PathBuf) -> Result<(), Error> {
    app.emit("project_loaded_from_path", path)
}
#[allow(dead_code)]
pub fn project_unloaded(app: &AppHandle, project: Project) -> Result<(), Error> {
    app.emit("project_unloaded", project)
}
pub fn render_dir_chosen(app: &AppHandle, path: &Path) -> Result<(), Error> {
    app.emit("render_dir_chosen", path)
}
pub fn queue_export_chosen(app: &AppHandle, path: &Path) -> Result<(), Error> {
    app.emit("queue_export_chosen", path)
}
pub fn queue_import_chosen(app: &AppHandle, path: &Path) -> Result<(), Error> {
    app.emit("queue_import_chosen", path)
}
pub fn queue_imported(app: &AppHandle, queue: Vec<RenderTaskForExport>) -> Result<(), Error> {
    app.emit("queue_imported", queue)
}
pub fn scene_spatial_file_chosen(app: &AppHandle, path: &Path) -> Result<(), Error> {
    app.emit("scene_spatial_file_chosen", path)
}
pub fn scene_audio_file_chosen(app: &AppHandle, path: &Path) -> Result<(), Error> {
    app.emit("scene_audio_file_chosen", path)
}
pub fn scene_project_file_chosen(app: &AppHandle, path: &Path) -> Result<(), Error> {
    app.emit("scene_project_file_chosen", path)
}
#[allow(dead_code)]
pub fn show_dev_warning(app: &AppHandle) -> Result<(), Error> {
    app.emit("show_dev_warning", "")
}
