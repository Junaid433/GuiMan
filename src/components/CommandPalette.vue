<template>
  <Teleport to="body">
    <Transition name="palette">
      <div
        v-if="isOpen"
        class="fixed inset-0 z-50 flex items-start justify-center pt-[15vh]"
        @click.self="close"
        @keydown.escape="close"
      >
        <!-- Backdrop -->
        <div class="absolute inset-0 bg-black/50 backdrop-blur-sm"></div>

        <!-- Palette -->
        <div class="relative w-full max-w-2xl mx-4 bg-white dark:bg-gray-800 rounded-xl shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden">
          <!-- Search Input -->
          <div class="flex items-center gap-3 px-4 py-4 border-b border-gray-200 dark:border-gray-700">
            <svg class="w-5 h-5 text-gray-400 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd" />
            </svg>
            <input
              ref="searchInput"
              v-model="query"
              type="text"
              :placeholder="getPlaceholder()"
              class="flex-1 bg-transparent text-gray-900 dark:text-white placeholder-gray-400 focus:outline-none text-lg"
              @keydown.down.prevent="moveSelection(1)"
              @keydown.up.prevent="moveSelection(-1)"
              @keydown.enter="executeSelected"
              @keydown.tab.prevent="moveSelection(1)"
            />
            <kbd class="hidden sm:inline-flex items-center px-2 py-1 text-xs font-medium text-gray-400 bg-gray-100 dark:bg-gray-700 rounded">
              ESC
            </kbd>
          </div>

          <!-- Category Tabs -->
          <div class="flex gap-1 px-4 py-2 border-b border-gray-100 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50">
            <button
              v-for="cat in categories"
              :key="cat.id"
              @click="activeCategory = cat.id"
              :class="[
                'px-3 py-1.5 text-xs font-medium rounded-lg transition-colors',
                activeCategory === cat.id
                  ? 'bg-blue-600 text-white'
                  : 'text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-700'
              ]"
            >
              {{ cat.label }}
            </button>
          </div>

          <!-- Results -->
          <div class="max-h-96 overflow-auto">
            <div v-if="filteredCommands.length === 0" class="px-4 py-8 text-center text-gray-500 dark:text-gray-400">
              <svg class="w-12 h-12 mx-auto mb-3 text-gray-300 dark:text-gray-600" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd" />
              </svg>
              <p class="text-sm">No commands found</p>
              <p class="text-xs mt-1">Try a different search term</p>
            </div>

            <div v-else class="py-2">
              <div
                v-for="(command, index) in filteredCommands"
                :key="command.id"
                @click="executeCommand(command)"
                @mouseenter="selectedIndex = index"
                :class="[
                  'flex items-center gap-3 px-4 py-3 cursor-pointer transition-colors',
                  selectedIndex === index
                    ? 'bg-blue-50 dark:bg-blue-900/30'
                    : 'hover:bg-gray-50 dark:hover:bg-gray-700/50'
                ]"
              >
                <!-- Icon -->
                <div :class="[
                  'flex-shrink-0 w-10 h-10 rounded-lg flex items-center justify-center',
                  command.iconBg || 'bg-gray-100 dark:bg-gray-700'
                ]">
                  <span v-html="command.icon" class="w-5 h-5"></span>
                </div>

                <!-- Content -->
                <div class="flex-1 min-w-0">
                  <div class="flex items-center gap-2">
                    <span class="font-medium text-gray-900 dark:text-white">{{ command.label }}</span>
                    <span v-if="command.badge" :class="[
                      'px-1.5 py-0.5 text-xs rounded',
                      command.badgeClass || 'bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400'
                    ]">
                      {{ command.badge }}
                    </span>
                  </div>
                  <p class="text-sm text-gray-500 dark:text-gray-400 truncate">{{ command.description }}</p>
                </div>

                <!-- Shortcut -->
                <div v-if="command.shortcut" class="flex-shrink-0 hidden sm:flex items-center gap-1">
                  <kbd
                    v-for="(key, keyIndex) in command.shortcut.split('+')"
                    :key="keyIndex"
                    class="px-2 py-1 text-xs font-medium text-gray-500 dark:text-gray-400 bg-gray-100 dark:bg-gray-700 rounded"
                  >
                    {{ key }}
                  </kbd>
                </div>
              </div>
            </div>
          </div>

          <!-- Footer -->
          <div class="flex items-center justify-between px-4 py-2 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50 text-xs text-gray-500 dark:text-gray-400">
            <div class="flex items-center gap-4">
              <span class="flex items-center gap-1">
                <kbd class="px-1.5 py-0.5 bg-gray-200 dark:bg-gray-700 rounded">↑↓</kbd>
                Navigate
              </span>
              <span class="flex items-center gap-1">
                <kbd class="px-1.5 py-0.5 bg-gray-200 dark:bg-gray-700 rounded">↵</kbd>
                Select
              </span>
            </div>
            <span>{{ filteredCommands.length }} commands</span>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script>
