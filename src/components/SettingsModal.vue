<template>
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl w-full max-w-3xl max-h-[90vh] flex flex-col">
      <div class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700">
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white flex items-center gap-2">
          <svg class="w-7 h-7 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd" />
          </svg>
          Settings
        </h2>
        <button @click="$emit('close')" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200">
          <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </button>
      </div>

      <div class="flex-1 overflow-auto p-6 scrollbar-thin space-y-6">
        <div class="space-y-4">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">System Integration</h3>
          
          <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4">
            <div class="flex items-start gap-3">
              <svg class="w-6 h-6 text-blue-600 dark:text-blue-400 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" />
              </svg>
              <div class="flex-1">
                <h4 class="font-semibold text-gray-900 dark:text-white">Password-Free Package Management</h4>
                <p class="text-sm text-gray-600 dark:text-gray-300 mt-1">
                  Install the polkit policy to stop password prompts for every operation. You'll only authenticate once when you open GuiMan.
                </p>
                <div class="mt-3 flex items-center gap-3">
                  <div v-if="polkitInstalled" class="flex items-center gap-2 text-green-600 dark:text-green-400">
                    <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                    </svg>
                    <span class="text-sm font-medium">Installed</span>
                  </div>
                  <button 
                    v-else
                    @click="installPolkit"
                    :disabled="installingPolkit"
                    class="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white text-sm rounded-lg font-medium transition-colors"
                  >
                    {{ installingPolkit ? 'Installing...' : 'Install Now' }}
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="border-t border-gray-200 dark:border-gray-700 pt-6 space-y-4">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">General</h3>
          
          <div class="flex items-center justify-between">
            <div>
              <label class="text-sm font-medium text-gray-700 dark:text-gray-300">Check updates on startup</label>
              <p class="text-xs text-gray-500 dark:text-gray-400">Automatically check for package updates when app starts</p>
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
              <input type="checkbox" v-model="localConfig.checkUpdatesOnStartup" class="sr-only peer">
              <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
            </label>
          </div>

          <div class="flex items-center justify-between">
            <div>
              <label class="text-sm font-medium text-gray-700 dark:text-gray-300">Confirm actions</label>
              <p class="text-xs text-gray-500 dark:text-gray-400">Show confirmation dialog before install/remove</p>
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
              <input type="checkbox" v-model="localConfig.confirmActions" class="sr-only peer">
              <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
            </label>
          </div>

          <div class="flex items-center justify-between">
            <div>
              <label class="text-sm font-medium text-gray-700 dark:text-gray-300">Show notifications</label>
              <p class="text-xs text-gray-500 dark:text-gray-400">Display desktop notifications for operations</p>
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
              <input type="checkbox" v-model="localConfig.showNotifications" class="sr-only peer">
              <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
            </label>
          </div>

          <div class="flex items-center justify-between">
            <div>
              <label class="text-sm font-medium text-gray-700 dark:text-gray-300">Show package descriptions</label>
              <p class="text-xs text-gray-500 dark:text-gray-400">Display package descriptions in table</p>
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
              <input type="checkbox" v-model="localConfig.showDescriptions" class="sr-only peer">
              <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
            </label>
          </div>

          <div class="flex items-center justify-between">
            <div>
              <label class="text-sm font-medium text-gray-700 dark:text-gray-300">Compact view</label>
              <p class="text-xs text-gray-500 dark:text-gray-400">Use smaller text and compact spacing</p>
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
              <input type="checkbox" v-model="localConfig.compactView" class="sr-only peer">
              <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
            </label>
          </div>
        </div>

        <div class="border-t border-gray-200 dark:border-gray-700 pt-6 space-y-4">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Auto Refresh</h3>
          
          <div class="flex items-center justify-between">
            <div>
              <label class="text-sm font-medium text-gray-700 dark:text-gray-300">Enable auto refresh</label>
              <p class="text-xs text-gray-500 dark:text-gray-400">Automatically refresh package list</p>
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
              <input type="checkbox" v-model="localConfig.autoRefresh" class="sr-only peer">
              <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
            </label>
          </div>

          <div v-if="localConfig.autoRefresh">
            <label class="text-sm font-medium text-gray-700 dark:text-gray-300">Refresh interval (seconds)</label>
            <input type="number" v-model.number="localConfig.refreshInterval" min="60" max="3600" class="mt-1 w-full px-3 py-2 bg-gray-50 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-900 dark:text-white">
          </div>
        </div>

        <div class="border-t border-gray-200 dark:border-gray-700 pt-6 space-y-4">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">AUR Support (Experimental)</h3>
          
          <div class="flex items-center justify-between">
            <div>
              <label class="text-sm font-medium text-gray-700 dark:text-gray-300">Enable AUR</label>
              <p class="text-xs text-gray-500 dark:text-gray-400">Use AUR helper for additional packages</p>
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
              <input type="checkbox" v-model="localConfig.aurSupport" class="sr-only peer">
              <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
            </label>
          </div>

          <div v-if="localConfig.aurSupport">
            <label class="text-sm font-medium text-gray-700 dark:text-gray-300">AUR Helper</label>
            <select v-model="localConfig.aurHelper" class="mt-1 w-full px-3 py-2 bg-gray-50 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-900 dark:text-white">
              <option value="yay">yay</option>
              <option value="paru">paru</option>
            </select>
          </div>
        </div>

        <div class="border-t border-gray-200 dark:border-gray-700 pt-6 space-y-4">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Package Cache</h3>
          
          <div class="flex items-center justify-between">
            <div>
              <label class="text-sm font-medium text-gray-700 dark:text-gray-300">Keep package cache</label>
              <p class="text-xs text-gray-500 dark:text-gray-400">Retain downloaded packages in /var/cache/pacman/pkg</p>
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
              <input type="checkbox" v-model="localConfig.keepPackageCache" class="sr-only peer">
              <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
            </label>
          </div>

          <button @click="$emit('clean-cache')" class="w-full px-4 py-2 bg-orange-600 hover:bg-orange-700 text-white rounded-lg font-medium transition-colors">
            Clean Package Cache
          </button>
        </div>

        <div class="border-t border-gray-200 dark:border-gray-700 pt-6 space-y-4">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Advanced</h3>
          
          <div>
            <label class="text-sm font-medium text-gray-700 dark:text-gray-300">Max concurrent downloads</label>
            <input type="number" v-model.number="localConfig.maxConcurrentDownloads" min="1" max="10" class="mt-1 w-full px-3 py-2 bg-gray-50 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-900 dark:text-white">
          </div>

          <button @click="resetConfig" class="w-full px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg font-medium transition-colors">
            Reset to Defaults
          </button>
        </div>
      </div>

      <div class="p-6 border-t border-gray-200 dark:border-gray-700 flex justify-end gap-3">
        <button @click="$emit('close')" class="px-6 py-2 bg-gray-600 hover:bg-gray-700 text-white rounded-lg font-medium transition-colors">
          Cancel
        </button>
        <button @click="saveSettings" class="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors">
          Save Settings
        </button>
      </div>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri'

export default {
  name: 'SettingsModal',
  props: {
    config: Object
  },
  emits: ['close', 'save', 'clean-cache'],
  data() {
    return {
      localConfig: { ...this.config },
      polkitInstalled: false,
      installingPolkit: false
    }
  },
  async mounted() {
    try {
      this.polkitInstalled = await invoke('check_polkit_policy')
    } catch (error) {
      console.error('Failed to check polkit policy:', error)
    }
  },
  methods: {
    async installPolkit() {
      this.installingPolkit = true
      try {
        const result = await invoke('install_polkit_policy')
        this.polkitInstalled = true
        alert('✓ ' + result.message + '\n\nRestart GuiMan for changes to take effect.')
      } catch (error) {
        alert('✗ Failed to install polkit policy:\n' + error)
      } finally {
        this.installingPolkit = false
      }
    },
    saveSettings() {
      this.$emit('save', this.localConfig)
    },
    resetConfig() {
      if (confirm('Are you sure you want to reset all settings to defaults?')) {
        this.$emit('save', null)
        this.$emit('close')
      }
    }
  }
}
</script>

