<template>
  <div class="h-full flex flex-col bg-gray-50 dark:bg-gray-900">
    <div v-if="loading" class="flex-1 flex items-center justify-center">
      <div class="text-center">
        <svg class="animate-spin h-12 w-12 text-blue-600 mx-auto mb-4" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        <p class="text-gray-600 dark:text-gray-400">Loading packages...</p>
      </div>
    </div>

    <div v-else-if="packages.length === 0" class="flex-1 flex items-center justify-center">
      <div class="text-center">
        <svg class="w-16 h-16 text-gray-400 mx-auto mb-4" fill="currentColor" viewBox="0 0 20 20">
          <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
        </svg>
        <p class="text-gray-600 dark:text-gray-400 text-lg">No packages found</p>
      </div>
    </div>

    <div v-else class="flex-1 overflow-auto scrollbar-thin">
      <table class="w-full">
        <thead class="bg-gray-100 dark:bg-gray-800 sticky top-0 z-10">
          <tr>
            <th class="px-4 py-3 text-left">
              <input
                type="checkbox"
                @change="toggleAll"
                class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500"
              />
            </th>
            <th class="px-4 py-3 text-left text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wider">Package Name</th>
            <th class="px-4 py-3 text-left text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wider">Version</th>
            <th class="px-4 py-3 text-left text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wider">Repository</th>
            <th class="px-4 py-3 text-left text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wider">Status</th>
            <th class="px-4 py-3 text-left text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wider">Actions</th>
          </tr>
        </thead>
        <tbody class="bg-white dark:bg-gray-900 divide-y divide-gray-200 dark:divide-gray-700">
          <tr
            v-for="pkg in packages"
            :key="pkg.name"
            :class="[
              'hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors',
              isSelected(pkg) ? 'bg-blue-50 dark:bg-blue-900/20' : ''
            ]"
          >
            <td class="px-4 py-3">
              <input
                type="checkbox"
                :checked="isSelected(pkg)"
                @change="$emit('toggle-select', pkg)"
                class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500"
              />
            </td>
            <td class="px-4 py-3">
              <div class="flex items-center gap-2">
                <span :class="[
                  'font-medium text-gray-900 dark:text-white cursor-pointer hover:text-blue-600 dark:hover:text-blue-400',
                  compactView ? 'text-sm' : ''
                ]" @click="$emit('show-details', pkg)">
                  {{ pkg.name }}
                </span>
              </div>
              <div v-if="showDescriptions && pkg.description" :class="[
                'text-gray-500 dark:text-gray-400 mt-1',
                compactView ? 'text-xs' : 'text-sm'
              ]">
                {{ pkg.description }}
              </div>
            </td>
            <td class="px-4 py-3 text-sm text-gray-700 dark:text-gray-300">{{ pkg.version }}</td>
            <td class="px-4 py-3">
              <span :class="[
                'px-2 py-1 text-xs font-medium rounded-full',
                getRepoClass(pkg.repo)
              ]">
                {{ pkg.repo }}
              </span>
            </td>
            <td class="px-4 py-3">
              <span v-if="pkg.installed" class="flex items-center gap-1 text-green-600 dark:text-green-400 text-sm">
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                </svg>
                Installed
              </span>
              <span v-else class="text-gray-500 dark:text-gray-400 text-sm">Not installed</span>
            </td>
            <td class="px-4 py-3">
              <div class="flex gap-2">
                <button
                  v-if="!pkg.installed && pkg.repo !== 'log'"
                  @click="$emit('install', pkg)"
                  class="px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white text-sm rounded-md transition-colors"
                >
                  Install
                </button>
                <button
                  v-if="pkg.installed && pkg.repo !== 'log'"
                  @click="$emit('remove', pkg)"
                  class="px-3 py-1 bg-red-600 hover:bg-red-700 text-white text-sm rounded-md transition-colors"
                >
                  Remove
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script>
export default {
  name: 'PackageTable',
  props: {
    packages: Array,
    loading: Boolean,
    selectedPackages: Array,
    compactView: Boolean,
    showDescriptions: Boolean
  },
  emits: ['toggle-select', 'install', 'remove', 'show-details'],
  methods: {
    isSelected(pkg) {
      return this.selectedPackages.some(p => p.name === pkg.name)
    },
    toggleAll(event) {
      if (event.target.checked) {
        this.packages.forEach(pkg => {
          if (!this.isSelected(pkg)) {
            this.$emit('toggle-select', pkg)
          }
        })
      } else {
        this.packages.forEach(pkg => {
          if (this.isSelected(pkg)) {
            this.$emit('toggle-select', pkg)
          }
        })
      }
    },
    getRepoClass(repo) {
      const classes = {
        'core': 'bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200',
        'extra': 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200',
        'community': 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200',
        'multilib': 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200',
        'updates': 'bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-200',
        'orphan': 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200',
        'log': 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200'
      }
      return classes[repo] || 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200'
    }
  }
}
</script>

