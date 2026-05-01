import { invoke } from "@tauri-apps/api/core";
import type { RenderTask, RenderTaskForExport } from "./task-panel/task";
import type { Project } from "./project";

const render_task = (task: RenderTask) => {
  invoke("render_task", task);
};
export const render_many = (tasks: RenderTask[]) => {
  for (const task of tasks) {
    Command.render_task(task);
  }
};
const load_project = async () => {
  invoke("load_project");
};
const load_project_from_file = async (projectPath: String) => {
  invoke("load_project_from_file", { projectPath });
};
const choose_render_directory = async () => {
  invoke("choose_render_directory");
};
const create_project = async (project: Project, projectPath: String) => {
  invoke("create_project", { project, projectPath });
};
const pause_task = async (id: number) => {
  invoke("pause_task", { task: id });
};
const cancel_task = async (id: number) => {
  invoke("cancel_task", { task: id });
};
const resume_task = async (id: number) => {
  invoke("resume_task", { task: id });
};
const choose_scene_audio_file = async () => {
  invoke("choose_scene_audio_file");
};
const choose_scene_spatial_file = async () => {
  invoke("choose_scene_spatial_file");
};
const choose_scene_project_file = async () => {
  invoke("choose_scene_project_file");
};
const choose_queue_export = async () => {
  invoke("choose_queue_export");
};
const export_queue = async (queue: RenderTaskForExport[], path: String) => {
  invoke("export_queue", { queue, path });
};
const import_queue = async (path: String) => {
  invoke("import_queue", { path });
};
const choose_queue_import = async () => {
  invoke("choose_queue_import");
};
const set_render_directory = async (path: string) => {
  invoke("set_render_directory", { path });
};
const should_show_dev_warning = async (): Promise<boolean> => {
  return invoke("should_show_dev_warning");
};
const acknowledge_dev_warning = async () => {
  invoke("acknowledge_dev_warning");
};
const get_default_render_dir = async (projectPath: string): Promise<string> => {
  return invoke("get_default_render_dir", { projectPath });
};
export const Command = {
  pause_task,
  resume_task,
  cancel_task,
  create_project,
  choose_render_directory,
  load_project,
  load_project_from_file,
  render_task,
  choose_scene_audio_file,
  choose_scene_spatial_file,
  choose_scene_project_file,
  choose_queue_import,
  choose_queue_export,
  export_queue,
  import_queue,
  set_render_directory,
  should_show_dev_warning,
  acknowledge_dev_warning,
  get_default_render_dir,
};
