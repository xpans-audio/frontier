const SCENE_READ_BUFFER_SIZE: usize = 16384 * 2;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, BufReader},
    path::Path,
    sync::mpsc::Receiver,
    time::{Duration, Instant},
};
use tauri::Manager;
use xpans_taskrenderer::Control;
use xpans_taskrenderer::Progress as EncoderProgress;

use xpans_xsr::{Record, SpatialSampleMap, record_into_map};

use crate::AppState;

pub fn load_scene(path: &Path) -> io::Result<SpatialSampleMap<usize, u16, f32>> {
    let spatial_file = File::open(path)?;
    let spatial_file = BufReader::with_capacity(SCENE_READ_BUFFER_SIZE, spatial_file);
    let xsr: Record<usize, u16, f32> = rmp_serde::from_read(spatial_file).unwrap();
    // let starting_sample = xsr.starting_sample();
    let sample_map = record_into_map(xsr.samples);
    Ok(sample_map)
}
#[tauri::command]
pub fn pause_task(app: tauri::AppHandle, task: u32) {
    let state = app.state::<AppState>();
    let mut state_lock = state.lock().unwrap();
    let control_senders = &mut state_lock.control_senders;
    let entry = control_senders.get_mut(&task);
    if let Some(entry) = entry {
        if entry.send(Control::Pause).is_err() {
            control_senders.remove(&task);
        }
    }
}

#[tauri::command]
pub fn resume_task(app: tauri::AppHandle, task: u32) {
    let state = app.state::<AppState>();
    let mut state_lock = state.lock().unwrap();
    let control_senders = &mut state_lock.control_senders;
    let entry = control_senders.get_mut(&task);
    if let Some(entry) = entry {
        if entry.send(Control::Resume).is_err() {
            control_senders.remove(&task);
        }
    }
}

#[tauri::command]
pub fn cancel_task(app: tauri::AppHandle, task: u32) {
    let state = app.state::<AppState>();
    let mut state_lock = state.lock().unwrap();
    let control_senders = &mut state_lock.control_senders;
    let entry = control_senders.get_mut(&task);
    if let Some(entry) = entry {
        if entry.send(Control::Cancel).is_err() {
            control_senders.remove(&task);
        }
    }
}

pub fn manage_progress_channel(
    progress_receiver: Receiver<EncoderProgress>,
    channel: tauri::ipc::Channel<Progress>,
    update_rate_hz: u64,
) {
    let nanoseconds = 1_000_000_000 / update_rate_hz;
    let update_cycle = Duration::from_nanos(nanoseconds);
    let mut last_progress_instant = Instant::now();

    while let Ok(msg) = progress_receiver.recv() {
        if let EncoderProgress::Sample(_, _) = msg {
            let elapsed = last_progress_instant.elapsed();
            if elapsed < update_cycle {
                continue;
            }
            last_progress_instant = Instant::now();
        }
        let progress = match msg {
            EncoderProgress::Sample(current, target) => Progress::Rendering {
                progress: current as f32 / target as f32,
            },
            EncoderProgress::Failed => Progress::Failed,
            EncoderProgress::Finished => Progress::Finished,
        };
        let _ = channel.send(progress);
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "status")]
pub enum Progress {
    LoadingScene,
    Rendering { progress: f32 },
    Failed,
    Finished,
}
