<template>
  <div :class="{ 'dark': darkMode }" class="h-screen w-screen">
    <div class="flex h-full bg-gray-50 dark:bg-gray-900">
      <Sidebar
        :active-view="activeView"
        @change-view="handleViewChange"
        @toggle-theme="toggleTheme"
        @toggle-sidebar="toggleSidebar"
        @open-settings="showSettingsModal = true"
        :dark-mode="darkMode"
        :aur-enabled="config.aurSupport"
        :collapsed="config.sidebarCollapsed"
      />
      
      <div class="flex-1 flex flex-col overflow-hidden">
        <div class="relative">
          <SearchBar
            ref="searchBar"
            v-model="searchQuery"
            :active-view="activeView"
            :view-mode="config.viewMode"
            :unread-count="unreadNotificationCount"
            :has-active-notifications="hasActiveNotifications"
            @search="handleSearch"
            @update-system="handleUpdateSystem"
            @toggle-view-mode="toggleViewMode"
            @show-keyboard-shortcuts="showKeyboardShortcutsModal = true"
            @toggle-notifications="toggleNotificationCenter"
          />

          <!-- Notification System positioned relative to SearchBar -->
          <NotificationSystem
            ref="notificationSystem"
            :config="config"
            :show-center="showNotificationCenter"
            @action="handleNotificationAction"
            @close="showNotificationCenter = false"
            class="absolute top-full mt-1"
            style="right: 120px;"
          />
        </div>
        
        <div class="flex-1 overflow-hidden">
          <Dashboard
            v-if="activeView === 'dashboard'"
            :system-stats="systemStats"
            :recent-updates="recentUpdates"
            :popular-packages="popularPackages"
            :packages="packages"
            :loading="loading"
            @update-system="handleUpdateSystem"
            @change-view="handleViewChange"
            @show-details="showPackageDetails"
          />
          <PackageTable
            v-else-if="config.viewMode === 'table' && activeView !== 'dashboard'"
            :packages="displayPackages"
            :loading="loading"
            :selected-packages="selectedPackages"
            :active-view="activeView"
            :compact-view="config.compactView"
            :show-descriptions="config.showDescriptions"
            @toggle-select="toggleSelect"
            @install="handleInstall"
            @remove="handleRemove"
            @show-details="showPackageDetails"
            @search-example="handleSearchExample"
          />
          <PackageGrid
            v-else-if="activeView !== 'dashboard'"
            :packages="displayPackages"
            :loading="loading"
            :selected-packages="selectedPackages"
            :active-view="activeView"
            @toggle-select="toggleSelect"
            @install="handleInstall"
            @remove="handleRemove"
            @show-details="showPackageDetails"
            @search-example="handleSearchExample"
            @select-all="handleSelectAll"
            @clear-selection="handleClearSelection"
            @show-dependencies="handleShowDependencies"
          />
        </div>
        
        <StatusBar 
          :selected-count="selectedPackages.length"
          @install-selected="handleInstallSelected"
          @remove-selected="handleRemoveSelected"
        />
      </div>
      
      <LogModal 
        v-if="showLogModal"
        :logs="logs"
        :operation="currentOperation"
        :completed="operationCompleted"
        :success="operationSuccess"
        @close="closeLogModal"
      />

      <SettingsModal
        v-if="showSettingsModal"
        ref="settingsModal"
        :config="config"
        @close="showSettingsModal = false"
        @save="saveSettings"
        @clean-cache="handleCleanCache"
      />

      <ConfirmDialog
        v-if="showConfirmDialog"
        :title="confirmDialog.title"
        :message="confirmDialog.message"
        :confirm-text="confirmDialog.confirmText"
        :type="confirmDialog.type"
        @confirm="confirmDialog.onConfirm"
        @cancel="showConfirmDialog = false"
      />

    <PackageDetailsModal 
      v-if="showDetailsModal"
      :package-info="selectedPackageForDetails"
      :details="packageDetails"
      @close="showDetailsModal = false"
      @install="handleInstall"
      @remove="handleRemove"
      @show-dependencies="showDependencyGraph"
    />
    
    <DependencyGraph
      v-if="showDependencyGraphModal"
      :package-name="dependencyGraphPackage"
      @close="showDependencyGraphModal = false"
      @package-click="handleDependencyPackageClick"
    />
    
    <BackupModal
      v-if="showBackupModal"
      @close="showBackupModal = false"
    />

    <!-- Keyboard Shortcuts Modal -->
    <div v-if="showKeyboardShortcutsModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" @click="showKeyboardShortcutsModal = false">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full mx-4" @click.stop>
        <div class="p-6">
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Keyboard Shortcuts</h3>
            <button @click="showKeyboardShortcutsModal = false" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300">
              <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
              </svg>
            </button>
          </div>
          <div class="space-y-3">
            <div class="flex justify-between items-center py-2 border-b border-gray-200 dark:border-gray-700">
              <span class="text-gray-700 dark:text-gray-300">Focus Search</span>
              <kbd class="px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded text-sm">Ctrl+F</kbd>
            </div>
            <div class="flex justify-between items-center py-2 border-b border-gray-200 dark:border-gray-700">
              <span class="text-gray-700 dark:text-gray-300">Toggle View Mode</span>
              <kbd class="px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded text-sm">Ctrl+T</kbd>
            </div>
            <div class="flex justify-between items-center py-2 border-b border-gray-200 dark:border-gray-700">
              <span class="text-gray-700 dark:text-gray-300">Toggle Sidebar</span>
              <kbd class="px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded text-sm">Ctrl+B</kbd>
            </div>
            <div class="flex justify-between items-center py-2 border-b border-gray-200 dark:border-gray-700">
              <span class="text-gray-700 dark:text-gray-300">Update System</span>
              <kbd class="px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded text-sm">Ctrl+U</kbd>
            </div>
            <div class="flex justify-between items-center py-2 border-b border-gray-200 dark:border-gray-700">
              <span class="text-gray-700 dark:text-gray-300">Refresh View</span>
              <kbd class="px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded text-sm">Ctrl+R</kbd>
            </div>
            <div class="flex justify-between items-center py-2 border-b border-gray-200 dark:border-gray-700">
              <span class="text-gray-700 dark:text-gray-300">Clear Search / Close Modals</span>
              <kbd class="px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded text-sm">Escape</kbd>
            </div>
            <div class="flex justify-between items-center py-2">
              <span class="text-gray-700 dark:text-gray-300">Show Shortcuts</span>
              <kbd class="px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded text-sm">F1</kbd>
            </div>
          </div>
        </div>
      </div>
    </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted, computed, onUnmounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { sendNotification, isPermissionGranted, requestPermission } from '@tauri-apps/plugin-notification'
