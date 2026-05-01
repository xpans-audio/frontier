use std::{
    collections::BTreeMap,
    env,
    fs::{DirBuilder, File},
    io,
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        mpsc::{Sender, channel},
    },
    thread::{self},
};
mod config;
mod events;
mod open;
mod project;
mod render;
mod task;
use xpans_taskrenderer::{Progress as EncoderProgress, RenderTask};

use tauri::{
    Emitter, Manager, State,
    menu::{MenuBuilder, SubmenuBuilder},
};
use tauri_plugin_dialog::DialogExt;
use xpans_projectfile::Project;
use xpans_renderconfig::RenderConfig;
use xpans_taskrenderer::{AtomicStatus, Control, manage_control};
use xpans_xsr::SpatialSampleMap;

use crate::{
    config::{AppConfig, create_or_open_config, update_config},
    open::{
        VALID_PROJECT_EXTENSIONS, get_default_render_dir, handle_file_dragged_in,
        load_project_from_file, open_on_start,
    },
    project::{
        choose_scene_audio_file, choose_scene_project_file, choose_scene_spatial_file,
        create_project,
    },
    render::{Progress, cancel_task, manage_progress_channel, pause_task, resume_task},
    task::RenderTaskForExport,
};

struct AppStateStruct {
    pub app_config: AppConfig,
    pub loaded_project: Option<Project>,
    pub render_directory: Option<PathBuf>,
    pub spatial_scene: Option<Arc<SpatialSampleMap<usize, u16, f32>>>,
    pub control_senders: BTreeMap<u32, Sender<Control>>,
}
impl AppStateStruct {
    fn app_config(&self) -> &AppConfig {
        &self.app_config
    }

    fn set_app_config(&mut self, app_config: AppConfig) {
        self.app_config = app_config.clone();
        let _ = update_config(&app_config);
    }
}
impl Default for AppStateStruct {
    fn default() -> Self {
        Self {
            app_config: Default::default(),
            loaded_project: Default::default(),
            render_directory: Default::default(),
            spatial_scene: None,
            control_senders: BTreeMap::new(),
        }
    }
}

type AppState = Mutex<AppStateStruct>;

#[tauri::command]
fn should_show_dev_warning(state: State<'_, AppState>) -> bool {
    state.lock().unwrap().app_config.acknowledged_dev_warning == false
}

#[tauri::command]
fn acknowledge_dev_warning(state: State<'_, AppState>) {
    let mut lock = state.lock().unwrap();
    let mut config = lock.app_config().clone();
    config.acknowledged_dev_warning = true;
    lock.set_app_config(config);
}

#[tauri::command]
fn load_project(app: tauri::AppHandle) {
    app.dialog()
        .file()
        .set_title("Load Project")
        .add_filter("xpans Project", VALID_PROJECT_EXTENSIONS)
        .pick_file(move |path| {
            if let Some(path) = path {
                open::load_project_from_file_inner(&app, path.as_path().unwrap());
            }
        });
}

fn ensure_unique_path<P>(directory: P, name: &str, extension: &str) -> PathBuf
where
    P: AsRef<Path>,
{
    let directory = directory.as_ref();
    if directory.join(name).with_extension(extension).exists() {
        for i in 2..u32::MAX {
            let new = directory
                .join([name.to_owned(), " ".to_owned(), i.to_string()].concat())
                .with_extension(extension);
            if !new.exists() {
                return new;
            }
        }
    }
    directory.join(name).with_extension(extension)
}

fn ensure_directory_exists<P>(directory: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    if !directory.as_ref().exists() {
        DirBuilder::new().recursive(true).create(directory)?;
    }
    Ok(())
}

#[tauri::command]
fn set_render_directory(state: State<'_, AppState>, path: PathBuf) {
    state.lock().unwrap().render_directory = Some(path);
}

#[tauri::command]
fn choose_render_directory(app: tauri::AppHandle) {
    app.dialog()
        .file()
        .set_title("Set Render Directory")
        .pick_folder(move |path| {
            if let Some(path) = path {
                let path = path.into_path().unwrap();
                let state = app.state::<AppState>();
                state.lock().unwrap().render_directory = Some(path.clone());
                let _ = events::render_dir_chosen(&app, &path);
            }
        });
}

#[tauri::command]
fn choose_queue_export(app: tauri::AppHandle) {
    app.dialog()
        .file()
        .set_title("Save render queue to file:")
        .add_filter("Render task file", &["json"])
        .save_file(move |path| {
            if let Some(path) = path {
                let path = path.into_path().unwrap();
                let _ = events::queue_export_chosen(&app, &path);
            }
        });
}

