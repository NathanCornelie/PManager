<template>
  <div class="project">
    <div class="search">
         <v-text-field  hide-details="true" label="Project" placeholder="Project ..." variant="outlined"></v-text-field>
    </div>
    <div>
        <ListProjects />
    </div>
  </div>
</template>

<script lang="ts" setup>
import ListProjects from './ListProjects.vue'

</script>

<style scoped lang="scss">
.project{
  width: 100%;
  margin: 40px ;

}
.search{
  margin-bottom: 10px;
}

</style>