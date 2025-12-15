<template>
  <!-- Header-based Notification Center -->
  <div v-if="showCenter" class="absolute top-full right-0 mt-1 w-96 max-h-96 bg-white dark:bg-gray-800 rounded-lg shadow-xl border border-gray-200 dark:border-gray-700 overflow-hidden z-50">
    <div class="p-4 border-b border-gray-200 dark:border-gray-700">
      <div class="flex items-center justify-between">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Notifications</h3>
        <div class="flex gap-2">
          <button
            v-if="notifications.length > 0"
            @click="clearAll"
            class="text-sm text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200"
          >
            Clear All
          </button>
          <button
            @click="$emit('close')"
            class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
          >
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
          </button>
        </div>
      </div>
    </div>

    <div class="max-h-80 overflow-y-auto">
      <div v-if="notifications.length === 0" class="p-8 text-center text-gray-500 dark:text-gray-400">
        <svg class="w-12 h-12 mx-auto mb-4 text-gray-300 dark:text-gray-600" fill="currentColor" viewBox="0 0 20 20">
          <path d="M10 2a6 6 0 00-6 6v3.586l-.707.707A1 1 0 004 14h12a1 1 0 00.707-1.707L16 11.586V8a6 6 0 00-6-6zM10 18a3 3 0 01-3-3h6a3 3 0 01-3 3z" />
        </svg>
        <p>No notifications yet</p>
      </div>

      <div v-else class="divide-y divide-gray-200 dark:divide-gray-700">
        <div
          v-for="notification in notifications.slice(0, 50)"
          :key="notification.id"
          :class="[
            'p-4 hover:bg-gray-50 dark:hover:bg-gray-700 cursor-pointer transition-colors',
            !notification.read ? 'bg-blue-50 dark:bg-blue-900/20' : ''
          ]"
          @click="markAsRead(notification.id)"
        >
          <div class="flex items-start gap-3">
            <div :class="['flex-shrink-0 w-6 h-6 rounded-full flex items-center justify-center', getIconBgClass(notification.type)]">
              <component :is="getIconComponent(notification.type)" class="w-4 h-4 text-white" />
            </div>
            <div class="flex-1 min-w-0">
              <div class="flex items-center justify-between">
                <h4 class="text-sm font-semibold text-gray-900 dark:text-white truncate">
                  {{ notification.title }}
                </h4>
                <span class="text-xs text-gray-500 dark:text-gray-400 whitespace-nowrap ml-2">
                  {{ formatTime(notification.timestamp) }}
                </span>
              </div>
              <p class="text-sm text-gray-600 dark:text-gray-300 mt-1">
                {{ notification.message }}
              </p>
              <div v-if="notification.progress !== undefined" class="mt-2">
                <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-1">
                  <div
                    class="h-1 rounded-full transition-all duration-300"
                    :class="getProgressBarClass(notification.type)"
                    :style="{ width: notification.progress + '%' }"
                  ></div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Minimal Toast for Immediate Feedback (optional) -->
  <div v-else-if="activeNotifications.length > 0" class="fixed top-4 right-4 z-50 space-y-2 max-w-sm">
    <TransitionGroup name="notification" tag="div">
      <div
        v-for="notification in activeNotifications.slice(0, 3)"
        :key="notification.id"
        :class="[
          'bg-white dark:bg-gray-800 border-l-4 rounded-lg shadow-lg p-4 max-w-sm transform transition-all duration-300 ease-in-out',
          getBorderClass(notification.type),
          { 'translate-x-full opacity-0': notification.exiting }
        ]"
        @mouseenter="pauseAutoHide(notification.id)"
        @mouseleave="resumeAutoHide(notification.id)"
        @click="dismiss(notification.id)"
      >
        <div class="flex items-start gap-3">
          <!-- Icon -->
          <div :class="['flex-shrink-0 w-6 h-6 rounded-full flex items-center justify-center', getIconBgClass(notification.type)]">
            <component :is="getIconComponent(notification.type)" class="w-4 h-4 text-white" />
          </div>

          <!-- Content -->
          <div class="flex-1 min-w-0">
            <div class="flex items-center justify-between">
              <h4 class="text-sm font-semibold text-gray-900 dark:text-white">
                {{ notification.title }}
              </h4>
              <button
                @click="dismiss(notification.id)"
                class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 ml-2"
              >
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                </svg>
              </button>
            </div>
            <p class="text-sm text-gray-600 dark:text-gray-300 mt-1">
              {{ notification.message }}
            </p>

            <!-- Actions -->
            <div v-if="notification.actions && notification.actions.length > 0" class="flex gap-2 mt-3">
              <button
                v-for="action in notification.actions"
                :key="action.label"
                @click="handleAction(notification.id, action)"
                :class="[
                  'px-3 py-1 text-xs font-medium rounded-md transition-colors',
                  action.primary
                    ? 'bg-blue-600 hover:bg-blue-700 text-white'
                    : 'bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 text-gray-800 dark:text-gray-200'
                ]"
              >
                {{ action.label }}
              </button>
            </div>

            <!-- Progress Bar -->
            <div v-if="notification.progress !== undefined" class="mt-3">
              <div class="flex items-center justify-between text-xs text-gray-500 dark:text-gray-400 mb-1">
                <span>{{ notification.progress }}%</span>
                <span v-if="notification.eta">{{ notification.eta }}</span>
              </div>
              <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                <div
                  class="h-2 rounded-full transition-all duration-300"
                  :class="getProgressBarClass(notification.type)"
                  :style="{ width: notification.progress + '%' }"
                ></div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </TransitionGroup>

    <!-- Notification Center Toggle -->
    <button
      @click="showNotificationCenter = !showNotificationCenter"
      :class="[
        'fixed bottom-4 right-4 z-40 p-3 rounded-full shadow-lg transition-all duration-200',
        showNotificationCenter ? 'bg-blue-600 text-white' : 'bg-white dark:bg-gray-800 text-gray-600 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700',
        unreadCount > 0 ? 'ring-2 ring-red-500 ring-opacity-75' : ''
      ]"
      :title="showNotificationCenter ? 'Hide notifications' : 'Show notifications'"
    >
      <div class="relative">
        <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
          <path d="M10 2a6 6 0 00-6 6v3.586l-.707.707A1 1 0 004 14h12a1 1 0 00.707-1.707L16 11.586V8a6 6 0 00-6-6zM10 18a3 3 0 01-3-3h6a3 3 0 01-3 3z" />
        </svg>
        <span v-if="unreadCount > 0" class="absolute -top-1 -right-1 bg-red-500 text-white text-xs rounded-full h-5 w-5 flex items-center justify-center">
          {{ unreadCount > 99 ? '99+' : unreadCount }}
        </span>
      </div>
    </button>

    <!-- Notification Center Panel -->
    <div
      v-if="showNotificationCenter"
      class="fixed bottom-16 right-4 w-80 max-h-96 bg-white dark:bg-gray-800 rounded-lg shadow-xl border border-gray-200 dark:border-gray-700 overflow-hidden z-50"
    >
      <div class="p-4 border-b border-gray-200 dark:border-gray-700">
        <div class="flex items-center justify-between">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Notifications</h3>
          <div class="flex gap-2">
            <button
              v-if="notifications.length > 0"
              @click="clearAll"
              class="text-sm text-gray-600 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200"
            >
              Clear All
            </button>
            <button
              @click="showNotificationCenter = false"
              class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
            >
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
              </svg>
            </button>
          </div>
        </div>
      </div>

      <div class="max-h-80 overflow-y-auto">
        <div v-if="notifications.length === 0" class="p-8 text-center text-gray-500 dark:text-gray-400">
          <svg class="w-12 h-12 mx-auto mb-4 text-gray-300 dark:text-gray-600" fill="currentColor" viewBox="0 0 20 20">
            <path d="M10 2a6 6 0 00-6 6v3.586l-.707.707A1 1 0 004 14h12a1 1 0 00.707-1.707L16 11.586V8a6 6 0 00-6-6zM10 18a3 3 0 01-3-3h6a3 3 0 01-3 3z" />
          </svg>
          <p>No notifications yet</p>
        </div>

        <div v-else class="divide-y divide-gray-200 dark:divide-gray-700">
          <div
            v-for="notification in notifications.slice(0, 50)"
            :key="notification.id"
            :class="[
              'p-4 hover:bg-gray-50 dark:hover:bg-gray-700 cursor-pointer transition-colors',
              !notification.read ? 'bg-blue-50 dark:bg-blue-900/20' : ''
            ]"
            @click="markAsRead(notification.id)"
          >
            <div class="flex items-start gap-3">
              <div :class="['flex-shrink-0 w-6 h-6 rounded-full flex items-center justify-center', getIconBgClass(notification.type)]">
                <component :is="getIconComponent(notification.type)" class="w-4 h-4 text-white" />
              </div>
              <div class="flex-1 min-w-0">
                <div class="flex items-center justify-between">
                  <h4 class="text-sm font-semibold text-gray-900 dark:text-white truncate">
                    {{ notification.title }}
                  </h4>
                  <span class="text-xs text-gray-500 dark:text-gray-400 whitespace-nowrap ml-2">
                    {{ formatTime(notification.timestamp) }}
                  </span>
                </div>
                <p class="text-sm text-gray-600 dark:text-gray-300 mt-1">
                  {{ notification.message }}
                </p>
                <div v-if="notification.progress !== undefined" class="mt-2">
                  <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-1">
                    <div
                      class="h-1 rounded-full transition-all duration-300"
                      :class="getProgressBarClass(notification.type)"
                      :style="{ width: notification.progress + '%' }"
                    ></div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { sendNotification, isPermissionGranted, requestPermission } from '@tauri-apps/plugin-notification'

