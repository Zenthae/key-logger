<script setup lang="ts">
import { invoke } from "@tauri-apps/api";

function test() {
  invoke("get_event_by_id").then((r) => console.log(r));
}
</script>

<template>
  <div>
    <button @click="test"></button>
  </div>
</template>

<style scoped></style>
