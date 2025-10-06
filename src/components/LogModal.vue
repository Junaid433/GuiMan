<template>
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-2xl w-full max-w-4xl max-h-[80vh] flex flex-col">
      <div class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700">
        <h2 class="text-xl font-bold text-gray-900 dark:text-white flex items-center gap-2">
          <svg v-if="!completed" class="w-6 h-6 text-blue-600 animate-spin" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          <svg v-else-if="success" class="w-6 h-6 text-green-600 dark:text-green-400" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
          </svg>
          <svg v-else class="w-6 h-6 text-red-600 dark:text-red-400" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
          </svg>
          {{ operation }}
        </h2>
        <button
          @click="$emit('close')"
          class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors"
        >
          <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </button>
      </div>

      <div class="flex-1 overflow-auto p-4 bg-[#0d1117] scrollbar-thin" ref="logContainer">
        <div class="font-mono text-sm leading-relaxed">
          <div v-if="logs.length === 0" class="text-gray-500 animate-pulse">
            <span class="text-purple-400">$</span> Waiting for output...
          </div>
          <div
            v-for="(log, index) in logs"
            :key="index"
            :class="getLogColor(log)"
            class="py-0.5"
          >
            {{ log }}
          </div>
          <div v-if="!completed" class="text-gray-500 mt-2 animate-pulse flex items-center gap-2">
            <div class="w-2 h-2 bg-purple-500 rounded-full animate-bounce"></div>
            <span>Processing...</span>
          </div>
        </div>
      </div>

      <div class="px-4 py-3 border-t border-gray-700 bg-[#161b22] flex justify-between items-center">
        <button
          @click="copyLogs"
          class="px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white text-sm rounded-lg font-medium transition-colors flex items-center gap-2"
        >
          <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
            <path d="M8 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z" />
            <path d="M6 3a2 2 0 00-2 2v11a2 2 0 002 2h8a2 2 0 002-2V5a2 2 0 00-2-2 3 3 0 01-3 3H9a3 3 0 01-3-3z" />
          </svg>
          Copy Logs
        </button>
        <button
          @click="$emit('close')"
          class="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors"
        >
          Close
        </button>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'LogModal',
  props: {
    logs: Array,
    operation: String,
    completed: {
      type: Boolean,
      default: false
    },
    success: {
      type: Boolean,
      default: false
    }
  },
  emits: ['close'],
  methods: {
    getLogColor(log) {
      if (log.includes('✓') || log.includes('complete!') || log.includes('SUCCESS')) {
        return 'text-green-400'
      } else if (log.includes('✗') || log.includes('error') || log.includes('Error') || log.includes('failed')) {
        return 'text-red-400'
      } else if (log.includes('warning') || log.includes('Warning')) {
        return 'text-yellow-400'
      } else if (log.includes('#') && (log.includes('###') || log.match(/#+\s*\d/))) {
        return 'text-cyan-400'
      } else if (log.includes('installing') || log.includes('downloading') || log.includes('removing') || log.includes('Retrieving')) {
        return 'text-blue-400'
      } else if (log.includes('::') || log.includes('Packages') || log.includes('Total')) {
        return 'text-purple-400'
      } else {
        return 'text-gray-300'
      }
    },
    copyLogs() {
      const logsText = this.logs.join('\n')
      navigator.clipboard.writeText(logsText).then(() => {
        alert('Logs copied to clipboard!')
      }).catch(err => {
        console.error('Failed to copy logs:', err)
      })
    }
  },
  watch: {
    logs: {
      handler() {
        this.$nextTick(() => {
          if (this.$refs.logContainer) {
            this.$refs.logContainer.scrollTop = this.$refs.logContainer.scrollHeight
          }
        })
      },
      deep: true
    }
  }
}
</script>