// Icon Components
const SuccessIcon = {
  template: `<svg fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" /></svg>`
}

const ErrorIcon = {
  template: `<svg fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" /></svg>`
}

const WarningIcon = {
  template: `<svg fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" /></svg>`
}

const InfoIcon = {
  template: `<svg fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" /></svg>`
}

export default {
  name: 'NotificationSystem',
  components: {
    SuccessIcon,
    ErrorIcon,
    WarningIcon,
    InfoIcon
  },
  props: {
    config: {
      type: Object,
      required: true
    },
    showCenter: {
      type: Boolean,
      default: false
    }
  },
  emits: ['action', 'close'],
  data() {
    return {
      notifications: [],
      activeNotifications: [],
      showNotificationCenter: false,
      nextId: 1,
      autoHideTimeouts: new Map()
    }
  },
  computed: {
    unreadCount() {
      return this.notifications.filter(n => !n.read).length
    }
  },
  methods: {
    // Public methods to be called by parent components
    show(type, title, message, options = {}) {
      const notification = {
        id: this.nextId++,
        type: type || 'info',
        title,
        message,
        timestamp: new Date(),
        read: false,
        actions: options.actions || [],
        progress: options.progress,
        eta: options.eta,
        persistent: options.persistent || false,
        sound: options.sound !== false
      }

      // Add to notifications list
      this.notifications.unshift(notification)

      // Always show as toast notification (not just when dropdown is open)
      this.activeNotifications.push(notification)
      if (!notification.persistent) {
        this.scheduleAutoHide(notification.id)
      }

      // Send desktop notification
      this.sendDesktopNotification(notification)

      // Play sound if enabled
      if (notification.sound && this.config.notificationSound) {
        this.playNotificationSound(notification.type)
      }

      // Limit notifications history
      if (this.notifications.length > 100) {
        this.notifications = this.notifications.slice(0, 100)
      }
    },

    // Update progress of an existing notification
    updateProgress(id, progress, eta) {
      const notification = this.notifications.find(n => n.id === id)
      if (notification) {
        notification.progress = progress
        notification.eta = eta

        const activeNotification = this.activeNotifications.find(n => n.id === id)
        if (activeNotification) {
          activeNotification.progress = progress
          activeNotification.eta = eta
        }
      }
    },

    dismiss(id) {
      const index = this.activeNotifications.findIndex(n => n.id === id)
      if (index > -1) {
        const notification = this.activeNotifications[index]
        notification.exiting = true

        setTimeout(() => {
          this.activeNotifications.splice(index, 1)
        }, 300)
      }

      // Clear auto-hide timeout
      if (this.autoHideTimeouts.has(id)) {
        clearTimeout(this.autoHideTimeouts.get(id))
        this.autoHideTimeouts.delete(id)
      }
    },

    markAsRead(id) {
      const notification = this.notifications.find(n => n.id === id)
      if (notification) {
        notification.read = true
      }
    },

    clearAll() {
      this.notifications = []
      this.activeNotifications.forEach(n => this.dismiss(n.id))
    },

    pauseAutoHide(id) {
      if (this.autoHideTimeouts.has(id)) {
        clearTimeout(this.autoHideTimeouts.get(id))
      }
    },

    resumeAutoHide(id) {
      this.scheduleAutoHide(id)
    },

    scheduleAutoHide(id) {
      const timeout = setTimeout(() => {
        this.dismiss(id)
        this.autoHideTimeouts.delete(id)
      }, 5000) // 5 seconds

      this.autoHideTimeouts.set(id, timeout)
    },

    handleAction(notificationId, action) {
      this.$emit('action', { notificationId, action })
      this.dismiss(notificationId)
    },

    async sendDesktopNotification(notification) {
      if (!this.config.showNotifications) return

      try {
        let permissionGranted = await isPermissionGranted()
        if (!permissionGranted) {
          const permission = await requestPermission()
          permissionGranted = permission === 'granted'
        }

        if (permissionGranted) {
          sendNotification({
            title: notification.title,
            body: notification.message
            // No icon specified to avoid showing app logo
          })
        }
      } catch (error) {
        console.error('Desktop notification error:', error)
      }
    },

    playNotificationSound(type) {
      // Create audio context and play appropriate sound
      try {
        const audioContext = new (window.AudioContext || window.webkitAudioContext)()
        const oscillator = audioContext.createOscillator()
        const gainNode = audioContext.createGain()

        oscillator.connect(gainNode)
        gainNode.connect(audioContext.destination)

        // Different frequencies for different types
        const frequencies = {
          success: 800,
          error: 400,
          warning: 600,
          info: 500
        }

        oscillator.frequency.setValueAtTime(frequencies[type] || 500, audioContext.currentTime)
        oscillator.type = 'sine'

        gainNode.gain.setValueAtTime(0.1, audioContext.currentTime)
        gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.3)

        oscillator.start(audioContext.currentTime)
        oscillator.stop(audioContext.currentTime + 0.3)
      } catch (error) {
        // Fallback or silent failure
        console.warn('Could not play notification sound:', error)
      }
    },

    // Helper methods
    getIconComponent(type) {
      const icons = {
        success: 'SuccessIcon',
        error: 'ErrorIcon',
        warning: 'WarningIcon',
        info: 'InfoIcon'
      }
      return icons[type] || 'InfoIcon'
    },

    getIconBgClass(type) {
      const classes = {
        success: 'bg-green-500',
        error: 'bg-red-500',
        warning: 'bg-yellow-500',
        info: 'bg-blue-500'
      }
      return classes[type] || 'bg-blue-500'
    },

    getBorderClass(type) {
      const classes = {
        success: 'border-green-500',
        error: 'border-red-500',
        warning: 'border-yellow-500',
        info: 'border-blue-500'
      }
      return classes[type] || 'border-blue-500'
    },

    getProgressBarClass(type) {
      const classes = {
        success: 'bg-green-500',
        error: 'bg-red-500',
        warning: 'bg-yellow-500',
        info: 'bg-blue-500'
      }
      return classes[type] || 'bg-blue-500'
    },


    formatTime(timestamp) {
      const now = new Date()
      const diff = now - timestamp

      const minutes = Math.floor(diff / 60000)
      const hours = Math.floor(diff / 3600000)
      const days = Math.floor(diff / 86400000)

      if (days > 0) return `${days}d ago`
      if (hours > 0) return `${hours}h ago`
      if (minutes > 0) return `${minutes}m ago`
      return 'Just now'
    }
  }
}
</script>

<style scoped>
.notification-enter-active, .notification-leave-active {
  transition: all 0.3s ease;
}

.notification-enter-from {
  opacity: 0;
  transform: translateX(100%);
}

.notification-leave-to {
  opacity: 0;
  transform: translateX(100%);
}

.notification-move {
  transition: transform 0.3s ease;
}
</style>
