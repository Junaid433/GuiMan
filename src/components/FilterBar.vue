<template>
  <div class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-4 py-3">
    <div class="flex items-center gap-4 flex-wrap">
      <!-- Quick Filters -->
      <div class="flex items-center gap-2">
        <span class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Filter:</span>
        <button
          v-for="filter in quickFilters"
          :key="filter.id"
          @click="toggleQuickFilter(filter.id)"
          :class="[
            'px-3 py-1.5 text-xs font-medium rounded-full transition-all',
            activeFilters.includes(filter.id)
              ? filter.activeClass
              : 'bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-600'
          ]"
        >
          {{ filter.label }}
          <span v-if="filter.count !== undefined" class="ml-1 opacity-75">({{ filter.count }})</span>
        </button>
      </div>

      <!-- Repository Filter -->
      <div class="flex items-center gap-2">
        <span class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Repo:</span>
        <select
          v-model="selectedRepo"
          @change="emitFilters"
          class="px-3 py-1.5 text-xs bg-gray-100 dark:bg-gray-700 border-0 rounded-lg text-gray-700 dark:text-gray-300 focus:ring-2 focus:ring-blue-500"
        >
          <option value="">All Repositories</option>
          <option v-for="repo in repositories" :key="repo" :value="repo">{{ repo }}</option>
        </select>
      </div>

      <!-- Sort Options -->
      <div class="flex items-center gap-2 ml-auto">
        <span class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase">Sort:</span>
        <select
          v-model="sortBy"
          @change="emitFilters"
          class="px-3 py-1.5 text-xs bg-gray-100 dark:bg-gray-700 border-0 rounded-lg text-gray-700 dark:text-gray-300 focus:ring-2 focus:ring-blue-500"
        >
          <option value="name">Name (A-Z)</option>
          <option value="name-desc">Name (Z-A)</option>
          <option value="size">Size (Largest)</option>
          <option value="size-asc">Size (Smallest)</option>
          <option value="date">Recently Updated</option>
          <option value="repo">Repository</option>
        </select>

        <!-- Sort Direction Toggle -->
        <button
          @click="toggleSortDirection"
          class="p-1.5 rounded-lg bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-600"
          :title="sortDirection === 'asc' ? 'Ascending' : 'Descending'"
        >
          <svg v-if="sortDirection === 'asc'" class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M3.293 9.707a1 1 0 010-1.414l6-6a1 1 0 011.414 0l6 6a1 1 0 01-1.414 1.414L11 5.414V17a1 1 0 11-2 0V5.414L4.707 9.707a1 1 0 01-1.414 0z" clip-rule="evenodd" />
          </svg>
          <svg v-else class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M16.707 10.293a1 1 0 010 1.414l-6 6a1 1 0 01-1.414 0l-6-6a1 1 0 111.414-1.414L9 14.586V3a1 1 0 012 0v11.586l4.293-4.293a1 1 0 011.414 0z" clip-rule="evenodd" />
          </svg>
        </button>
      </div>

      <!-- Results Count -->
      <div class="text-xs text-gray-500 dark:text-gray-400">
        <span class="font-semibold text-gray-700 dark:text-gray-300">{{ filteredCount }}</span> of {{ totalCount }} packages
      </div>

      <!-- Clear All Filters -->
      <button
        v-if="hasActiveFilters"
        @click="clearAllFilters"
        class="px-3 py-1.5 text-xs font-medium text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg transition-colors"
      >
        Clear Filters
      </button>
    </div>

    <!-- Active Filter Tags -->
    <div v-if="activeFilterTags.length > 0" class="flex items-center gap-2 mt-3 flex-wrap">
      <span class="text-xs text-gray-500 dark:text-gray-400">Active:</span>
      <span
        v-for="tag in activeFilterTags"
        :key="tag.id"
        class="inline-flex items-center gap-1 px-2 py-1 text-xs bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-300 rounded-full"
      >
        {{ tag.label }}
        <button @click="removeFilter(tag.id)" class="hover:text-blue-900 dark:hover:text-blue-100">
          <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </button>
      </span>
    </div>
  </div>
