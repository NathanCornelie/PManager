<template>
  <v-overlay v-model="overlay" class="d-flex align-center justify-center" on>
    <v-card style="width: 500px" class="pa-5">
      <div class="">
        <form @submit="submitForm">
          <v-text-field v-model="name" label="Name"></v-text-field>

          <v-text-field v-model="path" label="Path"></v-text-field>

          <v-text-field
            v-model="description"
            label="Description"
          ></v-text-field>
          <v-btn class="me-4" type="submit"> Create </v-btn>
        </form>
      </div>
    </v-card>
  </v-overlay>
</template>

<script lang="ts" setup>
import { ref, watch } from "vue";
import ProjectsCommand, { Project } from "../../tauri_commands/projects";
defineExpose({
  openModale,
});
interface ProjectForm {
  name: string;
  path: string;
  description: string;
}

const emit = defineEmits(["close"]);

const overlay = ref<boolean>(false);
const name = ref("");
const path = ref("");
const description = ref("");

watch(overlay, () => {
  if (!overlay.value) {
    emit("close");
  }
});
function openModale() {
  overlay.value = true;
}

function submitForm() {
  ProjectsCommand.create_project(
    new Project(0, name.value, path.value, description.value)
  );
}
</script>

<style></style>
