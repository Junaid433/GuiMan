<template>
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl w-full max-w-2xl max-h-[80vh] flex flex-col">
      <div class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700">
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white">{{ packageInfo.name }}</h2>
        <button @click="$emit('close')" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200">
          <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </button>
      </div>

      <div class="flex-1 overflow-auto p-6 scrollbar-thin space-y-4">
        <!-- Group View -->
        <div v-if="packageInfo.repo === 'group'">
          <div class="mb-4">
            <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">Group Summary</label>
            <p class="text-gray-900 dark:text-white mt-1">{{ details.size }}</p>
            <p class="text-gray-600 dark:text-gray-400 text-sm mt-1">{{ packageInfo.description }}</p>
          </div>
          
          <div class="border-t border-gray-200 dark:border-gray-700 pt-4">
            <label class="text-sm font-semibold text-gray-500 dark:text-gray-400 mb-3 block">Packages in Group</label>
            <div class="grid grid-cols-2 gap-2 max-h-96 overflow-auto">
              <div v-for="pkg in details.dependencies" :key="pkg" 
                   class="px-3 py-2 bg-gray-50 dark:bg-gray-700 text-gray-700 dark:text-gray-300 text-sm rounded border border-gray-200 dark:border-gray-600">
                {{ pkg }}
              </div>
            </div>
          </div>
        </div>

        <!-- Files/File Owner View -->
        <div v-else-if="packageInfo.repo === 'file' || packageInfo.repo === 'file-owner'">
          <div class="mb-4">
            <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">Package</label>
            <p class="text-gray-900 dark:text-white mt-1">{{ packageInfo.name }}</p>
          </div>
          
          <div v-if="packageInfo.repo === 'file-owner'" class="mb-4">
            <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">File Path</label>
            <p class="text-gray-700 dark:text-gray-300 mt-1 font-mono text-sm bg-gray-50 dark:bg-gray-700 px-3 py-2 rounded">
              {{ packageInfo.version }}
            </p>
          </div>
          
          <div class="border-t border-gray-200 dark:border-gray-700 pt-4">
            <label class="text-sm font-semibold text-gray-500 dark:text-gray-400 mb-3 block">
              Files ({{ details.size }})
            </label>
            <div class="max-h-96 overflow-auto space-y-1">
              <div v-for="file in details.dependencies" :key="file" 
                   class="px-3 py-1.5 bg-gray-50 dark:bg-gray-700 text-gray-700 dark:text-gray-300 text-xs rounded font-mono">
                {{ file }}
              </div>
            </div>
          </div>
        </div>

        <!-- Repository View -->
        <div v-else-if="packageInfo.repo === 'repository'">
          <div class="mb-4">
            <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">Repository Info</label>
            <p class="text-gray-900 dark:text-white mt-1 font-semibold">{{ packageInfo.name }}</p>
            <p class="text-gray-600 dark:text-gray-400 text-sm mt-1">{{ details.size }}</p>
          </div>
          
          <div class="border-t border-gray-200 dark:border-gray-700 pt-4">
            <label class="text-sm font-semibold text-gray-500 dark:text-gray-400 mb-2 block">Mirror URLs</label>
            <div class="space-y-2">
              <div v-for="(url, idx) in details.url.split(', ')" :key="idx" 
                   class="px-3 py-2 bg-gray-50 dark:bg-gray-700 rounded">
                <a :href="url" target="_blank" class="text-blue-600 dark:text-blue-400 hover:underline text-sm break-all">
                  {{ url }}
                </a>
              </div>
            </div>
          </div>
        </div>

        <!-- Regular Package View -->
        <div v-else>
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">Version</label>
              <p class="text-gray-900 dark:text-white">{{ packageInfo.version }}</p>
            </div>
            <div>
              <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">Repository</label>
              <p class="text-gray-900 dark:text-white">{{ packageInfo.repo }}</p>
            </div>
            <div>
              <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">Status</label>
              <p :class="getStatusClass()">
                {{ getStatusText() }}
              </p>
            </div>
            <div>
              <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">Size</label>
              <p class="text-gray-900 dark:text-white">{{ details.size || 'Unknown' }}</p>
            </div>
          </div>

          <div v-if="packageInfo.description" class="border-t border-gray-200 dark:border-gray-700 pt-4">
            <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">Description</label>
            <p class="mt-1 text-gray-900 dark:text-white">{{ packageInfo.description }}</p>
          </div>

          <div v-if="details.url" class="border-t border-gray-200 dark:border-gray-700 pt-4">
            <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">URL</label>
            <a :href="details.url" target="_blank" class="mt-1 block text-blue-600 dark:text-blue-400 hover:underline">
              {{ details.url }}
            </a>
          </div>

          <div v-if="details.licenses && details.licenses.length" class="border-t border-gray-200 dark:border-gray-700 pt-4">
            <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">Licenses</label>
            <p class="mt-1 text-gray-900 dark:text-white">{{ details.licenses.join(', ') }}</p>
          </div>

          <div v-if="details.dependencies && details.dependencies.length" class="border-t border-gray-200 dark:border-gray-700 pt-4">
            <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">Dependencies</label>
            <div class="mt-2 flex flex-wrap gap-2">
              <span v-for="dep in details.dependencies" :key="dep" class="px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 text-xs rounded">
                {{ dep }}
              </span>
            </div>
          </div>

          <div v-if="details.optionalDeps && details.optionalDeps.length" class="border-t border-gray-200 dark:border-gray-700 pt-4">
            <label class="text-sm font-semibold text-gray-500 dark:text-gray-400">Optional Dependencies</label>
            <div class="mt-2 flex flex-wrap gap-2">
              <span v-for="dep in details.optionalDeps" :key="dep" class="px-2 py-1 bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-300 text-xs rounded">
                {{ dep }}
              </span>
            </div>
          </div>
        </div>
      </div>

      <div class="p-6 border-t border-gray-200 dark:border-gray-700 flex justify-between gap-3">
        <div class="flex gap-3">
          <button @click="$emit('close')" class="px-6 py-2 bg-gray-600 hover:bg-gray-700 text-white rounded-lg font-medium transition-colors">
            Close
          </button>
          <button 
            v-if="!['group', 'file', 'file-owner', 'repository', 'log'].includes(packageInfo.repo)"
            @click="showDependencyGraph" 
            class="px-6 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded-lg font-medium transition-colors flex items-center gap-2"
          >
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M12.316 3.051a1 1 0 01.633 1.265l-4 12a1 1 0 11-1.898-.632l4-12a1 1 0 011.265-.633zM5.707 6.293a1 1 0 010 1.414L3.414 10l2.293 2.293a1 1 0 11-1.414 1.414l-3-3a1 1 0 010-1.414l3-3a1 1 0 011.414 0zm8.586 0a1 1 0 011.414 0l3 3a1 1 0 010 1.414l-3 3a1 1 0 11-1.414-1.414L16.586 10l-2.293-2.293a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
            Dependencies
          </button>
        </div>
        <!-- Only show action buttons for regular packages, not for groups/files/repos/history -->
        <div v-if="!['group', 'file', 'file-owner', 'repository', 'log'].includes(packageInfo.repo)" class="flex gap-3">
          <!-- AUR Package Actions -->
          <div v-if="packageInfo.repo === 'aur'" class="flex gap-2">
            <button v-if="isUpdatePackage()" @click="$emit('install', packageInfo)" class="px-4 py-2 bg-orange-600 hover:bg-orange-700 text-white text-sm rounded-lg font-medium transition-colors">
              Update
            </button>
            <button v-else-if="!packageInfo.installed" @click="$emit('install', packageInfo)" class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white text-sm rounded-lg font-medium transition-colors">
              Install
            </button>
            <button v-else @click="$emit('remove', packageInfo)" class="px-4 py-2 bg-red-600 hover:bg-red-700 text-white text-sm rounded-lg font-medium transition-colors">
              Remove
            </button>
            
            <!-- AUR Advanced Actions -->
            <div class="relative">
              <button @click="showAurActions = !showAurActions" class="px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white text-sm rounded-lg font-medium transition-colors flex items-center gap-1">
                AUR
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
                </svg>
              </button>
              
              <div v-if="showAurActions" class="absolute right-0 bottom-full mb-2 w-48 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-lg z-10">
                <button @click="votePackage" class="w-full px-4 py-2 text-left text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-t-lg">
                  👍 Vote
                </button>
                <button @click="flagPackage" class="w-full px-4 py-2 text-left text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700">
                  🚩 Flag Out-of-Date
                </button>
                <button @click="adoptPackage" class="w-full px-4 py-2 text-left text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700">
                  🤝 Adopt Package
                </button>
                <button @click="showBuildOptions" class="w-full px-4 py-2 text-left text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-b-lg">
                  ⚙️ Build Options
                </button>
              </div>
            </div>
          </div>
          
          <!-- Regular Package Actions -->
          <div v-else class="flex gap-3">
            <button v-if="isUpdatePackage()" @click="$emit('install', packageInfo)" class="px-6 py-2 bg-orange-600 hover:bg-orange-700 text-white rounded-lg font-medium transition-colors">
              Update
            </button>
            <button v-else-if="!packageInfo.installed" @click="$emit('install', packageInfo)" class="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors">
              Install
            </button>
            <button v-else @click="$emit('remove', packageInfo)" class="px-6 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg font-medium transition-colors">
              Remove
            </button>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Build Options Modal -->
    <div v-if="showBuildOptionsModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-60 p-4">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl w-full max-w-md">
        <div class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-lg font-bold text-gray-900 dark:text-white">Build Options</h3>
          <button @click="showBuildOptionsModal = false" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200">
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
          </button>
        </div>
        
        <div class="p-4 max-h-96 overflow-auto">
          <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
            Configure build options for {{ packageInfo.name }}
          </p>
          
          <div class="space-y-3">
            <div v-for="option in buildOptions" :key="option.name" class="flex items-start gap-3">
              <input 
                type="checkbox" 
                v-model="option.enabled" 
                :id="option.name"
                class="mt-1 w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
              >
              <div class="flex-1">
                <label :for="option.name" class="text-sm font-medium text-gray-900 dark:text-white cursor-pointer">
                  {{ option.name }}
                </label>
                <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                  {{ option.description }}
                </p>
              </div>
            </div>
          </div>
        </div>
        
        <div class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-between gap-3">
          <button @click="showBuildOptionsModal = false" class="px-4 py-2 bg-gray-600 hover:bg-gray-700 text-white rounded-lg font-medium transition-colors">
            Cancel
          </button>
          <button @click="installWithOptions" class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors">
            Install with Options
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core'

