<template>
  <div
    class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6 hover:shadow-md transition-all duration-200 cursor-pointer group"
    :class="{ 'ring-2 ring-blue-500 dark:ring-blue-400': isSelected }"
    @click="handleCardClick"
    @contextmenu.prevent="handleRightClick"
  >
    <!-- Header with checkbox and package info -->
    <div class="flex items-start justify-between mb-4">
      <div class="flex items-start gap-3 flex-1 min-w-0">
        <!-- Repository Logo -->
        <div class="flex-shrink-0">
          <div class="w-12 h-12 rounded-lg flex items-center justify-center bg-gray-100 dark:bg-gray-700 border border-gray-200 dark:border-gray-600">
            <component :is="getRepoLogo(pkg.repo)" class="w-8 h-8" />
          </div>
        </div>

        <!-- Package Info -->
        <div class="flex-1 min-w-0">
          <div class="flex items-center gap-2 mb-1">
            <input
              type="checkbox"
              :checked="isSelected"
              @change="$emit('toggle-select', pkg)"
              @click.stop
              class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500"
            />
            <h3
              class="font-semibold text-gray-900 dark:text-white truncate group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors"
              @click.stop="$emit('show-details', pkg)"
            >
              {{ pkg.name }}
            </h3>
          </div>

          <div class="flex items-center gap-2 mb-2">
            <span
              class="px-2 py-1 text-xs font-medium rounded-full"
              :class="getRepoClass(pkg.repo)"
            >
              {{ pkg.repo }}
            </span>
            <span class="text-sm text-gray-500 dark:text-gray-400">{{ pkg.version }}</span>
          </div>

          <p
            v-if="pkg.description"
            class="text-sm text-gray-600 dark:text-gray-400 line-clamp-2 mb-2"
          >
            {{ pkg.description }}
          </p>

          <!-- Additional Details -->
          <div class="grid grid-cols-2 gap-2 text-xs text-gray-500 dark:text-gray-400">
            <div v-if="pkg.size || pkg.installed_size" class="flex items-center gap-1">
              <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 010 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 010 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 010 2H4a1 1 0 01-1-1z" clip-rule="evenodd" />
              </svg>
              {{ formatSize(pkg.size || pkg.installed_size) }}
            </div>
            <div v-if="pkg.depends_on && pkg.depends_on.length > 0" class="flex items-center gap-1">
              <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M3.293 3.293a1 1 0 011.414 0l6 6a1 1 0 01-1.414 1.414l-6-6a1 1 0 010-1.414z" clip-rule="evenodd" />
                <path fill-rule="evenodd" d="M14.95 6.364l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414z" clip-rule="evenodd" />
                <path fill-rule="evenodd" d="M15.657 10.657a1 1 0 00-1.414-1.414l-6 6a1 1 0 001.414 1.414l6-6z" clip-rule="evenodd" />
              </svg>
              {{ pkg.depends_on.length }} deps
            </div>
            <div v-if="pkg.groups && pkg.groups.length > 0" class="flex items-center gap-1">
              <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                <path d="M13 6a3 3 0 11-6 0 3 3 0 016 0zM18 8a2 2 0 11-4 0 2 2 0 014 0zM14 15a4 4 0 00-8 0v3h8v-3zM6 8a2 2 0 11-4 0 2 2 0 014 0zM16 18v-3a5.972 5.972 0 00-.75-2.906A3.005 3.005 0 0119 15v3h-3zM4.75 12.094A5.973 5.973 0 004 15v3H1v-3a3 3 0 013.75-2.906z" />
              </svg>
              {{ pkg.groups.length }} groups
            </div>
            <div v-if="pkg.arch" class="flex items-center gap-1">
              <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 010 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 010 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 010 2H4a1 1 0 01-1-1z" clip-rule="evenodd" />
              </svg>
              {{ pkg.arch }}
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Status and Actions -->
    <div class="flex items-center justify-between">
      <!-- Status -->
      <div class="flex items-center gap-2">
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
      </div>

      <!-- Actions -->
      <div class="flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
        <!-- Groups: Show "View" button -->
        <button
          v-if="pkg.repo === 'group'"
          @click.stop="$emit('show-details', pkg)"
          class="px-3 py-1 bg-teal-600 hover:bg-teal-700 text-white text-sm rounded-md transition-colors"
        >
          View
        </button>
        <button
          v-else-if="pkg.repo === 'group' && pkg.installed"
          @click.stop="$emit('install', pkg)"
          class="px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white text-sm rounded-md transition-colors"
        >
          Install
        </button>

        <!-- Files: Show "View Files" button -->
        <button
          v-else-if="pkg.repo === 'file' || pkg.repo === 'file-owner'"
          @click.stop="$emit('show-details', pkg)"
          class="px-3 py-1 bg-cyan-600 hover:bg-cyan-700 text-white text-sm rounded-md transition-colors"
        >
          View
        </button>

        <!-- Repository: Show "View Packages" button -->
        <button
          v-else-if="pkg.repo === 'repository'"
          @click.stop="$emit('show-details', pkg)"
          class="px-3 py-1 bg-violet-600 hover:bg-violet-700 text-white text-sm rounded-md transition-colors"
        >
          View
        </button>

        <!-- History: Show "View" button -->
        <button
          v-else-if="pkg.repo === 'history'"
          @click.stop="$emit('show-details', pkg)"
          class="px-3 py-1 bg-indigo-600 hover:bg-indigo-700 text-white text-sm rounded-md transition-colors"
        >
          View
        </button>

        <!-- Regular packages: Update/Install/Remove -->
        <template v-else>
          <button
            v-if="isUpdatePackage(pkg)"
            @click.stop="$emit('install', pkg)"
            class="px-3 py-1 bg-orange-600 hover:bg-orange-700 text-white text-sm rounded-md transition-colors"
          >
            Update
          </button>
          <button
            v-if="!pkg.installed && pkg.repo !== 'log' && !isUpdatePackage(pkg)"
            @click.stop="$emit('install', pkg)"
            class="px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white text-sm rounded-md transition-colors"
          >
            Install
          </button>
          <button
            v-if="pkg.installed && pkg.repo !== 'log' && !isUpdatePackage(pkg)"
            @click.stop="$emit('remove', pkg)"
            class="px-3 py-1 bg-red-600 hover:bg-red-700 text-white text-sm rounded-md transition-colors"
          >
            Remove
          </button>
        </template>
      </div>
    </div>

    <!-- Context Menu -->
    <ContextMenu
      :visible="contextMenuVisible"
      :position="contextMenuPosition"
      :package="pkg"
      :is-selected="isSelected"
      :can-install="canInstall"
      :can-remove="canRemove"
      :can-show-dependencies="canShowDependencies"
      @action="handleContextAction"
      @close="closeContextMenu"
    />
  </div>