</template>

<script>
export default {
  name: 'FilterBar',
  props: {
    totalCount: {
      type: Number,
      default: 0
    },
    filteredCount: {
      type: Number,
      default: 0
    },
    packages: {
      type: Array,
      default: () => []
    },
    activeView: {
      type: String,
      default: ''
    }
  },
  emits: ['filter-change'],
  data() {
    return {
      activeFilters: [],
      selectedRepo: '',
      sortBy: 'name',
      sortDirection: 'asc'
    }
  },
  computed: {
    quickFilters() {
      const installedCount = this.packages.filter(p => p.installed).length
      const notInstalledCount = this.packages.filter(p => !p.installed).length
      const aurCount = this.packages.filter(p => p.repo === 'aur').length
      const hasUpdates = this.packages.filter(p => p.description?.includes('→')).length

      return [
        { 
          id: 'installed', 
          label: 'Installed', 
          count: installedCount,
          activeClass: 'bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-300'
        },
        { 
          id: 'not-installed', 
          label: 'Not Installed', 
          count: notInstalledCount,
          activeClass: 'bg-gray-200 dark:bg-gray-600 text-gray-800 dark:text-gray-200'
        },
        { 
          id: 'aur', 
          label: 'AUR', 
          count: aurCount,
          activeClass: 'bg-pink-100 dark:bg-pink-900 text-pink-700 dark:text-pink-300'
        },
        { 
          id: 'updates', 
          label: 'Has Updates', 
          count: hasUpdates,
          activeClass: 'bg-orange-100 dark:bg-orange-900 text-orange-700 dark:text-orange-300'
        }
      ]
    },
    repositories() {
      const repos = new Set(this.packages.map(p => p.repo).filter(r => r && r !== 'aur'))
      return Array.from(repos).sort()
    },
    hasActiveFilters() {
      return this.activeFilters.length > 0 || this.selectedRepo !== ''
    },
    activeFilterTags() {
      const tags = []
      
      this.activeFilters.forEach(filterId => {
        const filter = this.quickFilters.find(f => f.id === filterId)
        if (filter) {
          tags.push({ id: filterId, label: filter.label })
        }
      })
      
      if (this.selectedRepo) {
        tags.push({ id: 'repo', label: `Repo: ${this.selectedRepo}` })
      }
      
      return tags
    }
  },
  methods: {
    toggleQuickFilter(filterId) {
      const index = this.activeFilters.indexOf(filterId)
      if (index > -1) {
        this.activeFilters.splice(index, 1)
      } else {
        // Handle mutually exclusive filters
        if (filterId === 'installed') {
          this.activeFilters = this.activeFilters.filter(f => f !== 'not-installed')
        } else if (filterId === 'not-installed') {
          this.activeFilters = this.activeFilters.filter(f => f !== 'installed')
        }
        this.activeFilters.push(filterId)
      }
      this.emitFilters()
    },
    removeFilter(filterId) {
      if (filterId === 'repo') {
        this.selectedRepo = ''
      } else {
        this.activeFilters = this.activeFilters.filter(f => f !== filterId)
      }
      this.emitFilters()
    },
    clearAllFilters() {
      this.activeFilters = []
      this.selectedRepo = ''
      this.sortBy = 'name'
      this.sortDirection = 'asc'
      this.emitFilters()
    },
    toggleSortDirection() {
      this.sortDirection = this.sortDirection === 'asc' ? 'desc' : 'asc'
      this.emitFilters()
    },
    emitFilters() {
      this.$emit('filter-change', {
        quickFilters: this.activeFilters,
        repo: this.selectedRepo,
        sortBy: this.sortBy,
        sortDirection: this.sortDirection
      })
    }
  }
}
</script>
