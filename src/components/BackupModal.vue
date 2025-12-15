<template>
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl w-full max-w-4xl max-h-[90vh] flex flex-col">
      <div class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700">
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white flex items-center gap-2">
          <svg class="w-7 h-7 text-green-600" fill="currentColor" viewBox="0 0 20 20">
            <path d="M4 4a2 2 0 00-2 2v8a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2H4zm0 2h12v8H4V6z"/>
            <path d="M6 8h8v2H6V8zm0 4h8v2H6v-2z"/>
          </svg>
          Package Backup & Restore
        </h2>
        
        <div class="flex items-center gap-3">
          <div class="flex gap-2">
            <button 
              @click="activeTab = 'backups'"
              :class="[
                'px-3 py-1.5 text-sm rounded-lg font-medium transition-colors',
                activeTab === 'backups' 
                  ? 'bg-green-600 text-white' 
                  : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600'
              ]"
            >
              Backups
            </button>
            <button 
              @click="activeTab = 'hooks'"
              :class="[
                'px-3 py-1.5 text-sm rounded-lg font-medium transition-colors',
                activeTab === 'hooks' 
                  ? 'bg-green-600 text-white' 
                  : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600'
              ]"
            >
              Hooks
            </button>
            <button 
              @click="activeTab = 'export'"
              :class="[
                'px-3 py-1.5 text-sm rounded-lg font-medium transition-colors',
                activeTab === 'export' 
                  ? 'bg-green-600 text-white' 
                  : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600'
              ]"
            >
              Export
            </button>
          </div>
          
          <button @click="$emit('close')" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200">
            <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
          </button>
        </div>
      </div>

      <div class="flex-1 overflow-auto p-6">
        <!-- Backups Tab -->
        <div v-if="activeTab === 'backups'" class="space-y-6">
          <!-- Create Backup Section -->
          <div class="bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg p-4">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2">
              <svg class="w-5 h-5 text-green-600" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
              </svg>
              Create New Backup
            </h3>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Backup Name</label>
                <input 
                  v-model="newBackup.name"
                  type="text" 
                  placeholder="Leave empty for auto-generated name"
                  class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                >
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Description</label>
                <input 
                  v-model="newBackup.description"
                  type="text" 
                  placeholder="Optional description"
                  class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                >
              </div>
            </div>
            
            <button 
              @click="createBackup"
              :disabled="creatingBackup"
              class="px-4 py-2 bg-green-600 hover:bg-green-700 disabled:bg-gray-400 text-white rounded-lg font-medium transition-colors flex items-center gap-2"
            >
              <svg v-if="creatingBackup" class="w-4 h-4 animate-spin" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd" />
              </svg>
              <svg v-else class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd" />
              </svg>
              {{ creatingBackup ? 'Creating...' : 'Create Backup' }}
            </button>
          </div>
          
          <!-- Existing Backups -->
          <div>
            <div class="flex items-center justify-between mb-4">
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Existing Backups</h3>
              <button @click="loadBackups" class="text-sm text-blue-600 hover:text-blue-700">
                <svg class="w-4 h-4 inline mr-1" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd" />
                </svg>
                Refresh
              </button>
            </div>
            
            <div v-if="loading" class="text-center py-8">
              <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-green-600 mb-4"></div>
              <p class="text-gray-600 dark:text-gray-400">Loading backups...</p>
            </div>
            
            <div v-else-if="backups.length === 0" class="text-center py-8">
              <svg class="w-16 h-16 text-gray-400 mx-auto mb-4" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd" />
              </svg>
              <p class="text-gray-600 dark:text-gray-400 text-lg font-medium">No backups found</p>
              <p class="text-gray-500 dark:text-gray-500 text-sm">Create your first backup to get started</p>
            </div>
            
            <div v-else class="space-y-3">
              <div 
                v-for="backup in backups" 
                :key="backup.name"
                class="bg-white dark:bg-gray-700 border border-gray-200 dark:border-gray-600 rounded-lg p-4 hover:shadow-md transition-shadow"
              >
                <div class="flex items-center justify-between">
                  <div class="flex-1">
                    <div class="flex items-center gap-3 mb-2">
                      <h4 class="font-semibold text-gray-900 dark:text-white">{{ backup.name }}</h4>
                      <span class="text-xs bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-300 px-2 py-1 rounded-full">
                        {{ backup.total_packages }} packages
                      </span>
                      <span v-if="backup.aur_packages.length > 0" class="text-xs bg-orange-100 dark:bg-orange-900 text-orange-700 dark:text-orange-300 px-2 py-1 rounded-full">
                        {{ backup.aur_packages.length }} AUR
                      </span>
                    </div>
                    <p class="text-sm text-gray-600 dark:text-gray-400 mb-1">{{ backup.description || 'No description' }}</p>
                    <p class="text-xs text-gray-500 dark:text-gray-500">Created: {{ formatDate(backup.timestamp) }}</p>
                  </div>
                  
                  <div class="flex gap-2">
                    <button 
                      @click="showRestoreDialog(backup)"
                      class="px-3 py-1.5 bg-blue-600 hover:bg-blue-700 text-white text-sm rounded-lg font-medium transition-colors"
                    >
                      Restore
                    </button>
                    <button 
                      @click="deleteBackup(backup.name)"
                      class="px-3 py-1.5 bg-red-600 hover:bg-red-700 text-white text-sm rounded-lg font-medium transition-colors"
                    >
                      Delete
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <!-- Hooks Tab -->
        <div v-if="activeTab === 'hooks'" class="space-y-4">
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Pacman Hooks</h3>
            <button @click="loadHooks" class="text-sm text-blue-600 hover:text-blue-700">
              <svg class="w-4 h-4 inline mr-1" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd" />
              </svg>
              Refresh
            </button>
          </div>
          
          <div v-if="loadingHooks" class="text-center py-8">
            <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-green-600 mb-4"></div>
            <p class="text-gray-600 dark:text-gray-400">Loading hooks...</p>
          </div>
          
          <div v-else-if="hooks.length === 0" class="text-center py-8">
            <svg class="w-16 h-16 text-gray-400 mx-auto mb-4" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd" />
            </svg>
            <p class="text-gray-600 dark:text-gray-400 text-lg font-medium">No hooks found</p>
          </div>
          
          <div v-else class="space-y-3">
            <div 
              v-for="hook in hooks" 
              :key="hook.name"
              class="bg-white dark:bg-gray-700 border border-gray-200 dark:border-gray-600 rounded-lg p-4"
            >
              <div class="flex items-start justify-between">
                <div class="flex-1">
                  <div class="flex items-center gap-3 mb-2">
                    <h4 class="font-semibold text-gray-900 dark:text-white">{{ hook.name }}</h4>
                    <span :class="[
                      'text-xs px-2 py-1 rounded-full',
                      hook.enabled 
                        ? 'bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-300' 
                        : 'bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-300'
                    ]">
                      {{ hook.enabled ? 'Enabled' : 'Disabled' }}
                    </span>
                  </div>
                  <p class="text-sm text-gray-600 dark:text-gray-400 mb-2">{{ hook.description }}</p>
                  <p class="text-xs text-gray-500 dark:text-gray-500 mb-2">{{ hook.path }}</p>
                  <div v-if="hook.triggers.length > 0" class="flex flex-wrap gap-1">
                    <span 
                      v-for="trigger in hook.triggers" 
                      :key="trigger"
                      class="text-xs bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-300 px-2 py-1 rounded"
                    >
                      {{ trigger }}
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <!-- Export Tab -->
        <div v-if="activeTab === 'export'" class="space-y-6">
          <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2">
              <svg class="w-5 h-5 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd" />
              </svg>
              Export Package List
            </h3>
            
            <div class="grid grid-cols-2 md:grid-cols-4 gap-3 mb-4">
              <button 
                @click="exportPackages('txt')"
                class="px-4 py-3 bg-white dark:bg-gray-700 border border-gray-200 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-600 transition-colors text-left"
              >
                <div class="text-sm font-medium text-gray-900 dark:text-white">.txt</div>
                <div class="text-xs text-gray-500 dark:text-gray-400">Plain text list</div>
              </button>
              
              <button 
                @click="exportPackages('json')"
                class="px-4 py-3 bg-white dark:bg-gray-700 border border-gray-200 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-600 transition-colors text-left"
              >
                <div class="text-sm font-medium text-gray-900 dark:text-white">.json</div>
                <div class="text-xs text-gray-500 dark:text-gray-400">JSON format</div>
              </button>
              
              <button 
                @click="exportPackages('csv')"
                class="px-4 py-3 bg-white dark:bg-gray-700 border border-gray-200 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-600 transition-colors text-left"
              >
                <div class="text-sm font-medium text-gray-900 dark:text-white">.csv</div>
                <div class="text-xs text-gray-500 dark:text-gray-400">Comma separated</div>
              </button>
              
              <button 
                @click="exportPackages('pacman')"
                class="px-4 py-3 bg-white dark:bg-gray-700 border border-gray-200 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-600 transition-colors text-left"
              >
                <div class="text-sm font-medium text-gray-900 dark:text-white">pacman</div>
                <div class="text-xs text-gray-500 dark:text-gray-400">Install command</div>
              </button>
            </div>
            
            <div v-if="exportResult" class="mt-4">
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Export Result</label>
              <textarea 
                v-model="exportResult"
                readonly
                rows="10"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-800 text-gray-900 dark:text-white font-mono text-sm"
              ></textarea>
              <button 
                @click="copyToClipboard"
                class="mt-2 px-3 py-1.5 bg-blue-600 hover:bg-blue-700 text-white text-sm rounded-lg font-medium transition-colors"
              >
                Copy to Clipboard
              </button>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Restore Dialog -->
      <div v-if="showRestore" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-60 p-4">
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl w-full max-w-md">
          <div class="p-6">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Restore Backup</h3>
            <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
              Restore packages from "{{ selectedBackup?.name }}"?
            </p>
            
            <div class="space-y-3 mb-6">
              <label class="flex items-center">
                <input v-model="restoreOptions.installMissing" type="checkbox" class="mr-2">
                <span class="text-sm text-gray-700 dark:text-gray-300">Install missing packages</span>
              </label>
              <label class="flex items-center">
                <input v-model="restoreOptions.removeExtra" type="checkbox" class="mr-2">
                <span class="text-sm text-gray-700 dark:text-gray-300">Remove extra packages (not recommended)</span>
              </label>
            </div>
            
            <div class="flex gap-3">
              <button 
                @click="confirmRestore"
                :disabled="restoringBackup"
                class="flex-1 px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 text-white rounded-lg font-medium transition-colors"
              >
                {{ restoringBackup ? 'Restoring...' : 'Restore' }}
              </button>
              <button 
                @click="showRestore = false"
                class="flex-1 px-4 py-2 bg-gray-600 hover:bg-gray-700 text-white rounded-lg font-medium transition-colors"
              >
                Cancel
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core'

