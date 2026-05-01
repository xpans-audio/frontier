use serde::{Deserialize, Serialize};
use xpans_renderconfig::RenderConfig;
use xpans_taskrenderer::RenderTask;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderTaskForExport {
    pub name: String,
    pub render_config: RenderConfig,
}

impl RenderTaskForExport {
    pub fn into_task(&self) -> RenderTask {
        RenderTask {
            name: self.name.clone(),
            config: self.render_config,
        }
    }
    pub fn from_task(task: &RenderTask) -> Self {
        Self {
            name: task.name.clone(),
            render_config: task.config,
        }
    }
}
