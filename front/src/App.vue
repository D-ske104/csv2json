<script setup lang="ts">
import { ref } from 'vue';
import { csv_to_json } from '../../wasm/pkg';
import FileUploader from './components/FileUploader.vue';
import JsonViewer from './components/JsonViewer.vue';
import LoadingIndicator from './components/LoadingIndicator.vue';

const output = ref("");
const fileName = ref("");
const isLoading = ref(false);

async function processFile(file: File) {
  isLoading.value = true;
  fileName.value = file.name.replace(/\.csv$/, '');
  try {
    const text = await file.text();
    output.value = csv_to_json(text);
  } catch (error) {
    console.error("Error reading file:", error);
    output.value = `エラーが発生しました: ${error instanceof Error ? error.message : '不明なエラー'}`;
  } finally {
    isLoading.value = false;
  }
}
</script>

<template>
  <div class="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-4xl mx-auto">
      <h1 class="text-3xl font-bold text-center text-gray-900 mb-8">CSV to JSON コンバーター</h1>
      
      <FileUploader @file-dropped="processFile" />
      <LoadingIndicator :isLoading="isLoading" />
      <JsonViewer :output="output" :fileName="fileName" />
    </div>
  </div>
</template>

<style scoped>
/* ドラッグ中のアニメーション効果 */
.border-dashed {
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0% {
    box-shadow: 0 0 0 0 rgba(59, 130, 246, 0.3);
  }
  70% {
    box-shadow: 0 0 0 10px rgba(59, 130, 246, 0);
  }
  100% {
    box-shadow: 0 0 0 0 rgba(59, 130, 246, 0);
  }
}
</style>
