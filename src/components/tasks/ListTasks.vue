<template>
  <div v-for="(task, index) in list_tasks" :key="index" class="product_card">
    <v-card class="card" color="indigo">
      <v-card-title>
        <div class="card_head">
          <p>{{ task.name }}</p>

          <p>{{ renderTasksProjectName(task.project_id) }}</p>
        </div>
      </v-card-title>
      <v-card-subtitle>{{ task.description }}</v-card-subtitle>
    </v-card>
  </div>
</template>
<script setup lang="ts">
import { useProjectStore } from "../../stores/projects";
import { Task } from "../../tauri_commands/tasks";

defineProps({
  list_tasks: Array<Task>,
});

const projectStore = useProjectStore();
function renderTasksProjectName(project_id: number | null) {
  if (project_id)
    return (
      projectStore.projects.filter((e) => e.id == project_id)[0]?.name || ""
    );
  else return "";
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
