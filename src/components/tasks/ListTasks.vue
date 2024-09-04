<script setup lang="ts">
import {ref} from "vue";

const list_tasks = ref<{ name: string; project: string; description: string }[]>([
  {name: "Fix le header 1", project: "AppStock", description: "Project 1"},
  {name: "Project 2", project: "SudokuSolver", description: "Project 2"},
]);
</script>

<template>
  <div v-for="(task,index) in list_tasks" :key="index" class="product_card">
    <v-card class="card" color="indigo">


      <v-card-title>
        <div class="card_head">
          <p>{{ task.name }}</p>
          <p> {{ task.project }}</p>
        </div>
      </v-card-title>
      <v-card-subtitle>{{task.description}}</v-card-subtitle>
    </v-card>
  </div>
</template>

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