#[tauri::command]
fn choose_queue_import(app: tauri::AppHandle) {
    app.dialog()
        .file()
        .set_title("Load render queue from file:")
        .add_filter("Render task file", &["json"])
        .pick_file(move |path| {
            if let Some(path) = path {
                let path = path.into_path().unwrap();
                let _ = events::queue_import_chosen(&app, &path);
            }
        });
}

#[tauri::command]
fn export_queue(queue: Vec<RenderTaskForExport>, path: PathBuf) {
    let queue: Vec<_> = queue.iter().map(|task| task.into_task()).collect();
    let file = File::create_new(path).unwrap();
    let _ = serde_json::to_writer_pretty(file, &queue);
}

#[tauri::command]
fn import_queue(app: tauri::AppHandle, path: PathBuf) {
    let file = File::open(path).unwrap();
    let queue: Vec<RenderTask> = serde_json::from_reader(file).unwrap();
    let queue: Vec<_> = queue
        .iter()
        .map(|task| RenderTaskForExport::from_task(task))
        .collect();
    let _ = events::queue_imported(&app, queue);
}

#[tauri::command]
async fn render_task(
    app: tauri::AppHandle,
    name: String,
    progress_channel: tauri::ipc::Channel<Progress>,
    render_config: RenderConfig,
) {
    let id = progress_channel.id();

    let state = app.state::<AppState>();
    let mut state_lock = state.lock().unwrap();
    let project = state_lock.loaded_project.clone().unwrap();
    let target_dir = state_lock.render_directory.clone().unwrap();

    let audio_in = project.audio;
    let spatial_in = project.spatial;
    ensure_directory_exists(&target_dir).unwrap();
    let audio_out = ensure_unique_path(&target_dir, &name, "wav");

    let scene = &mut state_lock.spatial_scene;
    if scene.is_none() {
        let _ = app.emit("loading_scene", ());
        let spatial_scene = render::load_scene(&spatial_in).unwrap();
        let spatial_scene = Arc::new(spatial_scene);
        *scene = Some(spatial_scene);
        let _ = app.emit("loaded_scene", ());
    }
    let spatial_scene = scene.as_ref().unwrap().clone();

    let (progress_sender, progress_receiver) = channel();

    tauri::async_runtime::spawn_blocking(|| {
        manage_progress_channel(progress_receiver, progress_channel, 240)
    });

    let (control_sender, control_receiver) = channel();
    state_lock.control_senders.insert(id, control_sender);
    let atomic_status = AtomicStatus::default();
    let status_to_manager = atomic_status.clone();
    tauri::async_runtime::spawn_blocking(|| manage_control(control_receiver, status_to_manager));

    let audio_in = audio_in.clone();
    let audio_out = audio_out.clone();
    start_render(
        render_config,
        atomic_status,
        progress_sender,
        spatial_scene,
        audio_in,
        audio_out,
    );
}

fn start_render(
    config: RenderConfig,
    atomic_status: AtomicStatus,
    progress_sender: Sender<EncoderProgress>,
    spatial_scene: Arc<SpatialSampleMap<usize, u16, f32>>,
    audio_in: PathBuf,
    audio_out: PathBuf,
) {
    thread::spawn(move || {
        xpans_taskrenderer::render_config(
            config,
            atomic_status,
            progress_sender,
            spatial_scene,
            &audio_in,
            &audio_out,
        )
    });
}

// const OPEN_PROJECT_SHORTCUT: Shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyO);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let config = create_or_open_config();
            let mut state = AppStateStruct::default();
            state.app_config = config;
            app.manage(Mutex::new(state));
            let args: Vec<String> = env::args().collect();
            open_on_start(app.handle(), &args);
            // let args: Vec<String> = env::args().collect();
            // println!("{args:?}");
            let project_menu = SubmenuBuilder::new(app, "Project")
                .text("new_project", "New")
                .text("open_project", "Open")
                .build()?;
            let task_menu = SubmenuBuilder::new(app, "Render")
                .text("load_batch_config", "Import task queue")
                .text("start_all", "Start all tasks")
                .build()?;
            let main_menu = SubmenuBuilder::new(app, "Application")
                .about(None)
                .hide()
                .quit()
                .build()?;
            let menu = MenuBuilder::new(app)
                .items(&[&main_menu, &project_menu, &task_menu])
                .build()?;

            app.set_menu(menu.clone())?;
            Ok(())
        })
        .on_window_event(handle_file_dragged_in)
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_project,
            pause_task,
            cancel_task,
            resume_task,
            load_project,
            load_project_from_file,
            choose_render_directory,
            render_task,
            choose_scene_audio_file,
            choose_scene_spatial_file,
            choose_scene_project_file,
            choose_render_directory,
            set_render_directory,
            export_queue,
            choose_queue_export,
            choose_queue_import,
            import_queue,
            should_show_dev_warning,
            acknowledge_dev_warning,
            get_default_render_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
