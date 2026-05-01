type Rendering = {
  status: "rendering";
  progress: number;
};

type Error = {
  status: "error";
};

type Finished = {
  status: "finished";
};

export type RenderProgress = Rendering | Finished | Error;