export default {
  name: 'BackupModal',
  emits: ['close'],
  data() {
    return {
      activeTab: 'backups',
      loading: false,
      loadingHooks: false,
      creatingBackup: false,
      restoringBackup: false,
      backups: [],
      hooks: [],
      exportResult: '',
      showRestore: false,
      selectedBackup: null,
      newBackup: {
        name: '',
        description: ''
      },
      restoreOptions: {
        installMissing: true,
        removeExtra: false
      }
    }
  },
  mounted() {
    this.loadBackups()
  },
  methods: {
    async loadBackups() {
      this.loading = true
      try {
        this.backups = await invoke('list_backups')
      } catch (error) {
        console.error('Failed to load backups:', error)
        alert('Failed to load backups: ' + error)
      } finally {
        this.loading = false
      }
    },
    
    async loadHooks() {
      this.loadingHooks = true
      try {
        this.hooks = await invoke('list_pacman_hooks')
      } catch (error) {
        console.error('Failed to load hooks:', error)
        alert('Failed to load hooks: ' + error)
      } finally {
        this.loadingHooks = false
      }
    },
    
    async createBackup() {
      this.creatingBackup = true
      try {
        const result = await invoke('create_package_backup', {
          name: this.newBackup.name,
          description: this.newBackup.description
        })
        alert('✓ ' + result)
        this.newBackup.name = ''
        this.newBackup.description = ''
        await this.loadBackups()
      } catch (error) {
        console.error('Failed to create backup:', error)
        alert('✗ Failed to create backup: ' + error)
      } finally {
        this.creatingBackup = false
      }
    },
    
    async deleteBackup(backupName) {
      if (!confirm(`Are you sure you want to delete backup "${backupName}"?`)) {
        return
      }
      
      try {
        const result = await invoke('delete_package_backup', { backupName })
        alert('✓ ' + result)
        await this.loadBackups()
      } catch (error) {
        console.error('Failed to delete backup:', error)
        alert('✗ Failed to delete backup: ' + error)
      }
    },
    
    showRestoreDialog(backup) {
      this.selectedBackup = backup
      this.showRestore = true
    },
    
    async confirmRestore() {
      this.restoringBackup = true
      try {
        const result = await invoke('restore_package_backup', {
          backupName: this.selectedBackup.name,
          installMissing: this.restoreOptions.installMissing,
          removeExtra: this.restoreOptions.removeExtra
        })
        alert('✓ Restore completed:\n' + result)
        this.showRestore = false
      } catch (error) {
        console.error('Failed to restore backup:', error)
        alert('✗ Failed to restore backup: ' + error)
      } finally {
        this.restoringBackup = false
      }
    },
    
    async exportPackages(format) {
      try {
        this.exportResult = await invoke('export_packages', { format })
      } catch (error) {
        console.error('Failed to export packages:', error)
        alert('✗ Failed to export packages: ' + error)
      }
    },
    
    async copyToClipboard() {
      try {
        await navigator.clipboard.writeText(this.exportResult)
        alert('✓ Copied to clipboard')
      } catch (error) {
        console.error('Failed to copy to clipboard:', error)
        alert('✗ Failed to copy to clipboard')
      }
    },
    
    formatDate(timestamp) {
      try {
        const date = new Date(timestamp.replace(/_/g, ' ').replace(/-/g, '/'))
        return date.toLocaleString()
      } catch {
        return timestamp
      }
    }
  },
  watch: {
    activeTab(newTab) {
      if (newTab === 'hooks' && this.hooks.length === 0) {
        this.loadHooks()
      }
    }
  }
}
</script>
