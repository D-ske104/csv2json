<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { csv_to_json } from '../../wasm/pkg';
import add_line from './assets/add_line.svg'

const output = ref("")
const fileName = ref("")
const isDragging = ref(false)

// 共通のファイル処理関数
async function processFile(file: File) {
  // Save original filename without extension
  fileName.value = file.name.replace(/\.csv$/, '');

  const text = await file.text();
  let json;
  try {
    json = csv_to_json(text);
  } catch (error) {
    console.error("Error reading file:", error);
    return;
  }
  
  output.value = json
}

async function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement;
  const files = target.files;
  if (!files || files.length === 0) return;
  await processFile(files[0]);
}

// ドラッグアンドドロップ用のハンドラ
function handleDragOver(event: DragEvent) {
  event.preventDefault();
  isDragging.value = true;
}

function handleDragLeave(event: DragEvent) {
  event.preventDefault();
  isDragging.value = false;
}

async function handleDrop(event: DragEvent) {
  event.preventDefault();
  isDragging.value = false;
  
  const files = event.dataTransfer?.files;
  if (!files || files.length === 0) return;
  
  // CSVファイルのみを処理
  const file = files[0];
  if (!file.name.endsWith('.csv')) {
    alert('CSVファイルのみ対応しています');
    return;
  }
  
  await processFile(file);
}

function downloadJson() {
  // Create blob from JSON string
  const blob = new Blob([output.value], { type: 'application/json' });
  
  // Create download URL
  const url = URL.createObjectURL(blob);
  
  // Create and trigger download link
  const a = document.createElement('a');
  a.href = url;
  a.download = `${fileName.value || 'converted'}.json`;
  document.body.appendChild(a);
  a.click();
  
  // Clean up
  setTimeout(() => {
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }, 100);
}
</script>

<template>
  <div class="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-4xl mx-auto">
      <h1 class="text-3xl font-bold text-center text-gray-900 mb-8">CSV to JSON コンバーター</h1>
      
      <div 
        class="bg-white rounded-lg shadow-md p-6 mb-6 transition-all duration-200"
        @dragover="handleDragOver"
        @dragleave="handleDragLeave"
        @drop="handleDrop"
        :class="{ 'bg-blue-50 border-2 border-blue-300 border-dashed': isDragging }"
      >
        <label class="block text-sm font-medium text-gray-700 cursor-pointer">
          <div class="flex flex-col items-center justify-center p-6 border-2 border-gray-300 border-dashed rounded-md hover:border-indigo-500 transition-colors duration-150">
            <img :src="add_line" alt="ファイルを追加" class="w-16 h-16 mb-4" />
            <div class="text-center">
              <p class="text-lg font-medium">CSVファイルをドラッグ＆ドロップ</p>
              <p class="text-sm text-gray-500 mt-1">またはクリックしてファイルを選択</p>
            </div>
            <input 
              type="file" 
              accept=".csv" 
              @change.prevent="handleFileChange"
              class="sr-only"
            />
          </div>
        </label>
      </div>
      
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
