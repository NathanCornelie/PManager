<template>
  <div class="tasks">
    <div class="search">
      <v-text-field
        :hide-details="true"
        label="Tasks"
        placeholder="Tasks ..."
        variant="outlined"
      ></v-text-field>
      <v-btn @click="openCreateModale()">
        <v-icon size="x-large" icon="mdi-plus"></v-icon>
      </v-btn>
    </div>
    <ListTasks :list_tasks="tasksStore.tasks" />
    <CreateTask
      :list_projects="projectsStore.projects"
      ref="creteModale"
      @close="handleModaleClose()"
    />
  </div>
</template>

<script lang="ts" setup>
import ListTasks from "./ListTasks.vue";
import CreateTask from "./CreateTask.vue";
import { onMounted, useTemplateRef } from "vue";
import TasksCommand from "../../tauri_commands/tasks";
import { useProjectStore } from "../../stores/projects";
import { useTasksStore } from "../../stores/tasks";

const tasksStore = useTasksStore();
const projectsStore = useProjectStore();

const ref_modal = useTemplateRef("creteModale");
onMounted(async () => {
  await updateListTasks();
});
function openCreateModale() {
  ref_modal.value?.openModale();
}
async function handleModaleClose() {
  await updateListTasks();
}
async function updateListTasks() {
  tasksStore.setTasks(await TasksCommand.get_tasks());
}
</script>

<style scoped lang="scss">
.tasks {
  width: 100%;
  margin: 40px;
}

.search {
  margin-bottom: 10px;
  display: flex;
  align-items: center;
}
</style>
