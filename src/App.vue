<script setup lang="ts">
import Project from './components/projects/Projects.vue'
import Tasks from './components/tasks/Tasks.vue'
</script>

<template>
  <div class="app">

    <div class="main_container">
      <Project/>
      <Tasks/>
    </div>

  </div>
</template>

<style scoped lang="scss">
#app {
  width: 100%;
  height: 100vh;

  .main_container {

    display: flex;
    justify-content: center;

  }
}

</style>
