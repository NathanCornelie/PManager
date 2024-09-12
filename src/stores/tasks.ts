import { defineStore } from "pinia";
import { Task } from "../tauri_commands/tasks";

export type TasksState = {
  tasks: Task[];
};

export const useTasksStore = defineStore("tasks", {
  state: () => {
    return {
      tasks: [],
    } as TasksState;
  },
  actions: {
    setTasks(items: Task[]) {
      this.tasks = items;
    },
  },
});
