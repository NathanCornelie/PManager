<template>
  <v-overlay v-model="overlay" class="d-flex align-center justify-center" on>
    <v-card style="width: 500px" class="pa-5">
      <div class="">
        <form @submit.prevent="submitForm">
          <v-text-field v-model="editedTask.name" label="Name"></v-text-field>
          <div class="d-flex justify-space-between">
            <v-autocomplete
              class="pr-1"
              ref="autocomplete_project"
              v-model="project"
              :onchange="(e:Project)=>{if(tasksStore.selectedTask)tasksStore.selectedTask.value.project_id = e.id}"
              label="Projet"
              :items="[emptyProject, ...list_projects]"
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
            <v-autocomplete
              class="pl-2"
              v-model="editedTask.priority"
              label="Priority"
              :items="PRIORITY"
              variant="outlined"
              chips
            >
              <template v-slot:chip="{ item: item_priority }">
                {{ item_priority.raw }}
              </template>
            </v-autocomplete>
          </div>

          <v-textarea
            v-model="editedTask.description"
            label="Description"
          ></v-textarea>

          <div>
            <v-checkbox
              density="compact"
              v-for="sub in editedTask.subtasks"
              :key="sub.id"
              :label="sub.value"
              v-model="sub.done"
              hide-details
            ></v-checkbox>
          </div>

          <div
            v-if="mode == TypeVals.edit"
            class="d-flex justify-space-between"
          >
            <div>
              <v-btn
                class="me-4"
                @click="deleteTask()"
                style="background-color: #f44336"
              >
                Delete
              </v-btn>
              <v-btn
                class="me-4"
                v-if="mode == TypeVals.edit"
                @click="editTask()"
                style="background-color: #2196f3"
              >
                Save
              </v-btn>
            </div>
            <v-btn
              class="me-4"
              v-if="mode == TypeVals.edit"
              @click="validateTask()"
              style="background-color: #69f0ae"
            >
              Done
            </v-btn>
          </div>
          <v-btn class="me-4" v-else type="submit"> Create </v-btn>
        </form>
      </div>
    </v-card>
  </v-overlay>
</template>

<script lang="ts" setup>
import { onMounted, ref, watch } from "vue";
import TasksCommand, { SubTask, Task } from "../../tauri_commands/tasks";
import { Project } from "../../tauri_commands/projects";
import { useTasksStore } from "../../stores/tasks";
import { storeToRefs } from "pinia";
import { useProjectStore } from "../../stores/projects";

defineExpose({
  openModale,
});

enum TypeVals {
  create,
  edit,
}
defineProps({
  list_projects: Array<Project>,
});
const emit = defineEmits(["close"]);
const tasksStore = storeToRefs(useTasksStore());
const projectStore = useProjectStore();
const mode = ref<TypeVals>(TypeVals.create);
const project = ref<Project | undefined>(undefined);
const overlay = ref<boolean>(false);
const PRIORITY = ref<String[]>(["URGENT", "IMPORTANT", "NORMAL"]);
const emptyProject = ref<Project>(new Project(0, "Pas de projet"));
const editedTask = ref<Task>(new Task());

watch(overlay, () => {
  if (!overlay.value) {
    emit("close");
  }
  editedTask.value.subtasks = [
    new SubTask(1, false, "Substask 1"),
    new SubTask(2, false, "Substask 2"),
    new SubTask(3, false, "Substask 3"),
  ];
});
watch(project, () => {
  editedTask.value.project_id = project.value?.id || 0;
});
onMounted(() => {
  editedTask.value = JSON.parse(
    JSON.stringify(tasksStore.selectedTask.value)
  ) as Task;
  editedTask.value.subtasks = [
    new SubTask(1, false, "Substask 1"),
    new SubTask(2, false, "Substask 2"),
    new SubTask(3, false, "Substask 3"),
  ];
});

watch(
  () => tasksStore.selectedTask.value,
  (newValue) => {
    editedTask.value = JSON.parse(JSON.stringify(newValue)) as Task;
  }
);
function openModale(value: string) {
  if (value === "edit") mode.value = TypeVals.edit;
  else mode.value = TypeVals.create;
  if (tasksStore.selectedTask.value.project_id) {
    project.value = projectStore.projects.find(
      (e) => e.id == tasksStore.selectedTask.value.project_id
    );
  } else {
    project.value = undefined;
  }
  overlay.value = true;
}
async function submitForm() {
  if (tasksStore.selectedTask.value) {
    TasksCommand.create_task(
      new Task(
        editedTask.value.id,
        editedTask.value.name,
        editedTask.value.description,
        editedTask.value.project_id,
        editedTask.value?.priority,
        editedTask.value?.status
      )
    ).then(() => (overlay.value = false));
  }
}
async function deleteTask() {
  if (tasksStore.selectedTask.value) {
    TasksCommand.delete_task(tasksStore.selectedTask.value.id).then(
      () => (overlay.value = false)
    );
  }
}
async function editTask() {
  if (editedTask.value.id) {
    TasksCommand.update_task(editedTask.value).then(
      () => (overlay.value = false)
    );
  }
}
async function validateTask() {
  if (editedTask.value.id) {
    editedTask.value.status = "DONE";
    TasksCommand.update_task(editedTask.value).then(
      () => (overlay.value = false)
    );
  }
}
</script>

<style></style>
