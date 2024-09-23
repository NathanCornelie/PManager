<template>
  {{ sortedListTasks[0] }}
  <div
    v-for="(task, index) in taskStore.sortedTasks"
    :key="index"
    class="product_card"
  >
    <v-card class="card" :color="getCardColor(task)" @click="editTask(task)">
      <v-card-title>
        <div class="card_head">
          <p>{{ task.name }}</p>
          <p>{{ renderTasksProjectName(task.project_id) }}</p>
        </div>
      </v-card-title>
      <v-card-subtitle style="font-weight: bold">{{
        task.description
      }}</v-card-subtitle>
    </v-card>
  </div>
</template>
<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { useProjectStore } from "../../stores/projects";
import { Task } from "../../tauri_commands/tasks";
import { useTasksStore } from "../../stores/tasks";

const sortedListTasks = ref<Task[]>([]);

const taskStore = useTasksStore();
const emit = defineEmits(["displayEditModale"]);
const projectStore = useProjectStore();

onMounted(() => {
  sortedListTasks.value = taskStore.tasks;
  console.log(sortedListTasks.value.length);
  sortPropsList();
});

watch(taskStore.tasks, (newV) => {
  sortedListTasks.value = newV.map((e) => e);

  sortPropsList();
});
function sortPropsList() {
  sortedListTasks.value.sort((x, y) => {
    if (x.status == "DONE" && y.status != "DONE") return 1;
    if (y.status == "DONE" && x.status != "DONE") return -1;
    if (x.priority == "URGENT" && y.priority != "URGENT") return -1;
    if (y.priority == "URGENT" && x.priority != "URGENT") return 1;

    return 0;
  });
}
function renderTasksProjectName(project_id: number | null) {
  if (project_id)
    return (
      projectStore.projects.filter((e) => e.id == project_id)[0]?.name || ""
    );
  else return "";
}
function getCardColor(task: Task) {
  switch (task.priority) {
    case "URGENT":
      if (task.status == "DONE") return "red-lighten-3";
      return "red";
    case "IMPORTANT":
      if (task.status == "DONE") return "orange-lighten-3";
      return "orange-darken-2";
    default:
      if (task.status == "DONE") return "green-lighten-3";
      return "light-blue-darken-2";
  }
}
function editTask(task: Task) {
  emit("displayEditModale", task);
}
</script>

<style scoped lang="scss">
.product_card {
  margin-bottom: 5px;
}

.card {
  padding: 7px;
}

.card_head {
  display: flex;
  justify-content: space-between;
}
</style>