export default {
  name: 'CommandPalette',
  props: {
    isOpen: {
      type: Boolean,
      default: false
    },
    systemStats: {
      type: Object,
      default: () => ({})
    }
  },
  emits: ['close', 'command'],
  data() {
    return {
      query: '',
      selectedIndex: 0,
      activeCategory: 'all'
    }
  },
  computed: {
    categories() {
      return [
        { id: 'all', label: 'All' },
        { id: 'navigation', label: 'Navigation' },
        { id: 'packages', label: 'Packages' },
        { id: 'system', label: 'System' },
        { id: 'tools', label: 'Tools' }
      ]
    },
    allCommands() {
      return [
        // Navigation
        {
          id: 'nav-dashboard',
          category: 'navigation',
          label: 'Go to Dashboard',
          description: 'View system overview and stats',
          icon: '<svg fill="currentColor" viewBox="0 0 20 20"><path d="M3 4a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zM3 10a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6zM14 9a1 1 0 00-1 1v6a1 1 0 001 1h2a1 1 0 001-1v-6a1 1 0 00-1-1h-2z" /></svg>',
          iconBg: 'bg-blue-100 dark:bg-blue-900 text-blue-600 dark:text-blue-400',
          action: () => this.$emit('command', { type: 'navigate', view: 'dashboard' })
        },
        {
          id: 'nav-installed',
          category: 'navigation',
          label: 'View Installed Packages',
          description: 'Browse all installed packages',
          icon: '<svg fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" /></svg>',
          iconBg: 'bg-green-100 dark:bg-green-900 text-green-600 dark:text-green-400',
          action: () => this.$emit('command', { type: 'navigate', view: 'installed' })
        },
        {
          id: 'nav-updates',
          category: 'navigation',
          label: 'Check Updates',
          description: `${this.systemStats.updatesAvailable || 0} updates available`,
          badge: this.systemStats.updatesAvailable > 0 ? this.systemStats.updatesAvailable : null,
          badgeClass: 'bg-orange-100 dark:bg-orange-900 text-orange-600 dark:text-orange-400',
          icon: '<svg fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1z" clip-rule="evenodd" /></svg>',
          iconBg: 'bg-orange-100 dark:bg-orange-900 text-orange-600 dark:text-orange-400',
          action: () => this.$emit('command', { type: 'navigate', view: 'updates' })
        },
        {
          id: 'nav-orphans',
          category: 'navigation',
          label: 'View Orphan Packages',
          description: `${this.systemStats.orphans || 0} orphaned packages`,
          badge: this.systemStats.orphans > 0 ? this.systemStats.orphans : null,
          badgeClass: 'bg-red-100 dark:bg-red-900 text-red-600 dark:text-red-400',
          icon: '<svg fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92z" clip-rule="evenodd" /></svg>',
          iconBg: 'bg-red-100 dark:bg-red-900 text-red-600 dark:text-red-400',
          action: () => this.$emit('command', { type: 'navigate', view: 'orphans' })
        },
        {
          id: 'nav-aur',
          category: 'navigation',
          label: 'Browse AUR Packages',
          description: 'View Arch User Repository packages',
          icon: '<svg fill="currentColor" viewBox="0 0 20 20"><path d="M10.894 2.553a1 1 0 00-1.788 0l-7 14a1 1 0 001.169 1.409l5-1.429A1 1 0 009 15.571V11a1 1 0 112 0v4.571a1 1 0 00.725.962l5 1.428a1 1 0 001.17-1.408l-7-14z" /></svg>',
          iconBg: 'bg-purple-100 dark:bg-purple-900 text-purple-600 dark:text-purple-400',
          action: () => this.$emit('command', { type: 'navigate', view: 'aur' })
        },
        {
          id: 'nav-history',
          category: 'navigation',
          label: 'View Package History',
          description: 'See recent package changes',
          icon: '<svg fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd" /></svg>',
          iconBg: 'bg-indigo-100 dark:bg-indigo-900 text-indigo-600 dark:text-indigo-400',
          action: () => this.$emit('command', { type: 'navigate', view: 'history' })
        },

        // System Actions
        {
          id: 'sys-update',
          category: 'system',
          label: 'Update System',
          description: 'Update all packages to latest versions',
          shortcut: 'Ctrl+U',
          icon: '<svg fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1z" clip-rule="evenodd" /></svg>',
          iconBg: 'bg-green-100 dark:bg-green-900 text-green-600 dark:text-green-400',
          action: () => this.$emit('command', { type: 'action', action: 'update-system' })
        },
        {
          id: 'sys-sync',
          category: 'system',
          label: 'Sync Databases',
          description: 'Synchronize package databases',
          icon: '<svg fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1z" clip-rule="evenodd" /></svg>',
          iconBg: 'bg-cyan-100 dark:bg-cyan-900 text-cyan-600 dark:text-cyan-400',
          action: () => this.$emit('command', { type: 'action', action: 'sync-databases' })
        },
        {
          id: 'sys-clean-cache',
          category: 'system',
          label: 'Clean Package Cache',
          description: 'Remove cached package files',
          icon: '<svg fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd" /></svg>',
          iconBg: 'bg-orange-100 dark:bg-orange-900 text-orange-600 dark:text-orange-400',
          action: () => this.$emit('command', { type: 'action', action: 'clean-cache' })
        },
        {
          id: 'sys-remove-orphans',
          category: 'system',
          label: 'Remove All Orphans',
          description: 'Remove all orphaned packages at once',
          icon: '<svg fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9z" clip-rule="evenodd" /></svg>',
          iconBg: 'bg-red-100 dark:bg-red-900 text-red-600 dark:text-red-400',
          action: () => this.$emit('command', { type: 'action', action: 'remove-orphans' })
        },

        // Tools
        {
          id: 'tool-search',
          category: 'tools',
          label: 'Search Packages',
          description: 'Search for packages by name',
          shortcut: 'Ctrl+F',
          icon: '<svg fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd" /></svg>',
          iconBg: 'bg-blue-100 dark:bg-blue-900 text-blue-600 dark:text-blue-400',
          action: () => this.$emit('command', { type: 'action', action: 'focus-search' })
        },
        {
          id: 'tool-backup',
          category: 'tools',
          label: 'Backup & Restore',
          description: 'Create and manage system backups',
          icon: '<svg fill="currentColor" viewBox="0 0 20 20"><path d="M4 4a2 2 0 00-2 2v8a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2h-5L9 4H4zm7 5a1 1 0 10-2 0v1H8a1 1 0 100 2h1v1a1 1 0 102 0v-1h1a1 1 0 100-2h-1V9z" /></svg>',
          iconBg: 'bg-teal-100 dark:bg-teal-900 text-teal-600 dark:text-teal-400',
          action: () => this.$emit('command', { type: 'navigate', view: 'backup' })
        },
        {
          id: 'tool-settings',
          category: 'tools',
          label: 'Open Settings',
          description: 'Configure GuiMan preferences',
          icon: '<svg fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd" /></svg>',
          iconBg: 'bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-400',
          action: () => this.$emit('command', { type: 'action', action: 'open-settings' })
        },
        {
          id: 'tool-toggle-theme',
          category: 'tools',
          label: 'Toggle Dark Mode',
          description: 'Switch between light and dark theme',
          icon: '<svg fill="currentColor" viewBox="0 0 20 20"><path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z" /></svg>',
          iconBg: 'bg-yellow-100 dark:bg-yellow-900 text-yellow-600 dark:text-yellow-400',
          action: () => this.$emit('command', { type: 'action', action: 'toggle-theme' })
        },
        {
          id: 'tool-toggle-view',
          category: 'tools',
          label: 'Toggle View Mode',
          description: 'Switch between table and card view',
          shortcut: 'Ctrl+T',
          icon: '<svg fill="currentColor" viewBox="0 0 20 20"><path d="M5 3a2 2 0 00-2 2v2a2 2 0 002 2h2a2 2 0 002-2V5a2 2 0 00-2-2H5zM5 11a2 2 0 00-2 2v2a2 2 0 002 2h2a2 2 0 002-2v-2a2 2 0 00-2-2H5zM11 5a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V5zM11 13a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" /></svg>',
          iconBg: 'bg-violet-100 dark:bg-violet-900 text-violet-600 dark:text-violet-400',
          action: () => this.$emit('command', { type: 'action', action: 'toggle-view' })
        },
        {
          id: 'tool-refresh',
          category: 'tools',
          label: 'Refresh View',
          description: 'Reload current view data',
          shortcut: 'Ctrl+R',
          icon: '<svg fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1z" clip-rule="evenodd" /></svg>',
          iconBg: 'bg-blue-100 dark:bg-blue-900 text-blue-600 dark:text-blue-400',
          action: () => this.$emit('command', { type: 'action', action: 'refresh' })
        },

        // Package Actions
        {
          id: 'pkg-install',
          category: 'packages',
          label: 'Install Package...',
          description: 'Search and install a new package',
          icon: '<svg fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" /></svg>',
          iconBg: 'bg-green-100 dark:bg-green-900 text-green-600 dark:text-green-400',
          action: () => this.$emit('command', { type: 'action', action: 'install-package' })
        },
        {
          id: 'pkg-export',
          category: 'packages',
          label: 'Export Package List',
          description: 'Export installed packages to file',
          icon: '<svg fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd" /></svg>',
          iconBg: 'bg-blue-100 dark:bg-blue-900 text-blue-600 dark:text-blue-400',
          action: () => this.$emit('command', { type: 'action', action: 'export-packages' })
        }
      ]
    },
    filteredCommands() {
      let commands = this.allCommands

      // Filter by category
      if (this.activeCategory !== 'all') {
        commands = commands.filter(cmd => cmd.category === this.activeCategory)
      }

      // Filter by query
      if (this.query.trim()) {
        const query = this.query.toLowerCase()
        commands = commands.filter(cmd =>
          cmd.label.toLowerCase().includes(query) ||
          cmd.description.toLowerCase().includes(query)
        )
      }

      return commands
    }
  },
  watch: {
    isOpen(newVal) {
      if (newVal) {
        this.query = ''
        this.selectedIndex = 0
        this.activeCategory = 'all'
        this.$nextTick(() => {
          this.$refs.searchInput?.focus()
        })
      }
    },
    filteredCommands() {
      this.selectedIndex = 0
    }
  },
  methods: {
    close() {
      this.$emit('close')
    },
    getPlaceholder() {
      return 'Type a command or search...'
    },
    moveSelection(delta) {
      const newIndex = this.selectedIndex + delta
      if (newIndex >= 0 && newIndex < this.filteredCommands.length) {
        this.selectedIndex = newIndex
      }
    },
    executeSelected() {
      if (this.filteredCommands.length > 0) {
        this.executeCommand(this.filteredCommands[this.selectedIndex])
      }
    },
    executeCommand(command) {
      if (command.action) {
        command.action()
      }
      this.close()
    }
  }
}
</script>

<style scoped>
.palette-enter-active,
.palette-leave-active {
  transition: all 0.2s ease;
}

.palette-enter-from,
.palette-leave-to {
  opacity: 0;
}

.palette-enter-from > div:last-child,
.palette-leave-to > div:last-child {
  transform: scale(0.95) translateY(-10px);
}
</style>
