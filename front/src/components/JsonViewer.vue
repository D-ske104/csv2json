<template>
  <div v-if="output" class="space-y-4">
    <div class="flex justify-between items-center">
      <h2 class="text-xl font-semibold text-gray-700">変換結果</h2>
      <div class="flex space-x-2">
        <button 
          @click="copyToClipboard" 
          class="bg-green-600 hover:bg-green-700 text-white font-medium py-2 px-4 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 transition duration-150"
        >
          コピー
        </button>
        <button 
          @click="downloadJson" 
          class="bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition duration-150"
        >
          JSONをダウンロード
        </button>
      </div>
    </div>
    
    <div class="bg-gray-800 rounded-lg shadow-lg">
      <pre class="p-4 text-gray-100 overflow-auto max-h-96"><code ref="jsonCode">{{ formattedOutput }}</code></pre>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';
import hljs from 'highlight.js';
import 'highlight.js/styles/github-dark.css';

const props = defineProps<{
  output: string;
  fileName: string;
}>();

const jsonCode = ref<HTMLElement | null>(null);
const formattedOutput = ref('');

watch(() => props.output, (newOutput) => {
  formattedOutput.value = JSON.stringify(JSON.parse(newOutput), null, 2);
  highlightJson();
});

onMounted(() => {
  highlightJson();
});

function highlightJson() {
  if (jsonCode.value) {
    hljs.highlightBlock(jsonCode.value);
  }
}

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

function copyToClipboard() {
  navigator.clipboard.writeText(props.output).then(() => {
    alert('JSONがクリップボードにコピーされました。');
  }).catch((err) => {
    console.error('クリップボードへのコピーに失敗しました:', err);
  });
}
</script>
