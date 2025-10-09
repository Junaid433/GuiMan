<template>
  <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
    <div class="flex items-center justify-between mb-6">
      <h2 class="text-xl font-bold text-gray-900 dark:text-white">System Analytics</h2>
      <div class="flex gap-2">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          @click="activeTab = tab.id"
          :class="[
            'px-4 py-2 rounded-lg text-sm font-medium transition-colors',
            activeTab === tab.id
              ? 'bg-blue-600 text-white'
              : 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700'
          ]"
        >
          {{ tab.label }}
        </button>
      </div>
    </div>

    <div class="space-y-6">
      <!-- System Health Chart -->
      <div v-if="activeTab === 'health'" class="space-y-4">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white">System Health Metrics</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <div class="bg-gray-50 dark:bg-gray-700 p-4 rounded-lg">
            <div class="flex items-center justify-between mb-2">
              <span class="text-sm font-medium text-gray-600 dark:text-gray-400">Total Packages</span>
              <svg class="w-5 h-5 text-blue-500" fill="currentColor" viewBox="0 0 20 20">
                <path d="M3 4a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zM3 10a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6zM14 9a1 1 0 00-1 1v6a1 1 0 001 1h2a1 1 0 001-1v-6a1 1 0 00-1-1h-2z" />
              </svg>
            </div>
            <div class="text-2xl font-bold text-gray-900 dark:text-white">{{ systemStats.totalPackages }}</div>
          </div>

          <div class="bg-gray-50 dark:bg-gray-700 p-4 rounded-lg">
            <div class="flex items-center justify-between mb-2">
              <span class="text-sm font-medium text-gray-600 dark:text-gray-400">Updates Available</span>
              <svg class="w-5 h-5 text-orange-500" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd" />
              </svg>
            </div>
            <div class="text-2xl font-bold text-gray-900 dark:text-white">{{ systemStats.updatesAvailable }}</div>
          </div>

          <div class="bg-gray-50 dark:bg-gray-700 p-4 rounded-lg">
            <div class="flex items-center justify-between mb-2">
              <span class="text-sm font-medium text-gray-600 dark:text-gray-400">Orphaned Packages</span>
              <svg class="w-5 h-5 text-red-500" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
              </svg>
            </div>
            <div class="text-2xl font-bold text-gray-900 dark:text-white">{{ systemStats.orphans }}</div>
          </div>

          <div class="bg-gray-50 dark:bg-gray-700 p-4 rounded-lg">
            <div class="flex items-center justify-between mb-2">
              <span class="text-sm font-medium text-gray-600 dark:text-gray-400">AUR Packages</span>
              <svg class="w-5 h-5 text-purple-500" fill="currentColor" viewBox="0 0 20 20">
                <path d="M10.894 2.553a1 1 0 00-1.788 0l-7 14a1 1 0 001.169 1.409l5-1.429A1 1 0 009 15.571V11a1 1 0 112 0v4.571a1 1 0 00.725.962l5 1.428a1 1 0 001.17-1.408l-7-14z" />
              </svg>
            </div>
            <div class="text-2xl font-bold text-gray-900 dark:text-white">{{ systemStats.aurPackages }}</div>
          </div>
        </div>

        <!-- Health Score -->
        <div class="mt-6">
          <h4 class="text-md font-semibold text-gray-900 dark:text-white mb-4">System Health Score</h4>
          <div class="bg-gray-50 dark:bg-gray-700 p-4 rounded-lg">
            <div class="flex items-center justify-between mb-2">
              <span class="text-sm font-medium text-gray-600 dark:text-gray-400">Overall Health</span>
              <span class="text-sm font-bold" :class="healthScoreClass">{{ healthScore }}%</span>
            </div>
            <div class="w-full bg-gray-200 dark:bg-gray-600 rounded-full h-3">
              <div
                class="h-3 rounded-full transition-all duration-500"
                :class="healthScoreBarClass"
                :style="{ width: healthScore + '%' }"
              ></div>
            </div>
            <div class="flex justify-between text-xs text-gray-500 dark:text-gray-400 mt-2">
              <span>Critical</span>
              <span>Poor</span>
              <span>Fair</span>
              <span>Good</span>
              <span>Excellent</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Update Trends Chart -->
      <div v-if="activeTab === 'trends'" class="space-y-4">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Update Trends</h3>
        <div class="bg-gray-50 dark:bg-gray-700 p-4 rounded-lg">
          <div class="text-center text-gray-600 dark:text-gray-400 py-8">
            <svg class="w-16 h-16 mx-auto mb-4 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd" />
              <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zM3 10a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6zM14 9a1 1 0 00-1 1v6a1 1 0 001 1h2a1 1 0 001-1v-6a1 1 0 00-1-1h-2z" clip-rule="evenodd" />
            </svg>
            <p class="text-lg font-medium">Update Trends Coming Soon</p>
            <p class="text-sm">This feature will show update patterns over time</p>
          </div>
        </div>
      </div>

      <!-- Package Sizes Chart -->
      <div v-if="activeTab === 'sizes'" class="space-y-4">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Package Sizes Distribution</h3>
        <div class="bg-gray-50 dark:bg-gray-700 p-4 rounded-lg">
          <div class="text-center text-gray-600 dark:text-gray-400 py-8">
            <svg class="w-16 h-16 mx-auto mb-4 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
            <p class="text-lg font-medium">Package Sizes Coming Soon</p>
            <p class="text-sm">This feature will show package size distributions</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import PieChart from './PieChart.vue'

