<template>
  <div class="tasks">
    <div class="search">
      <v-text-field hide-details="true" label="Tasks" placeholder="Tasks ..." variant="outlined"></v-text-field>
    </div>
    <ListTasks/>
  </div>
</template>

<script lang="ts" setup>

import ListTasks from "./ListTasks.vue";
</script>

<style scoped lang="scss">
.tasks {
  width: 100%;
  margin: 40px;
}

.search {
  margin-bottom: 10px;
}
</style>