<script lang="ts" setup>
import Key from "./Key.vue";
</script>

<template>
  <div class="grid">
    <Key symbole="²" />
    <Key symbole="1" />
    <Key symbole="2" />
    <Key symbole="3" />
    <Key symbole="4" />
    <Key symbole="5" />
    <Key symbole="6" />
    <Key symbole="7" />
    <Key symbole="8" />
    <Key symbole="9" />
    <Key symbole="0" />
    <Key symbole="°" />
    <Key symbole="+" />
    <Key symbole="Backspace" />
    <Key symbole="Tab" />
    <Key symbole="A" />
    <Key symbole="Z" />
    <Key symbole="E" />
    <Key symbole="R" />
    <Key symbole="T" />
    <Key symbole="Y" />
    <Key symbole="U" />
    <Key symbole="I" />
    <Key symbole="O" />
    <Key symbole="P" />
    <Key symbole="^" />
    <Key symbole="$" />
    <Key symbole="Enter" />
  </div>
</template>

<style scoped>
.grid {
  height: 100vh;
  display: grid;
  grid-template-columns: repeat(14, minmax(0, 1fr));
  grid-template-rows: repeat(5, minmax(0, 1fr));
}
</style>
