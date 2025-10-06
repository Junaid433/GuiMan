<template>
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl w-full max-w-md">
      <div class="p-6">
        <div class="flex items-center gap-4">
          <div :class="[
            'flex-shrink-0 w-12 h-12 rounded-full flex items-center justify-center',
            type === 'danger' ? 'bg-red-100 dark:bg-red-900' : 'bg-yellow-100 dark:bg-yellow-900'
          ]">
            <svg v-if="type === 'danger'" class="w-6 h-6 text-red-600 dark:text-red-400" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
            </svg>
            <svg v-else class="w-6 h-6 text-yellow-600 dark:text-yellow-400" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" />
            </svg>
          </div>
          <div class="flex-1">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">{{ title }}</h3>
            <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">{{ message }}</p>
          </div>
        </div>
      </div>

      <div class="bg-gray-50 dark:bg-gray-700 px-6 py-4 flex justify-end gap-3 rounded-b-lg">
        <button @click="$emit('cancel')" class="px-4 py-2 bg-gray-600 hover:bg-gray-700 text-white rounded-lg font-medium transition-colors">
          Cancel
        </button>
        <button @click="$emit('confirm')" :class="[
          'px-4 py-2 text-white rounded-lg font-medium transition-colors',
          type === 'danger' ? 'bg-red-600 hover:bg-red-700' : 'bg-blue-600 hover:bg-blue-700'
        ]">
          {{ confirmText }}
        </button>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'ConfirmDialog',
  props: {
    title: String,
    message: String,
    confirmText: {
      type: String,
      default: 'Confirm'
    },
    type: {
      type: String,
      default: 'warning',
      validator: (value) => ['warning', 'danger'].includes(value)
    }
  },
  emits: ['confirm', 'cancel']
}
</script>

