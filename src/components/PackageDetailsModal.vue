<template>
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl w-full max-w-2xl max-h-[80vh] flex flex-col">
      <div class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700">
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white">{{ packageInfo.name }}</h2>
        <button @click="$emit('close')" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200">
          <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </button>
      </div>

      <div class="flex-1 overflow-auto p-6 scrollbar-thin space-y-4">
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">Version</label>
            <p class="text-gray-900 dark:text-white">{{ packageInfo.version }}</p>
          </div>
          <div>
            <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">Repository</label>
            <p class="text-gray-900 dark:text-white">{{ packageInfo.repo }}</p>
          </div>
          <div>
            <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">Status</label>
            <p :class="getStatusClass()">
              {{ getStatusText() }}
            </p>
          </div>
          <div>
            <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">Size</label>
            <p class="text-gray-900 dark:text-white">{{ details.size || 'Unknown' }}</p>
          </div>
        </div>

        <div v-if="packageInfo.description" class="border-t border-gray-200 dark:border-gray-700 pt-4">
          <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">Description</label>
          <p class="mt-1 text-gray-900 dark:text-white">{{ packageInfo.description }}</p>
        </div>

        <div v-if="details.url" class="border-t border-gray-200 dark:border-gray-700 pt-4">
          <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">URL</label>
          <a :href="details.url" target="_blank" class="mt-1 block text-blue-600 dark:text-blue-400 hover:underline">
            {{ details.url }}
          </a>
        </div>

        <div v-if="details.licenses && details.licenses.length" class="border-t border-gray-200 dark:border-gray-700 pt-4">
          <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">Licenses</label>
          <p class="mt-1 text-gray-900 dark:text-white">{{ details.licenses.join(', ') }}</p>
        </div>

        <div v-if="details.dependencies && details.dependencies.length" class="border-t border-gray-200 dark:border-gray-700 pt-4">
          <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">Dependencies</label>
          <div class="mt-2 flex flex-wrap gap-2">
            <span v-for="dep in details.dependencies" :key="dep" class="px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 text-xs rounded">
              {{ dep }}
            </span>
          </div>
        </div>

        <div v-if="details.optionalDeps && details.optionalDeps.length" class="border-t border-gray-200 dark:border-gray-700 pt-4">
          <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">Optional Dependencies</label>
          <div class="mt-2 flex flex-wrap gap-2">
            <span v-for="dep in details.optionalDeps" :key="dep" class="px-2 py-1 bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-300 text-xs rounded">
              {{ dep }}
            </span>
          </div>
        </div>
      </div>

      <div class="p-6 border-t border-gray-200 dark:border-gray-700 flex justify-between gap-3">
        <button @click="$emit('close')" class="px-6 py-2 bg-gray-600 hover:bg-gray-700 text-white rounded-lg font-medium transition-colors">
          Close
        </button>
        <div class="flex gap-3">
          <button v-if="isUpdatePackage()" @click="$emit('install', packageInfo)" class="px-6 py-2 bg-orange-600 hover:bg-orange-700 text-white rounded-lg font-medium transition-colors">
            Update
          </button>
          <button v-else-if="!packageInfo.installed" @click="$emit('install', packageInfo)" class="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors">
            Install
          </button>
          <button v-else @click="$emit('remove', packageInfo)" class="px-6 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg font-medium transition-colors">
            Remove
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'PackageDetailsModal',
  props: {
    packageInfo: Object,
    details: {
      type: Object,
      default: () => ({})
    }
  },
  emits: ['close', 'install', 'remove'],
  methods: {
    isUpdatePackage() {
      return this.packageInfo.description && this.packageInfo.description.includes('→')
    },
    getStatusText() {
      if (this.isUpdatePackage()) {
        return 'Update Required'
      }
      return this.packageInfo.installed ? 'Installed' : 'Not Installed'
    },
    getStatusClass() {
      if (this.isUpdatePackage()) {
        return 'text-orange-600 dark:text-orange-400'
      }
      return this.packageInfo.installed ? 'text-green-600 dark:text-green-400' : 'text-gray-600 dark:text-gray-400'
    }
  }
}
</script>

