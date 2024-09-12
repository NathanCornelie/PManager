import { invoke } from "@tauri-apps/api";
export class Project {
  id: number = 0;
  name: string = "";
  path: string = "";
  description: string = "";
  constructor(name: string, path: string, description: string) {
    this.name = name;
    this.description = description;
    this.path = path;
  }
}

export default abstract class ProjectsCommand {
  static async get_projects(): Promise<Project[]> {
    return  invoke("get_projects");
  }

  static async create_project(project: Project): Promise<Project> {
    return  invoke("create_project", {
      name: project.name,
      path: project.path,
      description: project.description,
    });
  }

  static async remove_project(id: number): Promise<number> {
    return invoke("remove_project", { id });
  }

  static async update_project(project: Project): Promise<Project> {
    return invoke("update_project", { project });
  }
}
