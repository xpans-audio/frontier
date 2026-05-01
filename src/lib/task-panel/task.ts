import { Channel } from "@tauri-apps/api/core";
import { type ConfigMode } from "./config_mode";
import type { RenderProgress } from "./task/status/progress";

export type RenderTask = {
  name: String;
  progressChannel: Channel<RenderProgress>;
  renderConfig: ConfigMode;
};

export type RenderTaskForExport = {
  name: String;
  render_config: ConfigMode;
};

export const taskForExport = (task: RenderTask): RenderTaskForExport => {
  let prepared: RenderTaskForExport = {
    name: task.name,
    render_config: task.renderConfig,
  };
  return prepared;
};

export const taskForImport = (task: RenderTaskForExport): RenderTask => {
  let prepared: RenderTask = {
    name: task.name,
    progressChannel: new Channel<RenderProgress>(),
    renderConfig: task.render_config,
  };
  return prepared;
};

export function new_task(renderConfig: ConfigMode): RenderTask {
  let name =
    renderConfig.mode[0].toUpperCase() + renderConfig.mode.substring(1);
  let progressChannel = new Channel<RenderProgress>();
  return {
    name,
    progressChannel,
    renderConfig,
  };
}
