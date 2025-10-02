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

          <div v-if="activeTab === 'mirrors'" class="space-y-4">
            <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4 mb-4">
              <div class="flex items-start gap-3">
                <svg class="w-5 h-5 text-blue-600 dark:text-blue-400 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M4.083 9h1.946c.089-1.546.383-2.97.837-4.118A6.004 6.004 0 004.083 9zM10 2a8 8 0 100 16 8 8 0 000-16zm0 2c-.076 0-.232.032-.465.262-.238.234-.497.623-.737 1.182-.389.907-.673 2.142-.766 3.556h3.936c-.093-1.414-.377-2.649-.766-3.556-.24-.56-.5-.948-.737-1.182C10.232 4.032 10.076 4 10 4zm3.971 5c-.089-1.546-.383-2.97-.837-4.118A6.004 6.004 0 0115.917 9h-1.946zm-2.003 2H8.032c.093 1.414.377 2.649.766 3.556.24.56.5.948.737 1.182.233.23.389.262.465.262.076 0 .232-.032.465-.262.238-.234.498-.623.737-1.182.389-.907.673-2.142.766-3.556zm1.166 4.118c.454-1.147.748-2.572.837-4.118h1.946a6.004 6.004 0 01-2.783 4.118zm-6.268 0C6.412 13.97 6.118 12.546 6.03 11H4.083a6.004 6.004 0 002.783 4.118z" clip-rule="evenodd" />
                </svg>
                <div class="flex-1">
                  <h4 class="font-semibold text-gray-900 dark:text-white">Mirror Management</h4>
                  <p class="text-xs text-gray-600 dark:text-gray-300 mt-1">
                    Configure pacman mirrors for faster downloads
                  </p>
                </div>
              </div>
            </div>

            <div class="flex justify-between items-center mb-4">
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Pacman Mirrors</h3>
              <div class="flex gap-2">
                <button @click="loadMirrors" :disabled="loadingMirrors" class="px-3 py-1.5 bg-gray-600 hover:bg-gray-700 disabled:opacity-50 text-white text-sm rounded transition-colors">
                  <span v-if="loadingMirrors">Loading...</span>
                  <span v-else>Refresh</span>
                </button>
                <button @click="rankMirrors" :disabled="rankingMirrors || mirrors.length === 0" class="px-3 py-1.5 bg-green-600 hover:bg-green-700 disabled:opacity-50 text-white text-sm rounded transition-colors" title="Requires reflector package">
                  <span v-if="rankingMirrors">Ranking...</span>
                  <span v-else>Auto-Rank</span>
                </button>
                <button @click="saveMirrors" :disabled="savingMirrors || mirrors.length === 0" class="px-3 py-1.5 bg-blue-600 hover:bg-blue-700 disabled:opacity-50 text-white text-sm rounded transition-colors">
                  <span v-if="savingMirrors">Saving...</span>
                  <span v-else>Save Changes</span>
                </button>
              </div>
            </div>

            <div v-if="loadingMirrors" class="text-center py-8">
              <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
              <p class="text-gray-600 dark:text-gray-400 mt-2">Loading mirrors...</p>
            </div>

            <div v-else-if="mirrors.length === 0" class="text-center py-8">
              <svg class="w-12 h-12 text-gray-400 mx-auto mb-4" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
              </svg>
              <p class="text-gray-600 dark:text-gray-400">No mirrors loaded</p>
              <button @click="loadMirrors" class="mt-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white text-sm rounded transition-colors">
                Load Mirrors
              </button>
            </div>

            <div v-else class="space-y-2 max-h-96 overflow-auto">
              <div v-for="(mirror, index) in mirrors" :key="mirror.url" 
                   class="flex items-center gap-3 p-3 bg-white dark:bg-gray-700 border border-gray-200 dark:border-gray-600 rounded-lg">
                <div class="flex items-center gap-2">
                  <button @click="moveMirror(index, -1)" :disabled="index === 0" 
                          class="p-1 text-gray-400 hover:text-gray-600 disabled:opacity-30">
                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M14.707 12.707a1 1 0 01-1.414 0L10 9.414l-3.293 3.293a1 1 0 01-1.414-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 010 1.414z" clip-rule="evenodd" />
                    </svg>
                  </button>
                  <button @click="moveMirror(index, 1)" :disabled="index === mirrors.length - 1" 
                          class="p-1 text-gray-400 hover:text-gray-600 disabled:opacity-30">
                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
                    </svg>
                  </button>
                </div>
                
                <div class="flex items-center">
                  <input type="checkbox" v-model="mirror.enabled" 
                         class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600">
                </div>
                
                <div class="flex-1 min-w-0">
                  <div class="flex items-center gap-2 mb-1">
                    <span class="text-xs font-medium px-2 py-1 bg-gray-100 dark:bg-gray-600 text-gray-700 dark:text-gray-300 rounded">
                      {{ mirror.country }}
                    </span>
                    <span v-if="mirror.enabled" class="text-xs font-medium px-2 py-1 bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-300 rounded">
                      Enabled
                    </span>
                    <span v-else class="text-xs font-medium px-2 py-1 bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-300 rounded">
                      Disabled
                    </span>
                  </div>
                  <p class="text-sm text-gray-600 dark:text-gray-400 font-mono truncate">{{ mirror.url }}</p>
                </div>
              </div>
            </div>

            <div class="mt-6 p-4 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg">
              <div class="flex items-start gap-3">
                <svg class="w-5 h-5 text-yellow-600 dark:text-yellow-400 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
                </svg>
                <div class="flex-1">
                  <h4 class="font-semibold text-gray-900 dark:text-white">Mirror Tips</h4>
                  <ul class="text-xs text-gray-600 dark:text-gray-300 mt-2 space-y-1">
                    <li>• Mirrors higher in the list have priority</li>
                    <li>• Use "Auto-Rank" to automatically sort by speed (requires reflector)</li>
                    <li>• Disable slow or unreliable mirrors</li>
                    <li>• Changes require root permission to save</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>

          <div v-if="activeTab === 'advanced'" class="space-y-4">
            <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-3 mb-4">
              <p class="text-xs text-blue-800 dark:text-blue-200">⚙️ Advanced pacman configuration</p>
            </div>

            <div>
              <label class="text-sm font-medium text-gray-900 dark:text-white">Max concurrent downloads</label>
              <input type="number" v-model.number="localConfig.maxConcurrentDownloads" min="1" max="10" class="mt-2 w-full px-3 py-2 bg-gray-50 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-900 dark:text-white">
              <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">Number of parallel package downloads</p>
            </div>

            <div class="flex items-center justify-between py-3 border-b border-gray-200 dark:border-gray-700">
              <div>
                <div class="text-sm font-medium text-gray-900 dark:text-white">Download timeout</div>
                <div class="text-xs text-gray-500 dark:text-gray-400">Seconds before timing out</div>
              </div>
              <input type="number" v-model.number="localConfig.downloadTimeout" min="30" max="600" class="w-24 px-3 py-1 bg-gray-50 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded text-gray-900 dark:text-white text-sm">
            </div>

            <div class="flex items-center justify-between py-3 border-b border-gray-200 dark:border-gray-700">
              <div>
                <div class="text-sm font-medium text-gray-900 dark:text-white">Check disk space</div>
                <div class="text-xs text-gray-500 dark:text-gray-400">Verify before installation</div>
              </div>
              <label class="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" v-model="localConfig.checkDiskSpace" class="sr-only peer">
                <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
              </label>
            </div>

            <div class="flex items-center justify-between py-3 border-b border-gray-200 dark:border-gray-700">
              <div>
                <div class="text-sm font-medium text-gray-900 dark:text-white">Verify packages</div>
                <div class="text-xs text-gray-500 dark:text-gray-400">Check package signatures</div>
              </div>
              <label class="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" v-model="localConfig.verifyPackages" class="sr-only peer">
                <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
              </label>
            </div>

            <div class="flex items-center justify-between py-3 border-b border-gray-200 dark:border-gray-700">
              <div>
                <div class="text-sm font-medium text-gray-900 dark:text-white">Color output</div>
                <div class="text-xs text-gray-500 dark:text-gray-400">Colorize terminal output</div>
              </div>
              <label class="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" v-model="localConfig.colorOutput" class="sr-only peer">
                <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
              </label>
            </div>

            <div class="flex items-center justify-between py-3 border-b border-gray-200 dark:border-gray-700">
              <div>
                <div class="text-sm font-medium text-gray-900 dark:text-white">Verbose output</div>
                <div class="text-xs text-gray-500 dark:text-gray-400">Show detailed information</div>
              </div>
              <label class="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" v-model="localConfig.verboseOutput" class="sr-only peer">
                <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
              </label>
            </div>

            <button @click="syncDatabases" class="w-full px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg font-medium transition-colors">
              Sync Package Databases
            </button>

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
              <p class="text-gray-500 dark:text-gray-400 text-xs mt-2">
                <a href="https://github.com/Junaid433" target="_blank" class="text-blue-600 hover:text-blue-700">By Junaid Rahman</a>
              </p>
            </div>

            <div class="space-y-3">
              <button @click="checkAppUpdates" :disabled="checkingUpdate || installingUpdate" class="w-full px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white rounded-lg font-medium transition-colors">
                {{ checkingUpdate ? 'Checking...' : installingUpdate ? 'Installing...' : 'Check for Updates' }}
              </button>
              
              <!-- Update Available -->
              <div v-if="updateInfo && updateInfo.update_available" class="bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg p-4">
                <div class="flex items-center justify-between mb-3">
                  <div>
                    <h4 class="font-semibold text-green-900 dark:text-green-100">Update Available!</h4>
                    <p class="text-sm text-green-700 dark:text-green-300">
                      {{ updateInfo.current_version }} → {{ updateInfo.latest_version }}
                    </p>
                  </div>
                  <button 
                    @click="installUpdate"
                    :disabled="installingUpdate"
                    class="px-4 py-2 bg-green-600 hover:bg-green-700 disabled:bg-green-400 text-white rounded-lg font-medium transition-colors"
                  >
                    {{ installingUpdate ? 'Installing...' : 'Install Now' }}
                  </button>
                </div>
                
                <!-- Update Progress -->
                <div v-if="updateProgress" class="space-y-2">
                  <div class="flex justify-between text-sm">
                    <span class="text-green-700 dark:text-green-300">{{ updateProgress.message }}</span>
                    <span class="text-green-600 dark:text-green-400">{{ updateProgress.progress }}%</span>
                  </div>
                  <div class="w-full bg-green-200 dark:bg-green-800 rounded-full h-2">
                    <div 
                      class="bg-green-600 h-2 rounded-full transition-all duration-300"
                      :style="{ width: updateProgress.progress + '%' }"
                    ></div>
                  </div>
                </div>
                
                <!-- Changelog -->
                <div v-if="updateInfo.changelog" class="mt-3 pt-3 border-t border-green-200 dark:border-green-700">
                  <h5 class="text-sm font-medium text-green-900 dark:text-green-100 mb-2">What's New:</h5>
                  <div class="text-xs text-green-700 dark:text-green-300 max-h-20 overflow-y-auto whitespace-pre-line">
                    {{ updateInfo.changelog }}
                  </div>
                </div>
              </div>
              
              <!-- Auto-update Setting -->
              <div class="bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-4">
                <label class="flex items-center justify-between">
                  <div>
                    <span class="text-sm font-medium text-gray-900 dark:text-white">Automatic Updates</span>
                    <p class="text-xs text-gray-500 dark:text-gray-400">Install updates automatically in the background</p>
                  </div>
                  <input 
                    v-model="autoUpdateEnabled"
                    @change="toggleAutoUpdate"
                    type="checkbox" 
                    class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                  >
                </label>
              </div>
            </div>

            <div class="border-t border-gray-200 dark:border-gray-700 pt-4 text-center text-xs text-gray-500 dark:text-gray-400">
              <p>Built with Tauri + Vue.js + Rust</p>
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
import { listen } from '@tauri-apps/api/event'

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
        { id: 'mirrors', label: 'Mirrors' },
        { id: 'cache', label: 'Cache' },
        { id: 'advanced', label: 'Advanced' },
        { id: 'about', label: 'About' }
      ],
      localConfig: { ...this.config },
      polkitInstalled: false,
      installingPolkit: false,
      mirrors: [],
      loadingMirrors: false,
      savingMirrors: false,
      rankingMirrors: false,
      checkingUpdate: false,
      installingUpdate: false,
      updateInfo: null,
      updateProgress: null,
      autoUpdateEnabled: true,
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
    await this.loadAutoUpdateSetting()
  },
  methods: {
    async loadMirrors() {
      this.loadingMirrors = true
      try {
        this.mirrors = await invoke('get_mirrorlist_info')
      } catch (error) {
        console.error('Failed to load mirrors:', error)
        alert('Failed to load mirrors: ' + error)
      } finally {
        this.loadingMirrors = false
      }
    },
    
    async saveMirrors() {
      this.savingMirrors = true
      try {
        const result = await invoke('update_mirrorlist', { mirrors: this.mirrors })
        alert('✓ ' + result)
      } catch (error) {
        console.error('Failed to save mirrors:', error)
        alert('✗ Failed to save mirrors: ' + error)
      } finally {
        this.savingMirrors = false
      }
    },
    
    async rankMirrors() {
      this.rankingMirrors = true
      try {
        const result = await invoke('rank_mirrors', { 
          country: null, // Use all countries
          count: 20 // Top 20 mirrors
        })
        alert('✓ ' + result)
        // Reload mirrors to show new ranking
        await this.loadMirrors()
      } catch (error) {
        console.error('Failed to rank mirrors:', error)
        
        // Check if it's a reflector installation issue
        if (error.includes('reflector is not installed')) {
          const install = confirm(
            '❌ Reflector is not installed.\n\n' +
            'Reflector is needed to automatically rank mirrors by speed.\n' +
            'Would you like to install it now?\n\n' +
            'This will run: sudo pacman -S reflector'
          )
          
          if (install) {
            try {
              await this.installReflector()
            } catch (installError) {
              alert('✗ Failed to install reflector: ' + installError)
            }
          }
        } else {
          alert('✗ Failed to rank mirrors: ' + error)
        }
      } finally {
        this.rankingMirrors = false
      }
    },
    
    async installReflector() {
      try {
        const result = await invoke('install_package', { 
          packageName: 'reflector',
          useAur: false 
        })
        alert('✓ Reflector installed successfully! You can now use Auto-Rank.')
      } catch (error) {
        throw error
      }
    },
    
    moveMirror(index, direction) {
      const newIndex = index + direction
      if (newIndex < 0 || newIndex >= this.mirrors.length) return
      
      const mirrors = [...this.mirrors]
      const temp = mirrors[index]
      mirrors[index] = mirrors[newIndex]
      mirrors[newIndex] = temp
      this.mirrors = mirrors
    },
    
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
    async syncDatabases() {
      try {
        const result = await invoke('sync_databases')
        alert('✓ ' + result)
      } catch (error) {
        alert('✗ Failed to sync databases:\n' + error)
      }
    },
    resetConfig() {
      if (confirm('Are you sure you want to reset all settings to defaults?')) {
        this.$emit('save', null)
        this.$emit('close')
      }
    },
    async checkAppUpdates() {
      this.checkingUpdate = true
      this.updateInfo = null
      try {
        this.updateInfo = await invoke('check_app_updates')
        if (!this.updateInfo.update_available) {
          alert('✓ You are running the latest version!')
        }
      } catch (error) {
        console.error('Failed to check for updates:', error)
        alert('✗ Failed to check for updates: ' + error)
      } finally {
        this.checkingUpdate = false
      }
    },

    async installUpdate() {
      this.installingUpdate = true
      this.updateProgress = null
      
      try {
        // Listen for update progress
        const unlisten = await listen('update-progress', (event) => {
          this.updateProgress = event.payload
          if (event.payload.completed) {
            this.installingUpdate = false
            if (event.payload.error) {
              alert('✗ Update failed: ' + event.payload.error)
            } else {
              // Show restart dialog
              if (confirm('✓ Update installed successfully! Restart now?')) {
                invoke('restart_application')
              }
            }
          }
        })
        
        await invoke('install_app_update')
        
        // Clean up listener
        unlisten()
      } catch (error) {
        console.error('Failed to install update:', error)
        alert('✗ Failed to install update: ' + error)
        this.installingUpdate = false
      }
    },

    async toggleAutoUpdate() {
      try {
        await invoke('set_auto_update', { enabled: this.autoUpdateEnabled })
      } catch (error) {
        console.error('Failed to set auto-update:', error)
        // Revert the checkbox
        this.autoUpdateEnabled = !this.autoUpdateEnabled
      }
    },

    async loadAutoUpdateSetting() {
      try {
        this.autoUpdateEnabled = await invoke('get_auto_update_setting')
      } catch (error) {
        console.error('Failed to get auto-update setting:', error)
      }
    }
  }
}
</script>
