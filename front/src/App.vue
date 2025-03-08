<script setup lang="ts">
import { onMounted, ref } from 'vue';
import init, { csv_to_json } from '../../wasm/pkg/wasm.js';

onMounted(async () => {
  await init();
});

const output = ref("")

async function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement;
  const files = target.files;
  if (!files || files.length === 0) return;
  const file = files[0];

  const text = await file.text();
  const json = csv_to_json(text);
  
  output.value = json
}
</script>

<template>
  <input type="file" accept=".csv" @change.prevent="handleFileChange" />
  <pre>{{ output }}</pre>
</template>

