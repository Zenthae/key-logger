<script lang="ts" setup>
interface Props {
  symbole: string;
}

const { symbole } = defineProps<Props>();
</script>

<template>
  <div class="key">
    {{ symbole }}
  </div>
</template>

<style scoped>
.key {
  border: 1px black solid;
  border-radius: 15%;
  /* width: 50px;
  height: 50px; */
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
