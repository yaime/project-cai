<script setup lang="ts">
import { NConfigProvider, NMessageProvider, NDialogProvider, NGlobalStyle, darkTheme } from 'naive-ui';
import { ref } from 'vue';

const theme = ref(darkTheme);
</script>

<template>
  <n-config-provider :theme="theme">
    <n-global-style />
    <n-message-provider>
      <n-dialog-provider>
        <div class="app-layout">
          <h1>Financial System Init</h1>
        </div>
      </n-dialog-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<style>
html, body, #app {
  height: 100%;
  margin: 0;
}
.app-layout {
  height: 100%;
  display: flex;
  flex-direction: column;
}
</style>
