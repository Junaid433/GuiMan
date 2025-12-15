<template>
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl w-full max-w-6xl max-h-[90vh] flex flex-col">
      <div class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700">
        <div class="flex items-center gap-4">
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white flex items-center gap-2">
            <svg class="w-7 h-7 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M12.316 3.051a1 1 0 01.633 1.265l-4 12a1 1 0 11-1.898-.632l4-12a1 1 0 011.265-.633zM5.707 6.293a1 1 0 010 1.414L3.414 10l2.293 2.293a1 1 0 11-1.414 1.414l-3-3a1 1 0 010-1.414l3-3a1 1 0 011.414 0zm8.586 0a1 1 0 011.414 0l3 3a1 1 0 010 1.414l-3 3a1 1 0 11-1.414-1.414L16.586 10l-2.293-2.293a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
            Dependency Graph: {{ packageName }}
          </h2>
          
          <div class="flex gap-2">
            <button 
              @click="viewMode = 'dependencies'"
              :class="[
                'px-3 py-1.5 text-sm rounded-lg font-medium transition-colors',
                viewMode === 'dependencies' 
                  ? 'bg-blue-600 text-white' 
                  : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600'
              ]"
            >
              Dependencies
            </button>
            <button 
              @click="viewMode = 'reverse'"
              :class="[
                'px-3 py-1.5 text-sm rounded-lg font-medium transition-colors',
                viewMode === 'reverse' 
                  ? 'bg-blue-600 text-white' 
                  : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600'
              ]"
            >
              Required By
            </button>
          </div>
        </div>
        
        <div class="flex items-center gap-3">
          <div class="flex items-center gap-2">
            <label class="text-sm text-gray-600 dark:text-gray-400">Depth:</label>
            <select v-model="maxDepth" @change="loadGraph" class="px-2 py-1 bg-gray-100 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded text-sm">
              <option value="1">1</option>
              <option value="2">2</option>
              <option value="3">3</option>
              <option value="4">4</option>
              <option value="5">5</option>
            </select>
          </div>
          
          <button @click="$emit('close')" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200">
            <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
          </button>
        </div>
      </div>

      <div class="flex-1 overflow-auto p-6">
        <div v-if="loading" class="flex items-center justify-center h-64">
          <div class="text-center">
            <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mb-4"></div>
            <p class="text-gray-600 dark:text-gray-400">Loading dependency graph...</p>
          </div>
        </div>
        
        <div v-else-if="error" class="flex items-center justify-center h-64">
          <div class="text-center">
            <svg class="w-16 h-16 text-red-400 mx-auto mb-4" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
            </svg>
            <p class="text-red-600 dark:text-red-400 font-medium">{{ error }}</p>
          </div>
        </div>
        
        <div v-else-if="graph" class="space-y-4">
          <!-- Graph Statistics -->
          <div class="bg-gray-50 dark:bg-gray-900 rounded-lg p-4 mb-6">
            <div class="grid grid-cols-4 gap-4 text-center">
              <div>
                <div class="text-2xl font-bold text-blue-600">{{ Object.keys(graph.nodes).length }}</div>
                <div class="text-sm text-gray-600 dark:text-gray-400">Total Packages</div>
              </div>
              <div>
                <div class="text-2xl font-bold text-green-600">{{ installedCount }}</div>
                <div class="text-sm text-gray-600 dark:text-gray-400">Installed</div>
              </div>
              <div>
                <div class="text-2xl font-bold text-orange-600">{{ notInstalledCount }}</div>
                <div class="text-sm text-gray-600 dark:text-gray-400">Not Installed</div>
              </div>
              <div>
                <div class="text-2xl font-bold text-purple-600">{{ graph.max_depth }}</div>
                <div class="text-sm text-gray-600 dark:text-gray-400">Max Depth</div>
              </div>
            </div>
          </div>
          
          <!-- Dependency Tree -->
          <div class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-4">
            <div class="mb-4 flex items-center justify-between">
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
                {{ viewMode === 'dependencies' ? 'Dependency Tree' : 'Reverse Dependency Tree' }}
              </h3>
              <div class="flex items-center gap-4 text-sm">
                <div class="flex items-center gap-2">
                  <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                  <span class="text-gray-600 dark:text-gray-400">Installed</span>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-3 h-3 bg-orange-500 rounded-full"></div>
                  <span class="text-gray-600 dark:text-gray-400">Not Installed</span>
                </div>
                <div class="flex items-center gap-2">
                  <div class="w-3 h-3 bg-blue-500 rounded-full"></div>
                  <span class="text-gray-600 dark:text-gray-400">Root Package</span>
                </div>
              </div>
            </div>
            
            <div class="overflow-x-auto">
              <div class="min-w-max">
                <DependencyTreeNode 
                  :node="graph.nodes[graph.root]"
                  :graph="graph"
                  :is-root="true"
                  :view-mode="viewMode"
                  @node-click="handleNodeClick"
                />
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core'
import DependencyTreeNode from './DependencyTreeNode.vue'

export default {
  name: 'DependencyGraph',
  components: {
    DependencyTreeNode
  },
  props: {
    packageName: {
      type: String,
      required: true
    }
  },
  emits: ['close', 'package-click'],
  data() {
    return {
      graph: null,
      loading: false,
      error: null,
      viewMode: 'dependencies', // 'dependencies' or 'reverse'
      maxDepth: 2
    }
  },
  computed: {
    installedCount() {
      if (!this.graph) return 0
      return Object.values(this.graph.nodes).filter(node => node.installed).length
    },
    notInstalledCount() {
      if (!this.graph) return 0
      return Object.values(this.graph.nodes).filter(node => !node.installed).length
    }
  },
  mounted() {
    this.loadGraph()
  },
  watch: {
    viewMode() {
      this.loadGraph()
    }
  },
  methods: {
    async loadGraph() {
      this.loading = true
      this.error = null
      
      try {
        // console.log(`Loading ${this.viewMode} graph for ${this.packageName} with depth ${this.maxDepth}`)
        if (this.viewMode === 'dependencies') {
          this.graph = await invoke('get_dependency_tree', {
            package: this.packageName,
            maxDepth: parseInt(this.maxDepth)
          })
        } else {
          this.graph = await invoke('get_reverse_dependency_tree', {
            package: this.packageName,
            maxDepth: parseInt(this.maxDepth)
          })
        }
        // console.log('Graph loaded successfully:', this.graph)
      } catch (error) {
        console.error('Failed to load dependency graph:', error)
        this.error = error.toString()
      } finally {
        this.loading = false
      }
    },
    
    handleNodeClick(packageName) {
      this.$emit('package-click', packageName)
    }
  }
}
</script>
