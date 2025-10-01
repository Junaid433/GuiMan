<template>
  <div :class="{ 'dark': darkMode }" class="h-screen w-screen">
    <div class="flex h-full bg-gray-50 dark:bg-gray-900">
      <Sidebar 
        :active-view="activeView" 
        @change-view="handleViewChange"
        @toggle-theme="toggleTheme"
        @open-settings="showSettingsModal = true"
        :dark-mode="darkMode"
      />
      
      <div class="flex-1 flex flex-col overflow-hidden">
        <SearchBar 
          v-model="searchQuery" 
          @search="handleSearch"
          @update-system="handleUpdateSystem"
        />
        
        <div class="flex-1 overflow-hidden">
          <PackageTable 
            :packages="displayPackages"
            :loading="loading"
            :selected-packages="selectedPackages"
            :compact-view="config.compactView"
            :show-descriptions="config.showDescriptions"
            @toggle-select="toggleSelect"
            @install="handleInstallWithConfirm"
            @remove="handleRemoveWithConfirm"
            @show-details="showPackageDetails"
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
      />
    </div>
  </div>
</template>

<script>
import { ref, onMounted, computed, onUnmounted } from 'vue'
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
    PackageDetailsModal
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
    const showConfirmDialog = ref(false)
    const confirmDialog = ref({})
    const showDetailsModal = ref(false)
    const selectedPackageForDetails = ref(null)
    const packageDetails = ref({})
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
      
      try {
        const info = await invoke('get_package_info', { pkg: pkg.name })
        packageDetails.value = {
          size: info.installed_size || 'Unknown',
          url: info.url || '',
          licenses: info.licenses ? info.licenses.split(' ') : [],
          dependencies: info.depends_on ? info.depends_on.split(' ') : [],
          optionalDeps: info.optional_deps ? info.optional_deps.split(' ') : []
        }
      } catch (error) {
        console.error('Failed to get package info:', error)
      }
      
      showDetailsModal.value = true
    }

    const handleCleanCache = async () => {
      showSettingsModal.value = false
      currentOperation.value = 'Cleaning package cache'
      logs.value = []
      operationCompleted.value = false
      operationSuccess.value = false
      showLogModal.value = true

      try {
        await invoke('clean_cache')
      } catch (error) {
        logs.value.push(`Error: ${error}`)
        operationCompleted.value = true
        operationSuccess.value = false
      }
    }


    const handleViewChange = async (view) => {
      activeView.value = view
      selectedPackages.value = []
      loading.value = true

      try {
        switch (view) {
          case 'installed':
            packages.value = await invoke('list_installed')
            break
          case 'available':
            packages.value = await invoke('get_popular_packages')
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
            const history = await invoke('get_package_history')
            packages.value = history.map(line => ({
              name: line,
              version: '',
              repo: 'log',
              description: '',
              installed: false
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

    const handleSearch = async () => {
      if (!searchQuery.value.trim()) return
      
      loading.value = true
      try {
        packages.value = await invoke('search_package', { query: searchQuery.value })
        activeView.value = 'search'
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

      const doInstall = async () => {
        const packageNames = selectedPackages.value.map(p => p.name).join(' ')
        currentOperation.value = `Installing ${selectedPackages.value.length} packages`
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
        const packageList = selectedPackages.value.map(p => p.name).join(', ')
        showConfirm(
          'Install Multiple Packages',
          `Install ${selectedPackages.value.length} packages:\n${packageList}`,
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

      const doRemove = async () => {
        const packageNames = selectedPackages.value.map(p => p.name).join(' ')
        currentOperation.value = `Removing ${selectedPackages.value.length} packages`
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
        const packageList = selectedPackages.value.map(p => p.name).join(', ')
        showConfirm(
          'Remove Multiple Packages',
          `Remove ${selectedPackages.value.length} packages:\n${packageList}`,
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

      listen('cache-clean-log', (event) => {
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

      listen('cache-clean-complete', (event) => {
        operationCompleted.value = true
        operationSuccess.value = event.payload
        logs.value.push(event.payload ? '✓ Cache cleaned!' : '✗ Cache cleaning failed!')
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
      showConfirmDialog,
      confirmDialog,
      showDetailsModal,
      selectedPackageForDetails,
      packageDetails,
      toggleTheme,
      saveSettings,
      handleViewChange,
      handleSearch,
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
      closeLogModal
    }
  }
}
</script>