export default {
  name: 'PackageDetailsModal',
  props: {
    packageInfo: Object,
    details: {
      type: Object,
      default: () => ({})
    }
  },
  data() {
    return {
      showAurActions: false,
      showBuildOptionsModal: false,
      buildOptions: []
    }
  },
  emits: ['close', 'install', 'remove', 'show-dependencies'],
  methods: {
    isUpdatePackage() {
      return this.packageInfo.description && this.packageInfo.description.includes('→')
    },
    
    async votePackage() {
      this.showAurActions = false
      try {
        const config = JSON.parse(localStorage.getItem('guiman-config') || '{}')
        const aurHelper = config.aurHelper || 'yay'
        await invoke('vote_aur_package', {
          package: this.packageInfo.name,
          helper: aurHelper
        })
      } catch (error) {
        // This will show the AUR URL for manual voting
        const confirmed = confirm(error + '\n\nOpen AUR page in browser?')
        if (confirmed) {
          window.open(`https://aur.archlinux.org/packages/${this.packageInfo.name}`, '_blank')
        }
      }
    },
    
    async flagPackage() {
      this.showAurActions = false
      const comment = prompt('Optional comment for flagging (leave empty for no comment):')
      try {
        const config = JSON.parse(localStorage.getItem('guiman-config') || '{}')
        const aurHelper = config.aurHelper || 'yay'
        await invoke('flag_aur_package', {
          package: this.packageInfo.name,
          helper: aurHelper,
          comment: comment || null
        })
      } catch (error) {
        const confirmed = confirm(error + '\n\nOpen AUR page in browser?')
        if (confirmed) {
          window.open(`https://aur.archlinux.org/packages/${this.packageInfo.name}`, '_blank')
        }
      }
    },
    
    async adoptPackage() {
      this.showAurActions = false
      const confirmed = confirm(
        `Adopt package "${this.packageInfo.name}"?\n\n` +
        'This requires an AUR account and maintainer privileges.\n' +
        'You will be redirected to the AUR page.'
      )
      if (confirmed) {
        try {
          const config = JSON.parse(localStorage.getItem('guiman-config') || '{}')
          const aurHelper = config.aurHelper || 'yay'
          await invoke('adopt_aur_package', {
            package: this.packageInfo.name,
            helper: aurHelper
          })
        } catch (error) {
          alert(error)
          window.open(`https://aur.archlinux.org/packages/${this.packageInfo.name}`, '_blank')
        }
      }
    },
    
    async showBuildOptions() {
      this.showAurActions = false
      try {
        const config = JSON.parse(localStorage.getItem('guiman-config') || '{}')
        const aurHelper = config.aurHelper || 'yay'
        const buildInfo = await invoke('get_aur_build_info', {
          package: this.packageInfo.name,
          helper: aurHelper
        })
        
        // Parse build info and show options
        const options = this.parseBuildOptions(buildInfo)
        this.buildOptions = options
        this.showBuildOptionsModal = true
      } catch (error) {
        alert('Failed to get build info: ' + error)
      }
    },
    
    parseBuildOptions(buildInfo) {
      const options = [
        { name: '--needed', description: 'Skip already installed dependencies', enabled: true },
        { name: '--noconfirm', description: 'Skip confirmation prompts', enabled: true },
        { name: '--clean', description: 'Clean build files after install', enabled: false },
        { name: '--nodeps', description: 'Skip dependency checks', enabled: false },
        { name: '--nocheck', description: 'Skip check() function', enabled: false },
        { name: '--nopgp', description: 'Skip PGP signature verification', enabled: false },
        { name: '--rebuild', description: 'Always rebuild package', enabled: false },
        { name: '--redownload', description: 'Always redownload sources', enabled: false }
      ]
      
      // Add conditional options based on build info
      if (buildInfo.includes('Check Dependencies') || buildInfo.includes('checkdepends')) {
        options.find(opt => opt.name === '--nocheck').description += ' (package has check dependencies)'
      }
      
      if (buildInfo.includes('PGP') || buildInfo.includes('validpgpkeys')) {
        options.find(opt => opt.name === '--nopgp').description += ' (package uses PGP verification)'
      }
      
      return options
    },
    
    async installWithOptions() {
      this.showBuildOptionsModal = false
      const enabledOptions = this.buildOptions
        .filter(opt => opt.enabled)
        .map(opt => opt.name)
      
      try {
        const config = JSON.parse(localStorage.getItem('guiman-config') || '{}')
        const aurHelper = config.aurHelper || 'yay'
        const result = await invoke('install_aur_with_options', {
          package: this.packageInfo.name,
          helper: aurHelper,
          options: enabledOptions
        })
        alert('✓ ' + result)
        this.$emit('close')
      } catch (error) {
        alert('✗ Failed to install with options: ' + error)
      }
    },
    
    showDependencyGraph() {
      console.log('Showing dependency graph for:', this.packageInfo.name, 'repo:', this.packageInfo.repo)
      this.$emit('show-dependencies', this.packageInfo.name)
    },
    
    getStatusText() {
      if (this.isUpdatePackage()) {
        return 'Update Required'
      }
      return this.packageInfo.installed ? 'Installed' : 'Not Installed'
    },
    getStatusClass() {
      if (this.isUpdatePackage()) {
        return 'text-orange-600 dark:text-orange-400'
      }
      return this.packageInfo.installed ? 'text-green-600 dark:text-green-400' : 'text-gray-600 dark:text-gray-400'
    }
  }
}
</script>

