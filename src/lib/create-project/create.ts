import type { Project } from "../project";

export const notReadyToCreate = (
  project: Project,
): boolean => {
  return (
    project.audio == "" ||
    project.spatial == "" ||
    project.title == ""
  );
};
