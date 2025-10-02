<template>
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl w-full max-w-4xl max-h-[90vh] flex flex-col">
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

      <div class="flex flex-1 overflow-hidden">
        <div class="w-48 border-r border-gray-200 dark:border-gray-700 p-4">
          <button
            v-for="tab in tabs"
            :key="tab.id"
            @click="activeTab = tab.id"
            :class="[
              'w-full text-left px-3 py-2 rounded-lg mb-2 transition-colors text-sm font-medium',
              activeTab === tab.id
                ? 'bg-blue-600 text-white'
                : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'
            ]"
          >
            {{ tab.label }}
          </button>
        </div>

        <div class="flex-1 overflow-auto p-6 scrollbar-thin">
          <div v-if="activeTab === 'system'" class="space-y-6">
            <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4">
              <div class="flex items-start gap-3">
                <svg class="w-5 h-5 text-blue-600 dark:text-blue-400 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" />
                </svg>
                <div class="flex-1">
                  <h4 class="font-semibold text-gray-900 dark:text-white">Password-Free Mode</h4>
                  <p class="text-xs text-gray-600 dark:text-gray-300 mt-1">
                    Skip password prompts for package operations
                  </p>
                  <div class="mt-3">
                    <div v-if="polkitInstalled" class="flex items-center gap-2 text-green-600 dark:text-green-400 text-sm">
                      <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                      </svg>
                      Enabled
                    </div>
                    <button 
                      v-else
                      @click="installPolkit"
                      :disabled="installingPolkit"
                      class="px-3 py-1.5 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white text-sm rounded-lg transition-colors"
                    >
                      {{ installingPolkit ? 'Installing...' : 'Install' }}
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div v-if="activeTab === 'general'" class="space-y-4">
            <div class="flex items-center justify-between py-3 border-b border-gray-200 dark:border-gray-700">
              <div>
                <div class="text-sm font-medium text-gray-900 dark:text-white">Check updates on startup</div>
                <div class="text-xs text-gray-500 dark:text-gray-400">Auto-check for package updates</div>
              </div>
              <label class="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" v-model="localConfig.checkUpdatesOnStartup" class="sr-only peer">
                <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
              </label>
            </div>

            <div class="flex items-center justify-between py-3 border-b border-gray-200 dark:border-gray-700">
              <div>
                <div class="text-sm font-medium text-gray-900 dark:text-white">Confirm actions</div>
                <div class="text-xs text-gray-500 dark:text-gray-400">Show confirmation dialogs</div>
              </div>
              <label class="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" v-model="localConfig.confirmActions" class="sr-only peer">
                <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
              </label>
            </div>

            <div class="flex items-center justify-between py-3 border-b border-gray-200 dark:border-gray-700">
              <div>
                <div class="text-sm font-medium text-gray-900 dark:text-white">Show notifications</div>
                <div class="text-xs text-gray-500 dark:text-gray-400">Desktop notifications for operations</div>
              </div>
              <label class="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" v-model="localConfig.showNotifications" class="sr-only peer">
                <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
              </label>
            </div>

            <div class="flex items-center justify-between py-3 border-b border-gray-200 dark:border-gray-700">
              <div>
                <div class="text-sm font-medium text-gray-900 dark:text-white">Show descriptions</div>
                <div class="text-xs text-gray-500 dark:text-gray-400">Package descriptions in table</div>
              </div>
              <label class="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" v-model="localConfig.showDescriptions" class="sr-only peer">
                <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
              </label>
            </div>

            <div class="flex items-center justify-between py-3">
              <div>
                <div class="text-sm font-medium text-gray-900 dark:text-white">Compact view</div>
                <div class="text-xs text-gray-500 dark:text-gray-400">Smaller text and spacing</div>
              </div>
              <label class="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" v-model="localConfig.compactView" class="sr-only peer">
                <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
              </label>
            </div>
          </div>


          <div v-if="activeTab === 'aur'" class="space-y-4">
            <div class="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-3 mb-4">
              <p class="text-xs text-yellow-800 dark:text-yellow-200">⚠️ Experimental feature</p>
            </div>

            <div class="flex items-center justify-between py-3 border-b border-gray-200 dark:border-gray-700">
              <div>
                <div class="text-sm font-medium text-gray-900 dark:text-white">Enable AUR support</div>
                <div class="text-xs text-gray-500 dark:text-gray-400">Use AUR helper integration</div>
              </div>
              <label class="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" v-model="localConfig.aurSupport" class="sr-only peer">
                <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
              </label>
            </div>

            <div v-if="localConfig.aurSupport">
              <label class="text-sm font-medium text-gray-900 dark:text-white">AUR Helper</label>
              <select v-model="localConfig.aurHelper" class="mt-2 w-full px-3 py-2 bg-gray-50 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-900 dark:text-white">
                <option value="yay">yay</option>
                <option value="paru">paru</option>
              </select>
            </div>
          </div>

          <div v-if="activeTab === 'cache'" class="space-y-4">
            <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4 mb-4">
              <div class="space-y-3">
                <div class="flex items-center justify-between pb-3 border-b border-blue-200 dark:border-blue-700">
                  <div>
                    <div class="text-sm font-medium text-gray-900 dark:text-white">Pacman Cache</div>
                    <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">/var/cache/pacman/pkg</div>
                  </div>
                  <div class="text-2xl font-bold text-blue-600 dark:text-blue-400">
                    {{ cacheSizes.pacman }}
                  </div>
                </div>
                <div v-if="localConfig.aurSupport" class="flex items-center justify-between">
                  <div>
                    <div class="text-sm font-medium text-gray-900 dark:text-white">{{ (localConfig.aurHelper || 'aur').charAt(0).toUpperCase() + (localConfig.aurHelper || 'aur').slice(1) }} Cache</div>
                    <div class="text-xs text-gray-500 dark:text-gray-400 mt-1">{{ cacheSizes.yay_path }}</div>
                  </div>
                  <div class="text-2xl font-bold text-pink-600 dark:text-pink-400">
                    {{ cacheSizes.yay }}
                  </div>
                </div>
              </div>
            </div>

            <div class="flex items-center justify-between py-3 border-b border-gray-200 dark:border-gray-700">
              <div>
                <div class="text-sm font-medium text-gray-900 dark:text-white">Keep package cache</div>
                <div class="text-xs text-gray-500 dark:text-gray-400">Retain downloaded packages</div>
              </div>
              <label class="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" v-model="localConfig.keepPackageCache" class="sr-only peer">
                <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
              </label>
            </div>

            <div class="space-y-2">
              <button @click="refreshCacheSizes" class="w-full px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors">
                Refresh Cache Sizes
              </button>
              <button @click="$emit('clean-cache')" class="w-full px-4 py-2 bg-orange-600 hover:bg-orange-700 text-white rounded-lg font-medium transition-colors">
                Clean Package Cache
              </button>
            </div>
          </div>

          <div v-if="activeTab === 'advanced'" class="space-y-4">
            <div>
              <label class="text-sm font-medium text-gray-900 dark:text-white">Max concurrent downloads</label>
              <input type="number" v-model.number="localConfig.maxConcurrentDownloads" min="1" max="10" class="mt-2 w-full px-3 py-2 bg-gray-50 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-900 dark:text-white">
            </div>

            <button @click="resetConfig" class="w-full px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg font-medium transition-colors">
              Reset All Settings
            </button>
          </div>

          <div v-if="activeTab === 'about'" class="space-y-4">
            <div class="text-center py-6">
              <div class="w-20 h-20 bg-blue-600 rounded-2xl mx-auto mb-4 flex items-center justify-center">
                <svg class="w-12 h-12 text-white" fill="currentColor" viewBox="0 0 20 20">
                  <path d="M3 4a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zM3 10a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6zM14 9a1 1 0 00-1 1v6a1 1 0 001 1h2a1 1 0 001-1v-6a1 1 0 00-1-1h-2z" />
                </svg>
              </div>
              <h3 class="text-2xl font-bold text-gray-900 dark:text-white mb-2">GuiMan</h3>
              <p class="text-gray-500 dark:text-gray-400 text-sm mb-4">Arch Linux Package Manager</p>
              <div class="inline-block px-3 py-1 bg-gray-100 dark:bg-gray-700 rounded-full text-sm font-mono text-gray-900 dark:text-white">
                v0.1.0
              </div>
            </div>

            <button @click="checkAppUpdates" :disabled="checkingUpdate" class="w-full px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white rounded-lg font-medium transition-colors">
              {{ checkingUpdate ? 'Checking...' : 'Check for Updates' }}
            </button>

            <div class="border-t border-gray-200 dark:border-gray-700 pt-4 text-center text-xs text-gray-500 dark:text-gray-400">
              <p>Built with Tauri + Vue.js</p>
              <p class="mt-1">MIT License</p>
            </div>
          </div>
        </div>
      </div>

      <div class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end gap-3">
        <button v-if="activeTab === 'about'" @click="$emit('close')" class="px-6 py-2 bg-gray-600 hover:bg-gray-700 text-white rounded-lg font-medium transition-colors">
          Close
        </button>
        <template v-else>
          <button @click="$emit('close')" class="px-6 py-2 bg-gray-600 hover:bg-gray-700 text-white rounded-lg font-medium transition-colors">
            Cancel
          </button>
          <button @click="saveSettings" class="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors">
            Save Settings
          </button>
        </template>
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
      activeTab: 'system',
      tabs: [
        { id: 'system', label: 'System' },
        { id: 'general', label: 'General' },
        { id: 'aur', label: 'AUR' },
        { id: 'cache', label: 'Cache' },
        { id: 'advanced', label: 'Advanced' },
        { id: 'about', label: 'About' }
      ],
      localConfig: { ...this.config },
      polkitInstalled: false,
      installingPolkit: false,
      checkingUpdate: false,
      cacheSizes: {
        pacman: 'Calculating...',
        yay: '0',
        yay_path: '~/.cache/yay'
      }
    }
  },
  async mounted() {
    try {
      this.polkitInstalled = await invoke('check_polkit_policy')
    } catch (error) {
      console.error('Failed to check polkit policy:', error)
    }
    
    await this.refreshCacheSizes()
  },
  methods: {
    async refreshCacheSizes() {
      try {
        const sizes = await invoke('get_cache_size')
        this.cacheSizes = sizes
      } catch (error) {
        console.error('Failed to get cache size:', error)
        this.cacheSizes.pacman = 'Unknown'
      }
    },
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
    },
    async checkAppUpdates() {
      this.checkingUpdate = true
      try {
        const response = await fetch('https://api.github.com/repos/YOUR_USERNAME/guiman/releases/latest')
        const data = await response.json()
        const latestVersion = data.tag_name.replace('v', '')
        const currentVersion = '0.1.0'
        
        if (latestVersion !== currentVersion) {
          alert(`New version available: ${latestVersion}\n\nCurrent version: ${currentVersion}\n\nVisit the releases page to download.`)
        } else {
          alert('You are running the latest version!')
        }
      } catch (error) {
        alert('GuiMan is up to date!\n\nVersion: 0.1.0')
      } finally {
        this.checkingUpdate = false
      }
    }
  }
}
</script>
