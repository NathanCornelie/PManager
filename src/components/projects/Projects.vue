<template>
  <div class="project">
    <div class="search">
      <v-text-field
        :hide-details="true"
        label="Project"
        placeholder="Project ..."
        v-model="search_value"
        variant="outlined"
      >
      </v-text-field>

      <v-btn @click="openCreateModale()">
        <v-icon size="x-large" icon="mdi-plus"></v-icon>
      </v-btn>
    </div>

    <div>
      <ListProjects
        :list_projects="displayed_list_projects"
        @open-display-modale="openDisplayModale()"
      />
    </div>
    <CreateProject ref="createModale" @close="handleModaleClosed()" />
    <DisplayProject ref="displayDialog" @close="handleDisplayModaleCosed()" />
  </div>
</template>

<script lang="ts" setup>
import { onMounted, ref, useTemplateRef, watch } from "vue";
import ProjectsCommand, { Project } from "../../tauri_commands/projects";
import ListProjects from "./ListProjects.vue";
import CreateProject from "./CreateProject.vue";
import { useProjectStore } from "../../stores/projects";
import DisplayProject from "./DisplayProject.vue";
const projectStore = useProjectStore();

const displayDialogProject = useTemplateRef("displayDialog");
const createModale = useTemplateRef("createModale");
const displayed_list_projects = ref<Project[]>([]);
const search_value = ref<String>("");

onMounted(async () => {
  await updateListProjects();
  displayed_list_projects.value = projectStore.projects;
  projectStore.setSelectedProject(projectStore.projects[0] || null);
});

watch(search_value, (value) => {
  if (value) {
    displayed_list_projects.value = projectStore.projects.filter(
      (e) =>
        e.name.startsWith(value.valueOf()) || e.name.includes(value.valueOf())
    );
  } else {
    displayed_list_projects.value = projectStore.projects;
  }
});
async function updateListProjects() {
  projectStore.setProjects(await ProjectsCommand.get_projects());
}
function openCreateModale() {
  createModale.value?.openModale();
}
function openDisplayModale() {
  displayDialogProject.value?.openDialog();
}
async function handleModaleClosed() {
  await updateListProjects();
}
function handleDisplayModaleCosed() {}
</script>

<style scoped lang="scss">
.project {
  width: 100%;
  margin: 40px;
}
.search {
  display: flex;
  justify-content: center;
  align-items: center;
  margin-bottom: 10px;
}
</style>
