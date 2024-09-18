import { invoke } from "@tauri-apps/api";

export class Task {
  id: number = 0;
  name: string = "";
  priority: string = "";
  project_id: number = 0;
  description: string = "";
  constructor(
    name: string = "",
    project_id: number = 0,
    priority: string = "",
    description: string = ""
  ) {
    this.priority = priority;
    this.name = name;
    this.description = description;
    this.project_id = project_id;
  }
}

export default abstract class TasksCommand {
  static async get_tasks(): Promise<Task[]> {
    return invoke("get_tasks");
  }
  static async create_task(task: Task): Promise<Task> {
    return await invoke("create_task", {
      name: task.name,
      description: task.description,
      projectId: task.project_id?.toString() || "null",
    });
  }
  static async delete_task(id: number): Promise<number> {
    return invoke("delete_task", { taskId: id.toString()});
  }
  static async update_task(task: Task): Promise<Task> {
    return invoke("update_task", {
      task: task,
    });
  }
}