import Sidebar from './components/Sidebar.vue'
import SearchBar from './components/SearchBar.vue'
import PackageTable from './components/PackageTable.vue'
import StatusBar from './components/StatusBar.vue'
import LogModal from './components/LogModal.vue'
import SettingsModal from './components/SettingsModal.vue'
import ConfirmDialog from './components/ConfirmDialog.vue'
import PackageDetailsModal from './components/PackageDetailsModal.vue'
import DependencyGraph from './components/DependencyGraph.vue'
import BackupModal from './components/BackupModal.vue'
import Dashboard from './components/Dashboard.vue'
import PackageGrid from './components/PackageGrid.vue'
import NotificationSystem from './components/NotificationSystem.vue'
import configManager from './utils/config.js'
const appWindow = getCurrentWebviewWindow()

export default {
  name: 'App',
  components: {
    Sidebar,
    SearchBar,
    PackageTable,
    PackageGrid,
    StatusBar,
    LogModal,
    SettingsModal,
    ConfirmDialog,
    PackageDetailsModal,
    DependencyGraph,
    BackupModal,
    Dashboard,
    NotificationSystem
  },
  setup() {
    const activeView = ref('dashboard')
    const searchQuery = ref('')
    const packages = ref([])
    const loading = ref(false)
    const selectedPackages = ref([])
    const showLogModal = ref(false)
    const logs = ref([])
    const currentOperation = ref('')
    const operationCompleted = ref(false)
    const operationSuccess = ref(false)
    const config = ref(configManager.config)
    const darkMode = ref(config.value.theme === 'dark')
    const showSettingsModal = ref(false)
    const settingsModal = ref(null)
    const cacheJustCleaned = ref(false)
    const showConfirmDialog = ref(false)
    const confirmDialog = ref({})
    const showDetailsModal = ref(false)
    const selectedPackageForDetails = ref(null)
    const packageDetails = ref({})
    const showDependencyGraphModal = ref(false)
    const dependencyGraphPackage = ref('')
    const showBackupModal = ref(false)
    const showKeyboardShortcutsModal = ref(false)
    const showNotificationCenter = ref(false)
    const notificationSystem = ref(null)
    const systemStats = ref({
      totalPackages: 0,
      updatesAvailable: 0,
      orphans: 0,
      aurPackages: 0
    })
    const recentUpdates = ref([])
    const popularPackages = ref([])
    let refreshInterval = null

    const displayPackages = computed(() => {
      return packages.value
    })

    const unreadNotificationCount = computed(() => {
      if (!notificationSystem.value) return 0
      return notificationSystem.value.notifications.filter(n => !n.read).length
    })

    const hasActiveNotifications = computed(() => {
      if (!notificationSystem.value) return false
      return notificationSystem.value.activeNotifications.length > 0
    })

    const toggleTheme = () => {
      darkMode.value = !darkMode.value
      config.value.theme = darkMode.value ? 'dark' : 'light'
      configManager.save(config.value)
    }

    const toggleViewMode = () => {
      config.value.viewMode = config.value.viewMode === 'table' ? 'cards' : 'table'
      configManager.save(config.value)
    }

    const toggleSidebar = () => {
      config.value.sidebarCollapsed = !config.value.sidebarCollapsed
      configManager.save(config.value)
    }

    const saveSettings = (newConfig) => {
      if (newConfig === null) {
        configManager.reset()
        config.value = configManager.config
      } else {
        configManager.save(newConfig)
        config.value = newConfig
      }
      darkMode.value = config.value.theme === 'dark'
      setupAutoRefresh()
      showSettingsModal.value = false
    }

    const setupAutoRefresh = () => {
      if (refreshInterval) {
        clearInterval(refreshInterval)
        refreshInterval = null
      }
      
      if (config.value.autoRefresh) {
        refreshInterval = setInterval(() => {
          handleViewChange(activeView.value)
        }, config.value.refreshInterval * 1000)
      }
    }

    const showConfirm = (title, message, onConfirm, type = 'warning', confirmText = 'Confirm') => {
      confirmDialog.value = { title, message, onConfirm: () => {
        showConfirmDialog.value = false
        onConfirm()
      }, type, confirmText }
      showConfirmDialog.value = true
    }


    const showPackageDetails = async (pkg) => {
      selectedPackageForDetails.value = pkg
      packageDetails.value = {}
      
      showDetailsModal.value = true
      
      // Handle special views
      if (pkg.repo === 'group') {
        try {
          const groupPackages = await invoke('get_group_packages', { group: pkg.name })
          packageDetails.value = {
            size: `${pkg.groupData.installed_count} of ${pkg.groupData.total_count} packages installed`,
            url: '',
            licenses: [],
            dependencies: groupPackages.map(p => p.name),
            optionalDeps: []
          }
        } catch (error) {
          console.error('Failed to get group info:', error)
        }
        return
      }
      
      if (pkg.repo === 'repository') {
        packageDetails.value = {
          size: `${pkg.repoData.package_count} packages`,
          url: pkg.repoData.servers.join(', '),
          licenses: [],
          dependencies: [],
          optionalDeps: []
        }
        return
      }
      
      if (pkg.repo === 'file' || pkg.repo === 'file-owner') {
        try {
          const files = await invoke('list_package_files', { package: pkg.name })
          packageDetails.value = {
            size: `${files.length} files`,
            url: '',
            licenses: [],
            dependencies: files.slice(0, 50).map(f => f.path),
            optionalDeps: []
          }
        } catch (error) {
          console.error('Failed to get file info:', error)
        }
        return
      }
      
      // Regular package info
      try {
        const info = await invoke('get_package_info', { 
          pkg: pkg.name,
          repo: pkg.repo || 'unknown',
          is_installed: pkg.installed || false
        })
        let size = info.installed_size || info.download_size || 'Unknown'
        
        if (size === 'Unknown' && pkg.repo === 'aur' && !pkg.installed) {
          size = 'Unknown (AUR packages are built from source)'
        }
        
        packageDetails.value = {
          size: size,
          url: info.url || '',
          licenses: info.licenses ? info.licenses.split(' ').filter(l => l.trim()) : [],
          dependencies: info.depends_on ? info.depends_on.split(' ').filter(d => d.trim()) : [],
          optionalDeps: info.optional_deps ? info.optional_deps.split(' ').filter(d => d.trim()) : []
        }
      } catch (error) {
        console.error('Failed to get package info:', error)
      }
    }

    const handleCleanCache = async () => {
      showSettingsModal.value = false
      
      // Always show confirmation dialog for cache cleaning
      showConfirm(
        'Clean Package Cache',
        'Are you sure you want to clean the package cache?\n\nThis will remove all cached package files to free up disk space. This action cannot be undone.',
        async () => {
          currentOperation.value = 'Cleaning package cache (pacman -Scc + AUR helper -Scc)'
          logs.value = []
          operationCompleted.value = false
          operationSuccess.value = false
          showLogModal.value = true

          try {
            await invoke('clean_cache', { 
              aurHelper: config.value.aurHelper || 'yay' 
            })
          } catch (error) {
            logs.value.push(`Error: ${error}`)
            operationCompleted.value = true
            operationSuccess.value = false
          }
        },
        'warning',
        'Clean Cache'
      )
    }


    const handleViewChange = async (view) => {
      activeView.value = view
      selectedPackages.value = []
      
      if (view === 'backup') {
        showBackupModal.value = true
        return
      }
      
      loading.value = true

      try {
        switch (view) {
          case 'dashboard':
            // Load dashboard data without loading all installed packages
            try {
              const [orphans, updates, popular, history, installed] = await Promise.all([
                invoke('list_orphans'),
                invoke('check_updates'),
                invoke('get_popular_packages'),
                invoke('get_package_history'),
                invoke('list_installed')
              ])

              systemStats.value = {
                totalPackages: installed.length,
                updatesAvailable: updates.length,
                orphans: orphans.length,
                aurPackages: installed.filter(p => p.repo === 'aur').length
              }

              popularPackages.value = popular.slice(0, 6)
              recentUpdates.value = history.slice(0, 5).map(h => ({
                name: h.name,
                oldVersion: h.old_version || 'Unknown',
                newVersion: h.new_version || h.version,
                date: h.date || new Date().toISOString()
              }))

              // Store packages for analytics but don't display them
              packages.value = installed
            } catch (error) {
              console.error('Failed to load dashboard data:', error)
            }
            break
          case 'installed':
            packages.value = await invoke('list_installed')
            break
          case 'popular':
            packages.value = await invoke('get_popular_packages')
            break
          case 'aur':
            packages.value = await invoke('list_aur_packages', { helper: config.value.aurHelper })
            break
          case 'updates':
            packages.value = await invoke('check_updates')
            break
          case 'orphans':
            const orphans = await invoke('list_orphans')
            packages.value = orphans.map(name => ({
              name,
              version: '',
              repo: 'orphan',
              description: 'Orphaned package',
              installed: true
            }))
            break
          case 'history':
            packages.value = await invoke('get_package_history')
            break
          case 'groups':
            // For groups, we'll show a special view
            try {
              const groups = await invoke('list_groups')
              console.log('Groups received:', groups)
              packages.value = groups.map(g => ({
                name: g.name,
                version: `${g.installed_count}/${g.total_count} installed`,
                repo: 'group',
                description: `Package group with ${g.total_count} packages`,
                installed: g.installed_count > 0,
                groupData: g
              }))
            } catch (error) {
              console.error('Failed to load groups:', error)
              packages.value = []
            }
            break
          case 'files':
            // Files view will be handled separately with search
            packages.value = []
            break
          case 'repos':
            // Show repositories
            const repos = await invoke('list_repositories')
            packages.value = repos.map(r => ({
              name: r.name,
              version: `${r.package_count} packages`,
              repo: 'repository',
              description: r.servers.length > 0 ? r.servers[0] : 'No servers',
              installed: r.enabled,
              repoData: r
            }))
            break
          case 'search':
            if (searchQuery.value.trim()) {
              packages.value = await invoke('search_package', { query: searchQuery.value })
            }
            break
        }
      } catch (error) {
        console.error('Error loading packages:', error)
      } finally {
        loading.value = false
      }
    }

    const handleSearchExample = (exampleQuery) => {
      searchQuery.value = exampleQuery
      handleSearch()
    }

    const handleSearch = async () => {
      if (!searchQuery.value.trim()) return
      
      loading.value = true
      try {
        if (activeView.value === 'files') {
          // Search for files or find file owner
          if (searchQuery.value.startsWith('/')) {
            // Find package that owns this file
            try {
              const owner = await invoke('find_file_owner', { filePath: searchQuery.value })
              packages.value = [{
                name: owner,
                version: '',
                repo: 'file-owner',
                description: `Owns file: ${searchQuery.value}`,
                installed: true
              }]
            } catch (error) {
              packages.value = []
              logs.value = [`File '${searchQuery.value}' is not owned by any package`]
            }
          } else {
            // Search for files matching pattern
            const files = await invoke('search_files', { pattern: searchQuery.value })
            packages.value = files.map(f => ({
              name: f.package,
              version: f.path,
              repo: 'file',
              description: f.is_directory ? 'Directory' : 'File',
              installed: true
            }))
          }
        } else {
          // Regular package search
          const aurEnabled = config.value.aurSupport === true
          const aurHelper = config.value.aurHelper || 'yay'
          
          packages.value = await invoke('search_package', { 
            query: searchQuery.value,
            aurEnabled: aurEnabled,
            aurHelper: aurHelper
          })
          activeView.value = 'search'
        }
      } catch (error) {
        console.error('Search error:', error)
      } finally {
        loading.value = false
      }
    }

    const isSelected = (pkg) => {
      return selectedPackages.value.some(p => p.name === pkg.name)
    }

    const toggleSelect = (pkg) => {
      const index = selectedPackages.value.findIndex(p => p.name === pkg.name)
      if (index > -1) {
        selectedPackages.value.splice(index, 1)
      } else {
        selectedPackages.value.push(pkg)
      }
    }

    const handleInstall = async (pkg) => {
      // Close details modal if open
      showDetailsModal.value = false
      
      // Always show confirmation dialog for package installation
      showConfirm(
        'Install Package',
        `Are you sure you want to install ${pkg.name}?\n\nThis will download and install the package on your system.`,
        async () => {
          currentOperation.value = `Installing ${pkg.name}`
          logs.value = []
          operationCompleted.value = false
          operationSuccess.value = false
          showLogModal.value = true

          try {
            await invoke('install_package', { pkg: pkg.name })
          } catch (error) {
            logs.value.push(`Error: ${error}`)
            operationCompleted.value = true
            operationSuccess.value = false
          }
        },
        'warning',
        'Install'
      )
    }

    const handleRemove = async (pkg) => {
      // Close details modal if open
      showDetailsModal.value = false
      
      // Always show confirmation dialog for package removal
      showConfirm(
        'Remove Package',
        `Are you sure you want to remove ${pkg.name}?\n\nThis will uninstall the package from your system.`,
        async () => {
          currentOperation.value = `Removing ${pkg.name}`
          logs.value = []
          operationCompleted.value = false
          operationSuccess.value = false
          showLogModal.value = true

          try {
            await invoke('remove_package', { pkg: pkg.name })
          } catch (error) {
            logs.value.push(`Error: ${error}`)
            operationCompleted.value = true
            operationSuccess.value = false
          }
        },
        'danger',
        'Remove'
      )
    }

    const handleInstallSelected = async () => {
      if (selectedPackages.value.length === 0) return

      const packagesToInstall = [...selectedPackages.value]
      const packageNames = packagesToInstall.map(p => p.name).join(' ')
      const packageCount = packagesToInstall.length

      const doInstall = async () => {
        currentOperation.value = `Installing ${packageCount} packages`
        logs.value = []
        operationCompleted.value = false
        operationSuccess.value = false
        showLogModal.value = true

        try {
          await invoke('install_package', { pkg: packageNames })
        } catch (error) {
          logs.value.push(`Error: ${error}`)
          operationCompleted.value = true
          operationSuccess.value = false
        }
        
        selectedPackages.value = []
      }

      if (config.value.confirmActions) {
        const packageList = packagesToInstall.map(p => p.name).join(', ')
        showConfirm(
          'Install Multiple Packages',
          `Install ${packageCount} packages:\n${packageList}`,
          doInstall,
          'warning',
          'Install All'
        )
      } else {
        await doInstall()
      }
    }

    const handleRemoveSelected = async () => {
      if (selectedPackages.value.length === 0) return

      const packagesToRemove = [...selectedPackages.value]
      const packageNames = packagesToRemove.map(p => p.name).join(' ')
      const packageCount = packagesToRemove.length

      const doRemove = async () => {
        currentOperation.value = `Removing ${packageCount} packages`
        logs.value = []
        operationCompleted.value = false
        operationSuccess.value = false
        showLogModal.value = true

        try {
          await invoke('remove_package', { pkg: packageNames })
        } catch (error) {
          logs.value.push(`Error: ${error}`)
          operationCompleted.value = true
          operationSuccess.value = false
        }
        
        selectedPackages.value = []
      }

      if (config.value.confirmActions) {
        const packageList = packagesToRemove.map(p => p.name).join(', ')
        showConfirm(
          'Remove Multiple Packages',
          `Remove ${packageCount} packages:\n${packageList}`,
          doRemove,
          'danger',
          'Remove All'
        )
      } else {
        await doRemove()
      }
    }

    const handleUpdateSystem = async () => {
      // Always show confirmation dialog for system updates
      showConfirm(
        'Update System',
        'Are you sure you want to update your system?\n\nThis will update all installed packages to their latest versions. This operation may take some time.',
        async () => {
          currentOperation.value = 'Updating system'
          logs.value = []
          operationCompleted.value = false
          operationSuccess.value = false
          showLogModal.value = true

          try {
            await invoke('update_system')
          } catch (error) {
            logs.value.push(`Error: ${error}`)
            operationCompleted.value = true
            operationSuccess.value = false
          }
        },
        'warning',
        'Update System'
      )
    }

    const handleSelectAll = () => {
      packages.value.forEach(pkg => {
        if (!isSelected(pkg)) {
          toggleSelect(pkg)
        }
      })
    }

    const handleClearSelection = () => {
      selectedPackages.value.forEach(pkg => {
        toggleSelect(pkg)
      })
    }

    const handleShowDependencies = (packageName) => {
      showDependencyGraph(packageName)
    }

    const toggleNotificationCenter = () => {
      showNotificationCenter.value = !showNotificationCenter.value
    }

    const handleNotificationAction = ({ notificationId, action }) => {
      switch (action.action) {
        case 'view-logs':
          showLogModal.value = true
          break
        case 'view-updates':
          handleViewChange('updates')
          break
        default:
          console.log('Unknown notification action:', action)
      }
    }

    const showDependencyGraph = (packageName) => {
      dependencyGraphPackage.value = packageName
      showDependencyGraphModal.value = true
    }

    const handleDependencyPackageClick = (packageName) => {
      // Close dependency graph and show package details
      showDependencyGraphModal.value = false
      
      // Find the package in current packages list or fetch its info
      const existingPackage = packages.value.find(p => p.name === packageName)
      if (existingPackage) {
        showPackageDetails(existingPackage)
      } else {
        // Create a minimal package object and fetch details
        const packageObj = {
          name: packageName,
          version: 'Unknown',
          repo: 'unknown',
          description: '',
          installed: false
        }
        showPackageDetails(packageObj)
      }
    }

    const closeLogModal = () => {
      showLogModal.value = false
      handleViewChange(activeView.value)
    }

    const restoreWindowState = async () => {
      try {
        const savedState = localStorage.getItem('window_state')
        if (savedState) {
          const { width, height, x, y } = JSON.parse(savedState)
          await appWindow.setSize({ width, height })
          await appWindow.setPosition({ x, y })
        }
      } catch (error) {
        console.error('Failed to restore window state:', error)
      }
    }

    const saveWindowState = async () => {
      try {
        const size = await appWindow.innerSize()
        const position = await appWindow.innerPosition()
        const state = {
          width: size.width,
          height: size.height,
          x: position.x,
          y: position.y
        }
        localStorage.setItem('window_state', JSON.stringify(state))
      } catch (error) {
        console.error('Failed to save window state:', error)
      }
    }

    const sendNotif = (type, title, message, options = {}) => {
      if (!notificationSystem.value) return

      notificationSystem.value.show(type, title, message, options)
    }


    const handleKeydown = (event) => {
      // Ignore if user is typing in an input
      if (event.target.tagName === 'INPUT' || event.target.tagName === 'TEXTAREA') {
        return
      }

      // Ctrl+F or Cmd+F: Focus search bar
      if ((event.ctrlKey || event.metaKey) && event.key === 'f') {
        event.preventDefault()
        const searchInput = document.querySelector('input[type="text"]')
        if (searchInput) {
          searchInput.focus()
          searchInput.select()
        }
        return
      }

      // Ctrl+T: Toggle view mode
      if ((event.ctrlKey || event.metaKey) && event.key === 't') {
        event.preventDefault()
        toggleViewMode()
        return
      }

      // Ctrl+B: Toggle sidebar
      if ((event.ctrlKey || event.metaKey) && event.key === 'b') {
        event.preventDefault()
        toggleSidebar()
        return
      }

      // Escape: Clear search
      if (event.key === 'Escape') {
        if (searchQuery.value) {
          searchQuery.value = ''
          handleSearch()
        } else if (showDetailsModal.value) {
          showDetailsModal.value = false
        } else if (showLogModal.value) {
          closeLogModal()
        } else if (showSettingsModal.value) {
          showSettingsModal.value = false
        } else if (showConfirmDialog.value) {
          showConfirmDialog.value = false
        }
        return
      }

      // Ctrl+U: Update system
      if ((event.ctrlKey || event.metaKey) && event.key === 'u') {
        event.preventDefault()
        handleUpdateSystem()
        return
      }

      // Ctrl+R: Refresh current view
      if ((event.ctrlKey || event.metaKey) && event.key === 'r') {
        event.preventDefault()
        handleViewChange(activeView.value)
        return
      }

      // F1: Show keyboard shortcuts
      if (event.key === 'F1') {
        event.preventDefault()
        showKeyboardShortcutsModal.value = true
        return
      }
    }

    onMounted(async () => {
      await restoreWindowState()

      // Add global keyboard shortcuts
      document.addEventListener('keydown', handleKeydown)

      if (config.value.checkUpdatesOnStartup) {
        await handleViewChange('updates')
        const updateCount = packages.value.length
        if (updateCount > 0) {
          sendNotif('info', 'Updates Available', `${updateCount} package update${updateCount > 1 ? 's' : ''} available`, {
            actions: [{ label: 'View Updates', action: 'view-updates' }]
          })
        }
      } else {
        await handleViewChange('dashboard')
      }

      setupAutoRefresh()

      let saveTimeout
      const debouncedSave = () => {
        clearTimeout(saveTimeout)
        saveTimeout = setTimeout(saveWindowState, 500)
      }

      appWindow.onResized(debouncedSave)
      appWindow.onMoved(debouncedSave)

      listen('install-log', (event) => {
        logs.value.push(event.payload)
      })

      listen('remove-log', (event) => {
        logs.value.push(event.payload)
      })

      listen('update-log', (event) => {
        logs.value.push(event.payload)
      })

      listen('cache-clean-output', (event) => {
        logs.value.push(event.payload)
      })

      listen('install-complete', async (event) => {
        operationCompleted.value = true
        operationSuccess.value = event.payload.success
        logs.value.push(event.payload.message)
        if (event.payload.success) {
          sendNotif('success', 'Installation Complete', event.payload.message, {
            actions: [{ label: 'View Logs', action: 'view-logs' }]
          })
          // Refresh the current view to show newly installed packages
          await handleViewChange(activeView.value)
        } else {
          sendNotif('error', 'Installation Failed', event.payload.message, {
            actions: [{ label: 'View Logs', action: 'view-logs', primary: true }]
          })
        }
      })

      listen('remove-complete', async (event) => {
        operationCompleted.value = true
        operationSuccess.value = event.payload.success
        logs.value.push(event.payload.message)
        if (event.payload.success) {
          sendNotif('success', 'Removal Complete', event.payload.message, {
            actions: [{ label: 'View Logs', action: 'view-logs' }]
          })
          // Refresh the current view to update package status
          await handleViewChange(activeView.value)
        } else {
          sendNotif('error', 'Removal Failed', event.payload.message, {
            actions: [{ label: 'View Logs', action: 'view-logs', primary: true }]
          })
        }
      })

      listen('update-complete', async (event) => {
        operationCompleted.value = true
        operationSuccess.value = event.payload.success
        logs.value.push(event.payload.message)
        if (event.payload.success) {
          sendNotif('success', 'System Update Complete', event.payload.message, {
            actions: [{ label: 'View Logs', action: 'view-logs' }]
          })
          // Refresh the current view to update package statuses
          await handleViewChange(activeView.value)
        } else {
          sendNotif('error', 'System Update Failed', event.payload.message, {
            actions: [{ label: 'View Logs', action: 'view-logs', primary: true }]
          })
        }
      })

      listen('cache-clean-complete', async (event) => {
        operationCompleted.value = true
        operationSuccess.value = event.payload.success
        logs.value.push(event.payload.message)
        
        // Mark that cache was just cleaned so we refresh when modal reopens
        cacheJustCleaned.value = true
      })
    })

    onUnmounted(async () => {
      if (refreshInterval) {
        clearInterval(refreshInterval)
      }
      // Remove keyboard event listener
      document.removeEventListener('keydown', handleKeydown)
      await saveWindowState()
    })

    return {
      activeView,
      searchQuery,
      displayPackages,
      loading,
      selectedPackages,
      showLogModal,
      logs,
      currentOperation,
      operationCompleted,
      operationSuccess,
      darkMode,
      config,
      showSettingsModal,
      settingsModal,
      showConfirmDialog,
      confirmDialog,
      showDetailsModal,
      selectedPackageForDetails,
      packageDetails,
      showDependencyGraphModal,
      dependencyGraphPackage,
      showBackupModal,
      showKeyboardShortcutsModal,
      showNotificationCenter,
      notificationSystem,
      systemStats,
      recentUpdates,
      popularPackages,
      toggleTheme,
      toggleViewMode,
      toggleSidebar,
      toggleNotificationCenter,
      saveSettings,
      handleViewChange,
      handleSearch,
      handleSearchExample,
      toggleSelect,
      handleInstall,
      handleRemove,
      handleInstallSelected,
      handleRemoveSelected,
      handleUpdateSystem,
      handleCleanCache,
      showPackageDetails,
      showDependencyGraph,
      handleDependencyPackageClick,
      handleSelectAll,
      handleClearSelection,
      handleShowDependencies,
      handleNotificationAction,
      closeLogModal
    }
  }
}
</script>

