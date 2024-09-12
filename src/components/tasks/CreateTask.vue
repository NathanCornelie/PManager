<template>
  <v-overlay v-model="overlay" class="d-flex align-center justify-center" on>
    <v-card style="width: 500px" class="pa-5">
      <div class="">
        <form @submit.prevent="submitForm">
          <v-text-field v-model="name" label="Name"></v-text-field>
          <v-autocomplete
            v-model="project"
            label="Projet"
            :items="list_projects"
            variant="outlined"
            chips
          >
            <template v-slot:item="{ props, item }">
              <v-list-item
                v-bind="props"
                :title="item.raw.id + '-' + item.raw.name"
              ></v-list-item
            ></template>

            <template v-slot:chip="{ item }">
              {{ item.raw.name }}
            </template>
          </v-autocomplete>

          <v-textarea
            v-model="description"
            label="Description"
            clearable
          ></v-textarea>
          <v-btn class="me-4" type="submit"> Create </v-btn>
        </form>
      </div>
    </v-card>
  </v-overlay>
</template>

<script lang="ts" setup>
import { ref, watch } from "vue";
import TasksCommand, { Task } from "../../tauri_commands/tasks";
import { Project } from "../../tauri_commands/projects";

defineExpose({
  openModale,
});
defineProps({
  list_projects: Array<Project>,
});
const emit = defineEmits(["close"]);
const overlay = ref<boolean>(false);
const name = ref("");

const project = ref<Project | null>(null);
const description = ref("");

watch(overlay, () => {
  if (!overlay.value) {
    emit("close");
  }
});
function openModale() {
  overlay.value = true;
}
async function submitForm() {
  TasksCommand.add_task(
    new Task(name.value, project.value?.id || null, description.value)
  );
}
</script>

<style></style>
