import { defineStore } from "pinia";
import { Project } from "../tauri_commands/projects";

export type ProjectsState = {
  projects: Project[];
};
export const useProjectStore = defineStore("project", {
  state: () => {
    return {
      projects: [],
    } as ProjectsState;
  },
  actions: {
    setProjects(items: Project[]) {
      this.projects = items;
    },
  },
});
