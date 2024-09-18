<template>
  <div>
    <div
      v-for="(project, index) in list_projects"
      :key="index"
      class="product_card"
    >
      <v-card
        :title="project.name"
        :subtitle="project.path"
        @click="displayProject(project)"
        color="indigo"
      ></v-card>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { Project } from "../../tauri_commands/projects";

defineProps({
  list_projects: Array<Project>,
});
const emit = defineEmits(["openDisplayModale"]);
function displayProject(project: Project) {
  emit("openDisplayModale", project);
}
</script>

<style lang="scss" scoped>
.product_card {
  margin-bottom: 5px;
}
</style>
