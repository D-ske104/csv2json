<template>
  <div class="settings-panel bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md mb-6">
    <h2 class="text-xl font-semibold text-gray-700 dark:text-gray-100 mb-4">設定</h2>
    <div class="mb-4">
      <label for="delimiter" class="block text-sm font-medium text-gray-700 dark:text-gray-300">区切り文字</label>
      <select id="delimiter" v-model="delimiter" class="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 dark:border-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md">
        <option value=",">カンマ (,)</option>
        <option value=";">セミコロン (;)</option>
        <option value="\t">タブ (\\t)</option>
      </select>
    </div>
    <div class="mb-4">
      <label for="columns" class="block text-sm font-medium text-gray-700 dark:text-gray-300">含める列</label>
      <input type="text" id="columns" v-model="columnsInput" @input="updateSelectedColumns" placeholder="列名をカンマで区切って入力" class="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 dark:border-gray-700 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md" />
    </div>
    <div class="flex items-center mb-4">
      <input id="darkMode" type="checkbox" v-model="darkMode" @change="toggleDarkMode" class="h-4 w-4 text-indigo-600 dark:text-indigo-500 focus:ring-indigo-500 border-gray-300 dark:border-gray-700 rounded" />
      <label for="darkMode" class="ml-2 block text-sm font-medium text-gray-700 dark:text-gray-300">ダークモード</label>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';

const props = defineProps<{
  delimiter: string;
  selectedColumns: string[];
  darkMode: boolean;
}>();

const emit = defineEmits(['update:delimiter', 'update:selectedColumns', 'update:darkMode', 'toggle-dark-mode']);

const delimiter = ref(props.delimiter);
const columnsInput = ref(props.selectedColumns.join(', '));
const darkMode = ref(props.darkMode);

watch(delimiter, (newDelimiter) => {
  emit('update:delimiter', newDelimiter);
});

watch(columnsInput, (newColumnsInput) => {
  const columns = newColumnsInput.split(',').map(col => col.trim());
  emit('update:selectedColumns', columns);
});

watch(darkMode, (newDarkMode) => {
  emit('update:darkMode', newDarkMode);
});

function updateSelectedColumns() {
  const columns = columnsInput.value.split(',').map(col => col.trim());
  emit('update:selectedColumns', columns);
}

function toggleDarkMode() {
  emit('toggle-dark-mode');
}
</script>

<style scoped>
.settings-panel {
  transition: background-color 0.3s, color 0.3s;
}
</style>
