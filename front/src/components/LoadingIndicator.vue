<template>
  <div v-if="isLoading" class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50 z-50">
    <div class="bg-white p-6 rounded-lg shadow-xl flex flex-col items-center">
      <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500 mb-4"></div>
      <p class="text-lg font-medium text-gray-700">変換処理中...</p>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps({
  isLoading: Boolean
});
</script>
