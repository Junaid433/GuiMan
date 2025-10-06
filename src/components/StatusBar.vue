<template>
  <div class="bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700 px-4 py-3">
    <div class="flex items-center justify-between">
      <div class="text-sm text-gray-600 dark:text-gray-400">
        <span v-if="selectedCount > 0">
          {{ selectedCount }} package{{ selectedCount > 1 ? 's' : '' }} selected
        </span>
        <span v-else>No packages selected</span>
      </div>

      <div v-if="selectedCount > 0" class="flex gap-3">
        <button
          @click="$emit('install-selected')"
          class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium rounded-lg transition-colors flex items-center gap-2"
        >
          <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M10 5a1 1 0 011 1v3h3a1 1 0 110 2h-3v3a1 1 0 11-2 0v-3H6a1 1 0 110-2h3V6a1 1 0 011-1z" clip-rule="evenodd" />
          </svg>
          Install Selected
        </button>
        
        <button
          @click="$emit('remove-selected')"
          class="px-4 py-2 bg-red-600 hover:bg-red-700 text-white text-sm font-medium rounded-lg transition-colors flex items-center gap-2"
        >
          <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd" />
          </svg>
          Remove Selected
        </button>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'StatusBar',
  props: {
    selectedCount: Number
  },
  emits: ['install-selected', 'remove-selected']
}
</script>