</template>

<script>
import ContextMenu from './ContextMenu.vue'

// Repository Logo Components
const ArchLogo = {
  template: `
    <svg viewBox="0 0 24 24" class="text-blue-600 dark:text-blue-400">
      <path fill="currentColor" d="M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0zm0 2c5.514 0 10 4.486 10 10s-4.486 10-10 10S2 17.514 2 12 6.486 2 12 2zm-1.5 3v6h-3v2h3v6h3v-6h3v-2h-3v-6h-3z"/>
    </svg>
  `
}

const AURLogo = {
  template: `
    <svg viewBox="0 0 24 24" class="text-orange-600 dark:text-orange-400">
      <path fill="currentColor" d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
    </svg>
  `
}

const DefaultRepoLogo = {
  template: `
    <svg viewBox="0 0 24 24" class="text-gray-600 dark:text-gray-400">
      <path fill="currentColor" d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
    </svg>
  `
}

export default {
  name: 'PackageCard',
  components: {
    ContextMenu,
    ArchLogo,
    AURLogo,
    DefaultRepoLogo
  },
  props: {
    pkg: {
      type: Object,
      required: true
    },
    isSelected: {
      type: Boolean,
      default: false
    }
  },
  emits: ['toggle-select', 'install', 'remove', 'show-details', 'select-all', 'clear-selection'],
  data() {
    return {
      contextMenuVisible: false,
      contextMenuPosition: { x: 0, y: 0 }
    }
  },
  computed: {
    canInstall() {
      return !this.pkg.installed && this.pkg.repo !== 'log' && !this.isUpdatePackage(this.pkg)
    },
    canRemove() {
      return this.pkg.installed && this.pkg.repo !== 'log' && !this.isUpdatePackage(this.pkg)
    },
    canShowDependencies() {
      return this.pkg.repo !== 'group' && this.pkg.repo !== 'repository' && this.pkg.repo !== 'file' && this.pkg.repo !== 'file-owner'
    }
  },
  mounted() {
    // Add global click listener to close context menu
    document.addEventListener('click', this.closeContextMenu)
  },
  beforeUnmount() {
    document.removeEventListener('click', this.closeContextMenu)
  },
  methods: {
    handleCardClick() {
      // Allow card click to show details, but prevent if clicking on interactive elements
      this.$emit('show-details', this.pkg)
    },
    isUpdatePackage(pkg) {
      return pkg.description && pkg.description.includes('→')
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
    },
    getRepoLogo(repo) {
      if (repo === 'aur') {
        return 'AURLogo'
      } else if (['core', 'extra', 'community', 'multilib'].includes(repo)) {
        return 'ArchLogo'
      } else {
        return 'DefaultRepoLogo'
      }
    },
    formatSize(size) {
      if (!size || size === 'Unknown') return 'Unknown'

      // Handle string sizes like "1.2 MB" or "500 KB"
      if (typeof size === 'string') {
        return size
      }

      // Handle numeric sizes (bytes)
      const units = ['B', 'KB', 'MB', 'GB', 'TB']
      let unitIndex = 0
      let formattedSize = size

      while (formattedSize >= 1024 && unitIndex < units.length - 1) {
        formattedSize /= 1024
        unitIndex++
      }

      return `${formattedSize.toFixed(1)} ${units[unitIndex]}`
    },
    handleRightClick(event) {
      event.preventDefault()
      this.contextMenuPosition = {
        x: event.clientX,
        y: event.clientY
      }
      this.contextMenuVisible = true
    },
    closeContextMenu() {
      this.contextMenuVisible = false
    },
    handleContextAction(action) {
      switch (action) {
        case 'install':
          this.$emit('install', this.pkg)
          break
        case 'remove':
          this.$emit('remove', this.pkg)
          break
        case 'details':
          this.$emit('show-details', this.pkg)
          break
        case 'dependencies':
          // This will be handled by the parent component
          this.$emit('show-dependencies', this.pkg.name)
          break
        case 'select':
          this.$emit('toggle-select', this.pkg)
          break
        case 'select-all':
          this.$emit('select-all')
          break
        case 'clear-selection':
          this.$emit('clear-selection')
          break
      }
      this.closeContextMenu()
    }
  }
}
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
