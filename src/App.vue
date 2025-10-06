<template>
  <div :class="{ 'dark': darkMode }" class="h-screen w-screen">
    <div class="flex h-full bg-gray-50 dark:bg-gray-900">
      <Sidebar 
        :active-view="activeView" 
        @change-view="handleViewChange"
        @toggle-theme="toggleTheme"
        @open-settings="showSettingsModal = true"
        :dark-mode="darkMode"
        :aur-enabled="config.aurSupport"
      />
      
      <div class="flex-1 flex flex-col overflow-hidden">
        <SearchBar 
          v-model="searchQuery"
          :active-view="activeView"
          @search="handleSearch"
          @update-system="handleUpdateSystem"
        />
        
        <div class="flex-1 overflow-hidden">
          <PackageTable 
            :packages="displayPackages"
            :loading="loading"
            :selected-packages="selectedPackages"
            :active-view="activeView"
            :compact-view="config.compactView"
            :show-descriptions="config.showDescriptions"
            @toggle-select="toggleSelect"
            @install="handleInstallWithConfirm"
            @remove="handleRemoveWithConfirm"
            @show-details="showPackageDetails"
            @search-example="handleSearchExample"
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
      @install="handleInstallWithConfirm"
      @remove="handleRemoveWithConfirm"
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
    </div>
  </div>
</template>

<script>
import { ref, onMounted, computed, onUnmounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { appWindow } from '@tauri-apps/api/window'
import { sendNotification, isPermissionGranted, requestPermission } from '@tauri-apps/api/notification'
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
import configManager from './utils/config.js'

export default {
  name: 'App',
  components: {
    Sidebar,
    SearchBar,
    PackageTable,
    StatusBar,
    LogModal,
    SettingsModal,
    ConfirmDialog,
    PackageDetailsModal,
    DependencyGraph,
    BackupModal
  },
  setup() {
    const activeView = ref('installed')
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
    let refreshInterval = null

    const displayPackages = computed(() => {
      return packages.value
    })

    const toggleTheme = () => {
      darkMode.value = !darkMode.value
      config.value.theme = darkMode.value ? 'dark' : 'light'
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

    const handleInstallWithConfirm = (pkg) => {
      showDetailsModal.value = false
      if (config.value.confirmActions) {
        showConfirm(
          'Install Package',
          `Are you sure you want to install ${pkg.name}?`,
          () => handleInstall(pkg),
          'warning',
          'Install'
        )
      } else {
        handleInstall(pkg)
      }
    }

    const handleRemoveWithConfirm = (pkg) => {
      showDetailsModal.value = false
      if (config.value.confirmActions) {
        showConfirm(
          'Remove Package',
          `Are you sure you want to remove ${pkg.name}?`,
          () => handleRemove(pkg),
          'danger',
          'Remove'
        )
      } else {
        handleRemove(pkg)
      }
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

    const toggleSelect = (pkg) => {
      const index = selectedPackages.value.findIndex(p => p.name === pkg.name)
      if (index > -1) {
        selectedPackages.value.splice(index, 1)
      } else {
        selectedPackages.value.push(pkg)
      }
    }

    const handleInstall = async (pkg) => {
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
    }

    const handleRemove = async (pkg) => {
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

    const sendNotif = async (title, body) => {
      if (!config.value.showNotifications) return
      
      try {
        let permissionGranted = await isPermissionGranted()
        if (!permissionGranted) {
          const permission = await requestPermission()
          permissionGranted = permission === 'granted'
        }
        if (permissionGranted) {
          sendNotification({ title, body })
        }
      } catch (error) {
        console.error('Notification error:', error)
      }
    }


    onMounted(async () => {
      await restoreWindowState()

      if (config.value.checkUpdatesOnStartup) {
        await handleViewChange('updates')
        const updateCount = packages.value.length
        if (updateCount > 0) {
          await sendNotif('Updates Available', `${updateCount} package update${updateCount > 1 ? 's' : ''} available`)
        }
      } else {
        await handleViewChange('installed')
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

      listen('install-complete', (event) => {
        operationCompleted.value = true
        operationSuccess.value = event.payload
        logs.value.push(event.payload ? '✓ Installation complete!' : '✗ Installation failed!')
        if (event.payload) {
          sendNotif('Installation Complete', 'Package installation finished successfully')
        } else {
          sendNotif('Installation Failed', 'Package installation encountered an error')
        }
      })

      listen('remove-complete', (event) => {
        operationCompleted.value = true
        operationSuccess.value = event.payload
        logs.value.push(event.payload ? '✓ Removal complete!' : '✗ Removal failed!')
        if (event.payload) {
          sendNotif('Removal Complete', 'Package removal finished successfully')
        } else {
          sendNotif('Removal Failed', 'Package removal encountered an error')
        }
      })

      listen('update-complete', (event) => {
        operationCompleted.value = true
        operationSuccess.value = event.payload
        logs.value.push(event.payload ? '✓ Update complete!' : '✗ Update failed!')
        if (event.payload) {
          sendNotif('System Update Complete', 'All packages updated successfully')
        } else {
          sendNotif('System Update Failed', 'System update encountered an error')
        }
      })

      listen('cache-clean-complete', async (event) => {
        operationCompleted.value = true
        operationSuccess.value = event.payload
        logs.value.push(event.payload)
        
        // Mark that cache was just cleaned so we refresh when modal reopens
        cacheJustCleaned.value = true
      })
    })

    onUnmounted(async () => {
      if (refreshInterval) {
        clearInterval(refreshInterval)
      }
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
      toggleTheme,
      saveSettings,
      handleViewChange,
      handleSearch,
      handleSearchExample,
      toggleSelect,
      handleInstall,
      handleRemove,
      handleInstallWithConfirm,
      handleRemoveWithConfirm,
      handleInstallSelected,
      handleRemoveSelected,
      handleUpdateSystem,
      handleCleanCache,
      showPackageDetails,
      showDependencyGraph,
      handleDependencyPackageClick,
      closeLogModal
    }
  }
}
</script>

