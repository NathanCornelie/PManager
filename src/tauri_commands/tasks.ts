import { invoke } from "@tauri-apps/api";

export class Task {
  id: number = 0;
  name: string = "";
  description: string = "";
  project_id: number = 0;
  priority: string = "";
  status: string = "";
  subtasks: SubTask[] = [];
  constructor(
    id: number = 0,
    name: string = "",
    description: string = "",
    project_id: number = 0,
    priority: string = "",
    status: string = "",
    subtasks: SubTask[] = []
  ) {
    this.id = id;
    this.name = name;
    this.description = description;
    this.project_id = project_id;
    this.priority = priority;
    this.status = status;
    this.subtasks = subtasks;
  }
  /**
   * cop : return a new object with the fields matched
   */
  public copy() {
    return new Task(
      this.id,
      this.name,
      this.description,
      this.project_id,
      this.priority,
      this.status
    );
  }
}

export class SubTask {
  id: number = 0;
  done: boolean = false;
  value: string = "";
  constructor(id: number = 0, done: boolean = false, value: string = "") {
    this.id = id;
    this.done = done;
    this.value = value;
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
    return invoke("delete_task", { taskId: id.toString() });
  }
  static async update_task(task: Task): Promise<Task> {
    return invoke("update_task", {
      task: task,
    });
  }
}
