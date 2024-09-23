import { defineStore } from "pinia";
import { Task } from "../tauri_commands/tasks";

export type TasksState = {
  tasks: Task[];
  sortedTasks: Task[];
  selectedTask: Task;
};

export const useTasksStore = defineStore("tasks", {
  state: () => {
    return {
      tasks: [],
      sortedTasks: [],
      selectedTask: new Task(),
    } as TasksState;
  },
  actions: {
    setTasks(items: Task[]) {
      this.tasks = items;
      this.sortedTasks = items;
      this.sortedTasks.sort((x, y) => {
        if (x.status == "DONE" && y.status != "DONE") return 1;
        if (y.status == "DONE" && x.status != "DONE") return -1;
        if (x.priority == "URGENT" && y.priority != "URGENT") return -1;
        if (y.priority == "URGENT" && x.priority != "URGENT") return 1;

        return 0;
      });
    },
    setSelectedTask(value: Task) {
      this.selectedTask = value;
    },
  },
});