export default {
  name: 'DataVisualization',
  components: {
    PieChart
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
    packages: {
      type: Array,
      default: () => []
    }
  },
  data() {
    return {
      activeTab: 'health',
      tabs: [
        { id: 'health', label: 'Health' },
        { id: 'trends', label: 'Trends' },
        { id: 'sizes', label: 'Sizes' }
      ]
    }
  },
  computed: {
    repoData() {
      const repoCounts = {}
      this.packages.forEach(pkg => {
        const repo = pkg.repo || 'unknown'
        repoCounts[repo] = (repoCounts[repo] || 0) + 1
      })

      const colors = {
        'core': '#8B5CF6',
        'extra': '#3B82F6',
        'community': '#10B981',
        'multilib': '#F59E0B',
        'aur': '#EC4899',
        'orphan': '#EF4444',
        'unknown': '#6B7280'
      }

      return Object.entries(repoCounts)
        .map(([name, value]) => ({
          name,
          value,
          color: colors[name] || colors.unknown
        }))
        .sort((a, b) => b.value - a.value)
    },
    healthScore() {
      const total = this.systemStats.totalPackages
      if (total === 0) return 0

      const updatesRatio = this.systemStats.updatesAvailable / total
      const orphansRatio = this.systemStats.orphans / total
      const aurRatio = this.systemStats.aurPackages / total

      // Calculate health score (higher is better)
      // Updates available reduce score, orphans reduce score, AUR packages are neutral
      let score = 100
      score -= updatesRatio * 30 // Max 30 points deduction for updates
      score -= orphansRatio * 40 // Max 40 points deduction for orphans
      score = Math.max(0, Math.min(100, score))

      return Math.round(score)
    },
    healthScoreClass() {
      if (this.healthScore >= 80) return 'text-green-600 dark:text-green-400'
      if (this.healthScore >= 60) return 'text-yellow-600 dark:text-yellow-400'
      if (this.healthScore >= 40) return 'text-orange-600 dark:text-orange-400'
      return 'text-red-600 dark:text-red-400'
    },
    healthScoreBarClass() {
      if (this.healthScore >= 80) return 'bg-green-500'
      if (this.healthScore >= 60) return 'bg-yellow-500'
      if (this.healthScore >= 40) return 'bg-orange-500'
      return 'bg-red-500'
    }
  }
}
</script>
