<template>
  <div class="relative" @keydown.stop="onKeydown">
    <button
      ref="button"
      type="button"
      :aria-expanded="open.toString()"
      :aria-haspopup="true"
      class="w-full flex items-center justify-between px-3 py-2 bg-gray-50 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-900 dark:text-white text-sm focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-600"
      @click="toggle"
      @blur="onBlur"
    >
      <span class="truncate">{{ selectedLabel }}</span>
      <svg class="w-4 h-4 text-gray-500 dark:text-gray-300 ml-2" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
        <path fill-rule="evenodd" d="M5.23 7.21a.75.75 0 011.06.02L10 10.94l3.71-3.71a.75.75 0 111.06 1.06l-4.24 4.24a.75.75 0 01-1.06 0L5.21 8.29a.75.75 0 01.02-1.08z" clip-rule="evenodd" />
      </svg>
    </button>

    <ul
      v-if="open"
      ref="list"
      class="absolute z-50 mt-1 w-full max-h-48 overflow-auto bg-white dark:bg-gray-700 border border-gray-200 dark:border-gray-600 rounded-lg shadow-lg py-1"
      role="listbox"
      :aria-activedescendant="`option-${focusedIndex}`"
      tabindex="-1"
    >
      <li
        v-for="(opt, idx) in options"
        :key="opt.value"
        :id="`option-${idx}`"
        role="option"
        :aria-selected="modelValue === opt.value"
        class="px-3 py-2 text-sm cursor-pointer truncate hover:bg-gray-100 dark:hover:bg-gray-600"
        :class="{'bg-gray-100 dark:bg-gray-600': focusedIndex === idx}
        "
        @mousedown.prevent="select(opt.value)"
        @mousemove="focusedIndex = idx"
      >
        {{ opt.label }}
      </li>
    </ul>
  </div>
</template>

<script>
export default {
  name: 'CustomSelect',
  props: {
    options: {
      type: Array,
      required: true
    },
    modelValue: {
      type: [String, Number, Object],
      default: null
    }
  },
  emits: ['update:modelValue'],
  data() {
    return {
      open: false,
      focusedIndex: -1
    }
  },
  computed: {
    selectedLabel() {
      const found = this.options.find(o => o.value === this.modelValue)
      return found ? found.label : (this.options[0] && this.options[0].label) || ''
    }
  },
  methods: {
    toggle() {
      this.open = !this.open
      if (this.open) {
        this.$nextTick(() => {
          this.focusedIndex = this.options.findIndex(o => o.value === this.modelValue)
          if (this.focusedIndex < 0) this.focusedIndex = 0
        })
      }
    },
    select(value) {
      this.$emit('update:modelValue', value)
      this.$emit('change', value)
      this.open = false
    },
    onBlur(e) {
      // Close on blur (with slight timeout to allow click selection)
      setTimeout(() => { this.open = false }, 150)
    },
    onKeydown(e) {
      if (!this.open && (e.key === 'ArrowDown' || e.key === 'ArrowUp' || e.key === 'Enter' || e.key === ' ')) {
        e.preventDefault()
        this.toggle()
        return
      }

      if (!this.open) return

      if (e.key === 'ArrowDown') {
        e.preventDefault()
        if (this.focusedIndex < this.options.length - 1) this.focusedIndex++
      } else if (e.key === 'ArrowUp') {
        e.preventDefault()
        if (this.focusedIndex > 0) this.focusedIndex--
      } else if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault()
        const opt = this.options[this.focusedIndex]
        if (opt) this.select(opt.value)
      } else if (e.key === 'Escape') {
        e.preventDefault()
        this.open = false
      }
    }
  }
}
</script>

<style scoped>
/* Ensure the options use the same dark-mode colors */
:deep(.dark) .py-1 {
  background: #374151;
}
</style>
