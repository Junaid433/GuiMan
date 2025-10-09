<template>
  <div class="h-full flex flex-col bg-gray-50 dark:bg-gray-900">
    <div v-if="loading" class="flex-1 overflow-auto scrollbar-thin">
      <table class="w-full">
        <thead class="bg-gray-100 dark:bg-gray-800 sticky top-0 z-10">
          <tr>
            <th class="px-4 py-3 text-left">
              <div class="w-4 h-4 bg-gray-200 dark:bg-gray-700 rounded"></div>
            </th>
            <th class="px-4 py-3 text-left text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wider">Package Name</th>
            <th class="px-4 py-3 text-left text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wider">Version</th>
            <th class="px-4 py-3 text-left text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wider">Repository</th>
            <th class="px-4 py-3 text-left text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wider">Status</th>
            <th class="px-4 py-3 text-left text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wider">Actions</th>
          </tr>
        </thead>
        <tbody class="bg-white dark:bg-gray-900 divide-y divide-gray-200 dark:divide-gray-700">
          <tr v-for="i in 15" :key="i" class="animate-pulse">
            <td class="px-4 py-3">
              <div class="w-4 h-4 bg-gray-200 dark:bg-gray-700 rounded"></div>
            </td>
            <td class="px-4 py-3">
              <div class="space-y-1">
                <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-32"></div>
                <div class="h-3 bg-gray-200 dark:bg-gray-700 rounded w-48"></div>
              </div>
            </td>
            <td class="px-4 py-3">
              <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-20"></div>
            </td>
            <td class="px-4 py-3">
              <div class="h-6 bg-gray-200 dark:bg-gray-700 rounded w-16"></div>
            </td>
            <td class="px-4 py-3">
              <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-24"></div>
            </td>
            <td class="px-4 py-3">
              <div class="flex gap-2">
                <div class="h-6 bg-gray-200 dark:bg-gray-700 rounded w-16"></div>
                <div class="h-6 bg-gray-200 dark:bg-gray-700 rounded w-16"></div>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
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
              <span v-if="isUpdatePackage(pkg)" class="flex items-center gap-1 text-orange-600 dark:text-orange-400 text-sm">
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
                </svg>
                Update Required
              </span>
              <span v-else-if="pkg.installed" class="flex items-center gap-1 text-green-600 dark:text-green-400 text-sm">
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                </svg>
                Installed
              </span>
              <span v-else class="text-gray-500 dark:text-gray-400 text-sm">Not installed</span>
            </td>
            <td class="px-4 py-3">
              <div class="flex gap-2">
                <!-- Groups: Show "View" button -->
                <button
                  v-if="pkg.repo === 'group'"
                  @click="$emit('show-details', pkg)"
                  class="px-3 py-1 bg-teal-600 hover:bg-teal-700 text-white text-sm rounded-md transition-colors"
                >
                  View Group
                </button>
                <button
                  v-else-if="pkg.repo === 'group' && pkg.installed"
                  @click="$emit('install', pkg)"
                  class="px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white text-sm rounded-md transition-colors"
                >
                  Install Group
                </button>
                
                <!-- Files: Show "View Files" button -->
                <button
                  v-else-if="pkg.repo === 'file' || pkg.repo === 'file-owner'"
                  @click="$emit('show-details', pkg)"
                  class="px-3 py-1 bg-cyan-600 hover:bg-cyan-700 text-white text-sm rounded-md transition-colors"
                >
                  View Files
                </button>
                
                <!-- Repository: Show "View Packages" button -->
                <button
                  v-else-if="pkg.repo === 'repository'"
                  @click="$emit('show-details', pkg)"
                  class="px-3 py-1 bg-violet-600 hover:bg-violet-700 text-white text-sm rounded-md transition-colors"
                >
                  View Packages
                </button>
                
                <!-- History: Show "View" button -->
                <button
                  v-else-if="pkg.repo === 'history'"
                  @click="$emit('show-details', pkg)"
                  class="px-3 py-1 bg-indigo-600 hover:bg-indigo-700 text-white text-sm rounded-md transition-colors"
                >
                  View Details
                </button>
                
                <!-- Regular packages: Update/Install/Remove -->
                <template v-else>
                  <button
                    v-if="isUpdatePackage(pkg)"
                    @click="$emit('install', pkg)"
                    class="px-3 py-1 bg-orange-600 hover:bg-orange-700 text-white text-sm rounded-md transition-colors"
                  >
                    Update
                  </button>
                  <button
                    v-if="!pkg.installed && pkg.repo !== 'log' && !isUpdatePackage(pkg)"
                    @click="$emit('install', pkg)"
                    class="px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white text-sm rounded-md transition-colors"
                  >
                    Install
                  </button>
                  <button
                    v-if="pkg.installed && pkg.repo !== 'log' && !isUpdatePackage(pkg)"
                    @click="$emit('remove', pkg)"
                    class="px-3 py-1 bg-red-600 hover:bg-red-700 text-white text-sm rounded-md transition-colors"
                  >
                    Remove
                  </button>
                </template>
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
    showDescriptions: Boolean,
    activeView: String
  },
  emits: ['toggle-select', 'install', 'remove', 'show-details', 'search-example'],
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
    },
    isUpdatePackage(pkg) {
      return pkg.description && pkg.description.includes('→')
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
        'history': 'bg-indigo-100 text-indigo-800 dark:bg-indigo-900 dark:text-indigo-200',
        'aur': 'bg-pink-100 text-pink-800 dark:bg-pink-900 dark:text-pink-200',
        'group': 'bg-teal-100 text-teal-800 dark:bg-teal-900 dark:text-teal-200',
        'file': 'bg-cyan-100 text-cyan-800 dark:bg-cyan-900 dark:text-cyan-200',
        'file-owner': 'bg-cyan-100 text-cyan-800 dark:bg-cyan-900 dark:text-cyan-200',
        'repository': 'bg-violet-100 text-violet-800 dark:bg-violet-900 dark:text-violet-200',
        'log': 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200'
      }
      return classes[repo] || 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200'
    }
  }
}
</script>

