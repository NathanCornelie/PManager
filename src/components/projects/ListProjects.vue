<template>
  <div>
    <div v-for="(project,index) in list_projects" :key="index" class="product_card" >
      <v-card  :title="project.name" :subtitle="project.path" color="indigo"></v-card>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from "vue";

const list_projects = ref<{ name: string; path: string }[]>([
  { name: "Project 1", path: "/project1" },
  { name: "Project 2", path: "/project2" },
]);
</script>

<style lang="scss" scoped>
.product_card{
  margin-bottom: 5px;
}
</style>
