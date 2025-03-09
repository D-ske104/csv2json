<template>
  <div 
    class="bg-white rounded-lg shadow-md p-6 mb-6 transition-all duration-200"
    @dragover="handleDragOver"
    @dragleave="handleDragLeave"
    @drop="handleDrop"
    :class="{ 'bg-blue-50 border-2 border-blue-300 border-dashed': isDragging }"
  >
    <label class="block text-sm font-medium text-gray-700 cursor-pointer">
      <div class="flex flex-col items-center justify-center p-6 border-2 border-gray-300 border-dashed rounded-md hover:border-indigo-500 transition-colors duration-150">
        <img :src="addLine" alt="ファイルを追加" class="w-16 h-16 mb-4" />
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
</template>

<script setup lang="ts">
import { ref } from 'vue';
import addLine from '../assets/add_line.svg';

const emit = defineEmits(['file-dropped']);

const isDragging = ref(false);

function handleDragOver(event: DragEvent) {
  event.preventDefault();
  isDragging.value = true;
}

function handleDragLeave(event: DragEvent) {
  event.preventDefault();
  isDragging.value = false;
}

function handleDrop(event: DragEvent) {
  event.preventDefault();
  isDragging.value = false;
  const files = event.dataTransfer?.files;
  if (files && files.length > 0) {
    emit('file-dropped', files[0]);
  }
}

function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement;
  const files = target.files;
  if (files && files.length > 0) {
    emit('file-dropped', files[0]);
  }
}
</script>
