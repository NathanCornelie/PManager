import { invoke } from "@tauri-apps/api";

export class Task {
  id: number = 0;
  name: string = "";
  project_id: number | null = null;
  description: string = "";
  constructor(name: string, project_id: number | null, description: string) {
    this.name = name;
    this.description = description;
    this.project_id = project_id;
  }
}

export default abstract class TasksCommand {
  static async get_tasks(): Promise<Task[]> {
    return invoke("get_tasks");
  }
  static async add_task(task: Task): Promise<Task> {
    return await invoke("create_task", {
      name: task.name,
      description: task.description,
      projectId: task.project_id?.toString(),
    });
  }
  static async remove_task(id: number): Promise<number> {
    return invoke("remove_task", { id });
  }
  static async update_task(task: Task): Promise<Task> {
    return invoke("update_task", { task });
  }
}
