<template>
  <v-dialog v-model="isActive" max-width="1800">
    <v-card
      style="align-self: center"
      class="w-75 h-75"
      :subtitle="projectStore.selectedProject?.description"
    >
      <template v-slot:title="">
        <div class="d-flex align-center justify-center">
          <h3>{{ projectStore.selectedProject?.name }}</h3>
          <p class="ml-4 text-grey">{{ projectStore.selectedProject?.path }}</p>
        </div>
      </template>
      <template v-slot:text="">
        <div>
          <div class="d-flex justify-space-between">
            <div class="w-60">
              <v-card :title="displayedTask?.name" elevation="0">
                <v-card-text>
                  <p>
                    {{ displayedTask?.description }}
                  </p>
                </v-card-text>
              </v-card>
            </div>
            <!-- liste des tache  -->
            <div class="w-33">
              <v-list>
                <v-list-item
                  v-for="task in taskStore.tasks"
                  :key="task.id"
                  @click="displayTask(task)"
                  :title="task.name"
                  :subtitle="task.description"
                >
                </v-list-item>
              </v-list>
            </div>
          </div>
        </div>
      </template>
      <template v-slot:actions="">
        <v-btn @click="isActive = false">Close</v-btn>
      </template>
    </v-card>
  </v-dialog>
</template>

<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { useProjectStore } from "../../stores/projects";
import { useTasksStore } from "../../stores/tasks";
import { Task } from "../../tauri_commands/tasks";

defineExpose({ openDialog });

const isActive = ref<boolean>(false);
const projectStore = useProjectStore();
const taskStore = useTasksStore();
const displayedTask = ref<Task | null>(null);

onMounted(() => {
  displayedTask.value = taskStore.tasks[0];
});

function openDialog() {
  isActive.value = true;
}
function displayTask(task: Task) {
  displayedTask.value = task;
}
</script>

<style></style>
