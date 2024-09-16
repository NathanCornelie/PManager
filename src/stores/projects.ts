import { defineStore } from "pinia";
import { Project } from "../tauri_commands/projects";

export type ProjectsState = {
  projects: Project[];
  selectedProject: Project | null;
};
export const useProjectStore = defineStore("project", {
  state: () => {
    return {
      projects: [],
      selectedProject: null,
    } as ProjectsState;
  },
  actions: {
    setProjects(items: Project[]) {
      this.projects = items;
    },
    setSelectedProject(item: Project) {
      this.selectedProject = item;
    },
  },
});
