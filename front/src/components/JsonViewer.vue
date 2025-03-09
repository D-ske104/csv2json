<template>
  <div v-if="output" class="space-y-4">
    <div class="flex justify-between items-center">
      <h2 class="text-xl font-semibold text-gray-700">変換結果</h2>
      <button 
        @click="downloadJson" 
        class="bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition duration-150"
      >
        JSONをダウンロード
      </button>
    </div>
    
    <div class="bg-gray-800 rounded-lg shadow-lg">
      <pre class="p-4 text-gray-100 overflow-auto max-h-96">{{ output }}</pre>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';

const props = defineProps<{
  output: string;
  fileName: string;
}>();

function downloadJson() {
  const blob = new Blob([props.output], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `${props.fileName || 'converted'}.json`;
  document.body.appendChild(a);
  a.click();
  setTimeout(() => {
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }, 100);
}
</script>
