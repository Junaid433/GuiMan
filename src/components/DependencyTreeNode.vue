<template>
  <div class="dependency-node">
    <!-- Current Node -->
    <div class="flex items-center gap-3 mb-2">
      <!-- Level Indicator -->
      <div class="flex items-center">
        <div v-for="i in node.level" :key="i" class="w-6 h-px bg-gray-300 dark:bg-gray-600"></div>
        <div v-if="node.level > 0" class="w-2 h-2 bg-gray-400 rounded-full"></div>
      </div>
      
      <!-- Node Content -->
      <div 
        @click="$emit('node-click', node.name)"
        :class="[
          'flex items-center gap-3 p-3 rounded-lg border cursor-pointer transition-all hover:shadow-md',
          isRoot 
            ? 'bg-blue-50 dark:bg-blue-900/20 border-blue-200 dark:border-blue-800' 
            : node.installed 
              ? 'bg-green-50 dark:bg-green-900/20 border-green-200 dark:border-green-800' 
              : 'bg-orange-50 dark:bg-orange-900/20 border-orange-200 dark:border-orange-800'
        ]"
      >
        <!-- Status Indicator -->
        <div 
          :class="[
            'w-3 h-3 rounded-full',
            isRoot 
              ? 'bg-blue-500' 
              : node.installed 
                ? 'bg-green-500' 
                : 'bg-orange-500'
          ]"
        ></div>
        
        <!-- Package Info -->
        <div class="flex-1 min-w-0">
          <div class="flex items-center gap-2">
            <h4 class="font-semibold text-gray-900 dark:text-white truncate">{{ node.name }}</h4>
            <span class="text-sm text-gray-500 dark:text-gray-400">{{ node.version }}</span>
            <span 
              :class="[
                'text-xs px-2 py-1 rounded-full',
                getRepoColor(node.repo)
              ]"
            >
              {{ node.repo }}
            </span>
          </div>
          
          <div class="flex items-center gap-4 mt-1 text-xs text-gray-600 dark:text-gray-400">
            <span v-if="node.dependencies.length > 0">
              📦 {{ node.dependencies.length }} deps
            </span>
            <span v-if="node.optional_deps.length > 0">
              📋 {{ node.optional_deps.length }} optional
            </span>
            <span v-if="node.required_by.length > 0">
              🔗 {{ node.required_by.length }} required by
            </span>
            <span v-if="!node.installed" class="text-orange-600 dark:text-orange-400 font-medium">
              Not Installed
            </span>
          </div>
        </div>
        
        <!-- Expand/Collapse Button -->
        <button 
          v-if="hasChildren"
          @click.stop="expanded = !expanded"
          class="p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors"
        >
          <svg 
            :class="['w-4 h-4 transition-transform', expanded ? 'rotate-90' : '']" 
            fill="currentColor" 
            viewBox="0 0 20 20"
          >
            <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd" />
          </svg>
        </button>
      </div>
    </div>
    
    <!-- Children Nodes -->
    <div v-if="expanded && hasChildren" class="ml-6 space-y-1">
      <DependencyTreeNode
        v-for="childName in childrenList"
        :key="childName"
        :node="graph.nodes[childName]"
        :graph="graph"
        :view-mode="viewMode"
        :is-root="false"
        @node-click="$emit('node-click', $event)"
      />
    </div>
  </div>
</template>

<script>
export default {
  name: 'DependencyTreeNode',
  props: {
    node: {
      type: Object,
      required: true
    },
    graph: {
      type: Object,
      required: true
    },
    viewMode: {
      type: String,
      default: 'dependencies'
    },
    isRoot: {
      type: Boolean,
      default: false
    }
  },
  emits: ['node-click'],
  data() {
    return {
      expanded: true // Start expanded for better UX
    }
  },
  computed: {
    hasChildren() {
      return this.childrenList.length > 0
    },
    
    childrenList() {
      if (this.viewMode === 'dependencies') {
        const children = this.node.dependencies
          .map(dep => this.cleanDependencyName(dep))
          .filter(dep => this.graph.nodes[dep])
        // console.log(`Children for ${this.node.name}:`, children)
        return children
      } else {
        const children = this.node.required_by.filter(req => this.graph.nodes[req])
        // console.log(`Required by for ${this.node.name}:`, children)
        return children
      }
    }
  },
  methods: {
    cleanDependencyName(dep) {
      // Remove version constraints like >=, <=, =, etc.
      return dep.split(/[><>=]/)[0].trim()
    },
    
    getRepoColor(repo) {
      const colors = {
        'core': 'bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-300',
        'extra': 'bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-300',
        'community': 'bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-300',
        'multilib': 'bg-purple-100 dark:bg-purple-900 text-purple-700 dark:text-purple-300',
        'aur': 'bg-orange-100 dark:bg-orange-900 text-orange-700 dark:text-orange-300',
        'local': 'bg-gray-100 dark:bg-gray-900 text-gray-700 dark:text-gray-300'
      }
      return colors[repo.toLowerCase()] || 'bg-gray-100 dark:bg-gray-900 text-gray-700 dark:text-gray-300'
    }
  }
}
</script>

<style scoped>
.dependency-node {
  font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', monospace;
}
</style>
