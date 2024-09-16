import { defineStore } from "pinia";
import { Task } from "../tauri_commands/tasks";

export type TasksState = {
  tasks: Task[];
  selectedTask: Task;
};

export const useTasksStore = defineStore("tasks", {
  state: () => {
    return {
      tasks: [],
      selectedTask: new Task(),
    } as TasksState;
  },
  actions: {
    setTasks(items: Task[]) {
      this.tasks = items;
    },
    setSelectedTask(value: Task) {
      this.selectedTask = value;
    },
  },
});
