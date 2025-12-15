<template>
  <div class="h-full overflow-auto bg-gray-50 dark:bg-gray-900">
    <!-- Welcome Header -->
    <div class="bg-gradient-to-r from-blue-600 to-purple-600 dark:from-blue-800 dark:to-purple-800 text-white">
      <div class="max-w-7xl mx-auto px-6 py-8">
        <h1 class="text-3xl font-bold mb-2">Welcome to GuiMan</h1>
        <p class="text-blue-100 text-lg">Your Arch Linux package manager</p>
      </div>
    </div>

    <div class="max-w-7xl mx-auto px-6 py-8 space-y-8">
      <!-- System Overview Cards -->
      <div v-if="loading" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <SkeletonLoader
          v-for="i in 4"
          :key="i"
          type="dashboard-card"
        />
      </div>
      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-gray-600 dark:text-gray-400">Total Packages</p>
              <p class="text-3xl font-bold text-gray-900 dark:text-white">{{ systemStats.totalPackages }}</p>
            </div>
            <div class="p-3 bg-blue-100 dark:bg-blue-900 rounded-lg">
              <svg class="w-6 h-6 text-blue-600 dark:text-blue-400" fill="currentColor" viewBox="0 0 20 20">
                <path d="M3 4a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zM3 10a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6zM14 9a1 1 0 00-1 1v6a1 1 0 001 1h2a1 1 0 001-1v-6a1 1 0 00-1-1h-2z" />
              </svg>
            </div>
          </div>
        </div>

        <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-gray-600 dark:text-gray-400">Updates Available</p>
              <p class="text-3xl font-bold text-gray-900 dark:text-white">{{ systemStats.updatesAvailable }}</p>
            </div>
            <div class="p-3 bg-orange-100 dark:bg-orange-900 rounded-lg">
              <svg class="w-6 h-6 text-orange-600 dark:text-orange-400" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd" />
              </svg>
            </div>
          </div>
        </div>

        <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-gray-600 dark:text-gray-400">Orphaned Packages</p>
              <p class="text-3xl font-bold text-gray-900 dark:text-white">{{ systemStats.orphans }}</p>
            </div>
            <div class="p-3 bg-red-100 dark:bg-red-900 rounded-lg">
              <svg class="w-6 h-6 text-red-600 dark:text-red-400" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
              </svg>
            </div>
          </div>
        </div>

        <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-gray-600 dark:text-gray-400">AUR Packages</p>
              <p class="text-3xl font-bold text-gray-900 dark:text-white">{{ systemStats.aurPackages }}</p>
            </div>
            <div class="p-3 bg-purple-100 dark:bg-purple-900 rounded-lg">
              <svg class="w-6 h-6 text-purple-600 dark:text-purple-400" fill="currentColor" viewBox="0 0 20 20">
                <path d="M10.894 2.553a1 1 0 00-1.788 0l-7 14a1 1 0 001.169 1.409l5-1.429A1 1 0 009 15.571V11a1 1 0 112 0v4.571a1 1 0 00.725.962l5 1.428a1 1 0 001.17-1.408l-7-14z" />
              </svg>
            </div>
          </div>
        </div>
      </div>

      <!-- Quick Actions -->
      <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-4">Quick Actions</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <button
            @click="$emit('update-system')"
            class="flex items-center gap-3 p-4 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg hover:bg-green-100 dark:hover:bg-green-900/30 transition-colors group"
          >
            <div class="p-2 bg-green-100 dark:bg-green-900 rounded-lg group-hover:bg-green-200 dark:group-hover:bg-green-800">
              <svg class="w-5 h-5 text-green-600 dark:text-green-400" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd" />
              </svg>
            </div>
            <div class="text-left">
              <p class="font-medium text-gray-900 dark:text-white">Update System</p>
              <p class="text-sm text-gray-600 dark:text-gray-400">Update all packages</p>
            </div>
          </button>

          <button
            @click="$emit('change-view', 'updates')"
            class="flex items-center gap-3 p-4 bg-orange-50 dark:bg-orange-900/20 border border-orange-200 dark:border-orange-800 rounded-lg hover:bg-orange-100 dark:hover:bg-orange-900/30 transition-colors group"
          >
            <div class="p-2 bg-orange-100 dark:bg-orange-900 rounded-lg group-hover:bg-orange-200 dark:group-hover:bg-orange-800">
              <svg class="w-5 h-5 text-orange-600 dark:text-orange-400" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
              </svg>
            </div>
            <div class="text-left">
              <p class="font-medium text-gray-900 dark:text-white">Check Updates</p>
              <p class="text-sm text-gray-600 dark:text-gray-400">View available updates</p>
            </div>
          </button>

          <button
            @click="$emit('change-view', 'orphans')"
            class="flex items-center gap-3 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg hover:bg-red-100 dark:hover:bg-red-900/30 transition-colors group"
          >
            <div class="p-2 bg-red-100 dark:bg-red-900 rounded-lg group-hover:bg-red-200 dark:group-hover:bg-red-800">
              <svg class="w-5 h-5 text-red-600 dark:text-red-400" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M9 2a1 1 0 000 2h2a1 1 0 100-2H9z" clip-rule="evenodd" />
                <path fill-rule="evenodd" d="M10 5a1 1 0 011 1v3h3a1 1 0 110 2h-3v3a1 1 0 11-2 0v-3H6a1 1 0 110-2h3V6a1 1 0 011-1z" clip-rule="evenodd" />
              </svg>
            </div>
            <div class="text-left">
              <p class="font-medium text-gray-900 dark:text-white">Clean Orphans</p>
              <p class="text-sm text-gray-600 dark:text-gray-400">Remove unused packages</p>
            </div>
          </button>

          <button
            @click="$emit('change-view', 'search')"
            class="flex items-center gap-3 p-4 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg hover:bg-blue-100 dark:hover:bg-blue-900/30 transition-colors group"
          >
            <div class="p-2 bg-blue-100 dark:bg-blue-900 rounded-lg group-hover:bg-blue-200 dark:group-hover:bg-blue-800">
              <svg class="w-5 h-5 text-blue-600 dark:text-blue-400" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd" />
              </svg>
            </div>
            <div class="text-left">
              <p class="font-medium text-gray-900 dark:text-white">Search Packages</p>
              <p class="text-sm text-gray-600 dark:text-gray-400">Find and install packages</p>
            </div>
          </button>
        </div>
      </div>

      <!-- Recent Updates -->
      <div v-if="displayedUpdates.length > 0" class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-4">Recent Updates</h2>
        <div class="space-y-3">
          <div
            v-for="update in displayedUpdates"
            :key="update.name"
            class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg"
          >
            <div class="flex items-center gap-3">
              <div class="w-2 h-2 bg-green-500 rounded-full"></div>
              <div>
                <p class="font-medium text-gray-900 dark:text-white">{{ update.name }}</p>
                <p class="text-sm text-gray-600 dark:text-gray-400">{{ update.oldVersion }} → {{ update.newVersion }}</p>
              </div>
            </div>
            <span class="text-xs text-gray-500 dark:text-gray-400">{{ formatDate(update.date) }}</span>
          </div>
        </div>
      </div>

      <!-- System Analytics -->
      <SkeletonLoader
        v-if="loading"
        type="analytics"
      />
      <DataVisualization
        v-else
        :system-stats="systemStats"
        :packages="packages"
      />

      <!-- Popular Packages -->
      <div v-if="displayedPopular.length > 0" class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
        <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-4">Popular Packages</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <div
            v-for="pkg in displayedPopular"
            :key="pkg.name"
            class="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-700 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-600 transition-colors cursor-pointer"
            @click="$emit('show-details', pkg)"
          >
            <div class="w-10 h-10 bg-gradient-to-br from-blue-500 to-purple-600 rounded-lg flex items-center justify-center text-white font-bold text-sm">
              {{ pkg.name.charAt(0).toUpperCase() }}
            </div>
            <div class="flex-1 min-w-0">
              <p class="font-medium text-gray-900 dark:text-white truncate">{{ pkg.name }}</p>
              <p class="text-sm text-gray-600 dark:text-gray-400 truncate">{{ pkg.description }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { defineAsyncComponent, computed } from 'vue'
import SkeletonLoader from './SkeletonLoader.vue'

// Lazy load heavy visualization component
const DataVisualization = defineAsyncComponent(() => 
  import('./DataVisualization.vue')
)

// Memoize date formatting
const dateCache = new Map()
const formatDateCached = (dateString) => {
  if (!dateString) return ''
  if (dateCache.has(dateString)) return dateCache.get(dateString)
  const formatted = new Date(dateString).toLocaleDateString()
  dateCache.set(dateString, formatted)
  return formatted
}

export default {
  name: 'Dashboard',
  components: {
    DataVisualization,
    SkeletonLoader
  },
  props: {
    systemStats: {
      type: Object,
      default: () => ({
        totalPackages: 0,
        updatesAvailable: 0,
        orphans: 0,
        aurPackages: 0
      })
    },
    recentUpdates: {
      type: Array,
      default: () => []
    },
    popularPackages: {
      type: Array,
      default: () => []
    },
    packages: {
      type: Array,
      default: () => []
    },
    loading: {
      type: Boolean,
      default: false
    }
  },
  emits: ['update-system', 'change-view', 'show-details'],
  setup(props) {
    // Pre-slice arrays to avoid re-computing in template
    const displayedUpdates = computed(() => props.recentUpdates.slice(0, 5))
    const displayedPopular = computed(() => props.popularPackages.slice(0, 6))
    
    return {
      displayedUpdates,
      displayedPopular,
      formatDate: formatDateCached
    }
  }
}
</script>
