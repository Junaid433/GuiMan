<template>
  <div class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-4 py-3">
    <div class="flex gap-3 items-center flex-wrap">
      <div class="flex items-center gap-2">
        <svg class="w-4 h-4 text-gray-500 dark:text-gray-400" fill="currentColor" viewBox="0 0 20 20">
          <path fill-rule="evenodd" d="M3 3a1 1 0 011-1h12a1 1 0 011 1v3a1 1 0 01-.293.707L12 11.414V15a1 1 0 01-.293.707l-2 2A1 1 0 018 17v-5.586L3.293 6.707A1 1 0 013 6V3z" clip-rule="evenodd" />
        </svg>
        <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Filters:</span>
      </div>

      <select 
        v-model="localFilters.repository" 
        @change="emitFilters"
        class="px-3 py-1.5 text-sm bg-gray-50 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:outline-none"
      >
        <option value="all" class="bg-white dark:bg-gray-800 text-gray-900 dark:text-white">All Repositories</option>
        <option value="core" class="bg-white dark:bg-gray-800 text-gray-900 dark:text-white">Core</option>
        <option value="extra" class="bg-white dark:bg-gray-800 text-gray-900 dark:text-white">Extra</option>
        <option value="community" class="bg-white dark:bg-gray-800 text-gray-900 dark:text-white">Community</option>
        <option value="multilib" class="bg-white dark:bg-gray-800 text-gray-900 dark:text-white">Multilib</option>
        <option value="aur" v-if="showAur" class="bg-white dark:bg-gray-800 text-gray-900 dark:text-white">AUR</option>
      </select>

      <select 
        v-model="localFilters.status" 
        @change="emitFilters"
        class="px-3 py-1.5 text-sm bg-gray-50 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:outline-none"
      >
        <option value="all" class="bg-white dark:bg-gray-800 text-gray-900 dark:text-white">All Status</option>
        <option value="installed" class="bg-white dark:bg-gray-800 text-gray-900 dark:text-white">Installed</option>
        <option value="not-installed" class="bg-white dark:bg-gray-800 text-gray-900 dark:text-white">Not Installed</option>
        <option value="update-required" class="bg-white dark:bg-gray-800 text-gray-900 dark:text-white">Update Required</option>
      </select>

      <input 
        v-model="localFilters.search" 
        @input="emitFilters"
        type="text"
        placeholder="Filter by name..."
        class="px-3 py-1.5 text-sm bg-gray-50 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-900 dark:text-white placeholder-gray-500 dark:placeholder-gray-400 focus:ring-2 focus:ring-blue-500 focus:outline-none w-48"
      />

      <button 
        @click="clearFilters"
        class="px-3 py-1.5 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors"
      >
        Clear
      </button>

      <div class="ml-auto text-sm text-gray-500 dark:text-gray-400">
        {{ filteredCount }} {{ filteredCount === 1 ? 'package' : 'packages' }}
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'FilterBar',
  props: {
    filters: {
      type: Object,
      default: () => ({
        repository: 'all',
        status: 'all',
        search: ''
      })
    },
    filteredCount: {
      type: Number,
      default: 0
    },
    showAur: {
      type: Boolean,
      default: false
    }
  },
  data() {
    return {
      localFilters: { ...this.filters }
    }
  },
  watch: {
    filters: {
      handler(newFilters) {
        this.localFilters = { ...newFilters }
      },
      deep: true
    }
  },
  methods: {
    emitFilters() {
      this.$emit('update:filters', { ...this.localFilters })
    },
    clearFilters() {
      this.localFilters = {
        repository: 'all',
        status: 'all',
        search: ''
      }
      this.emitFilters()
    }
  }
}
</script>

