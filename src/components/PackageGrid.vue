<template>
  <div class="h-full flex flex-col bg-gray-50 dark:bg-gray-900">
    <div v-if="loading" class="flex-1 p-6">
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
        <SkeletonLoader
          v-for="i in 12"
          :key="i"
          type="card"
        />
      </div>
    </div>

    <div v-else-if="packages.length === 0" class="flex-1 flex items-center justify-center">
      <div class="text-center max-w-2xl px-4">
        <svg class="w-16 h-16 text-gray-400 mx-auto mb-4" fill="currentColor" viewBox="0 0 20 20">
          <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
        </svg>
        <p class="text-gray-600 dark:text-gray-400 text-lg font-medium mb-2">{{ getEmptyMessage() }}</p>
        <p class="text-gray-500 dark:text-gray-500 text-sm mb-4">{{ getEmptyHint() }}</p>

        <!-- Examples for Files view -->
        <div v-if="activeView === 'files'" class="mt-6 space-y-3">
          <p class="text-xs font-semibold text-gray-600 dark:text-gray-400 uppercase tracking-wider mb-3">Try these examples:</p>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
            <button
              v-for="example in fileExamples"
              :key="example.query"
              @click="$emit('search-example', example.query)"
              class="px-4 py-3 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors text-left"
            >
              <div class="text-sm font-medium text-gray-900 dark:text-white font-mono">{{ example.query }}</div>
              <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">{{ example.description }}</div>
            </button>
          </div>
        </div>
      </div>
    </div>

    <div v-else class="flex-1 overflow-auto p-6">
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
        <PackageCard
          v-for="pkg in packages"
          :key="pkg.name"
          :pkg="pkg"
          :is-selected="isSelected(pkg)"
          @toggle-select="$emit('toggle-select', pkg)"
          @install="$emit('install', pkg)"
          @remove="$emit('remove', pkg)"
          @show-details="$emit('show-details', pkg)"
          @select-all="$emit('select-all')"
          @clear-selection="$emit('clear-selection')"
          @show-dependencies="$emit('show-dependencies', $event)"
        />
      </div>
    </div>
  </div>
</template>

<script>
import PackageCard from './PackageCard.vue'
import SkeletonLoader from './SkeletonLoader.vue'

export default {
  name: 'PackageGrid',
  components: {
    PackageCard,
    SkeletonLoader
  },
  props: {
    packages: Array,
    loading: Boolean,
    selectedPackages: Array,
    activeView: String
  },
  emits: ['toggle-select', 'install', 'remove', 'show-details', 'search-example', 'select-all', 'clear-selection', 'show-dependencies'],
  data() {
    return {
      fileExamples: [
        { query: '/usr/bin/bash', description: 'Find which package owns bash' },
        { query: '/usr/bin/python', description: 'Find which package owns python' },
        { query: '/usr/lib/libgtk', description: 'Find GTK library files' },
        { query: 'vim', description: 'Search for vim-related files' },
        { query: 'systemd', description: 'Search for systemd files' },
        { query: '/etc/pacman.conf', description: 'Find pacman config owner' }
      ]
    }
  },
  methods: {
    getEmptyMessage() {
      switch (this.activeView) {
        case 'files':
          return 'Search for files or packages'
        case 'groups':
          return 'No package groups found'
        case 'repos':
          return 'No repositories found'
        case 'updates':
          return 'No updates available'
        case 'orphans':
          return 'No orphaned packages'
        case 'history':
          return 'No package history'
        case 'aur':
          return 'No AUR packages'
        default:
          return 'No packages found'
      }
    },
    getEmptyHint() {
      switch (this.activeView) {
        case 'files':
          return 'Enter a file path (e.g., /usr/bin/bash) or search for a filename pattern'
        case 'groups':
          return 'Package groups allow you to install multiple related packages at once'
        case 'repos':
          return 'Repositories contain collections of packages for your system'
        case 'updates':
          return 'Your system is up to date! Run "Update System" to check again'
        case 'orphans':
          return 'Orphaned packages are no longer needed. Great job keeping your system clean!'
        case 'history':
          return 'Package installation history will appear here'
        case 'aur':
          return 'Enable AUR support in settings to see AUR packages'
        default:
          return 'Try searching for packages or select a different view'
      }
    },
    isSelected(pkg) {
      return this.selectedPackages.some(p => p.name === pkg.name)
    }
  }
}
</script>
