<template>
  <div class="w-full max-w-md mx-auto">
    <svg :width="size" :height="size" viewBox="0 0 100 100" class="drop-shadow-sm">
      <!-- Pie slices -->
      <g v-for="(slice, index) in slices" :key="index">
        <path
          :d="slice.path"
          :fill="slice.color"
          :stroke="strokeColor"
          :stroke-width="strokeWidth"
          class="transition-opacity duration-150 hover:opacity-80 cursor-pointer"
          @mouseenter="handleSliceEnter($event, index)"
          @mousemove="handleMouseMove"
          @mouseleave="handleSliceLeave"
        />
      </g>

      <!-- Center circle for donut effect -->
      <circle
        v-if="donut"
        cx="50"
        cy="50"
        :r="innerRadius"
        :fill="centerColor"
      />

      <!-- Center text -->
      <text
        v-if="centerText"
        x="50"
        y="50"
        text-anchor="middle"
        dominant-baseline="middle"
        class="text-xs font-bold fill-gray-900 dark:fill-white"
      >
        {{ centerText }}
      </text>
    </svg>

    <!-- Tooltip -->
    <div
      v-if="hoveredSlice !== null && slices[hoveredSlice]"
      class="absolute bg-black text-white px-2 py-1 rounded text-sm pointer-events-none z-10 transform -translate-x-1/2 -translate-y-full"
      :style="{
        left: tooltipPosition.x + 'px',
        top: tooltipPosition.y + 'px'
      }"
    >
      {{ slices[hoveredSlice].label }}
      <div class="text-xs opacity-75">{{ slices[hoveredSlice].value }} ({{ slices[hoveredSlice].percentage.toFixed(1) }}%)</div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'PieChart',
  props: {
    data: {
      type: Array,
      default: () => []
    },
    size: {
      type: Number,
      default: 200
    },
    donut: {
      type: Boolean,
      default: false
    },
    strokeColor: {
      type: String,
      default: '#ffffff'
    },
    strokeWidth: {
      type: Number,
      default: 2
    },
    centerColor: {
      type: String,
      default: '#ffffff'
    }
  },
  data() {
    return {
      hoveredSlice: null,
      tooltipPosition: { x: 0, y: 0 }
    }
  },
  computed: {
    total() {
      return this.data.reduce((sum, item) => sum + item.value, 0)
    },
    slices() {
      if (this.total === 0) return []

      let currentAngle = -90 // Start from top
      const radius = 40
      const innerRadius = this.donut ? 20 : 0

      return this.data.map(item => {
        const percentage = (item.value / this.total) * 100
        const angle = (percentage / 100) * 360
        const startAngle = currentAngle
        const endAngle = currentAngle + angle

        // Convert angles to radians
        const startRad = (startAngle * Math.PI) / 180
        const endRad = (endAngle * Math.PI) / 180

        // Calculate path
        const x1 = 50 + radius * Math.cos(startRad)
        const y1 = 50 + radius * Math.sin(startRad)
        const x2 = 50 + radius * Math.cos(endRad)
        const y2 = 50 + radius * Math.sin(endRad)

        const largeArcFlag = angle > 180 ? 1 : 0

        let path = `M 50 50 L ${x1} ${y1} A ${radius} ${radius} 0 ${largeArcFlag} 1 ${x2} ${y2} Z`

        if (this.donut) {
          const ix1 = 50 + innerRadius * Math.cos(endRad)
          const iy1 = 50 + innerRadius * Math.sin(endRad)
          const ix2 = 50 + innerRadius * Math.cos(startRad)
          const iy2 = 50 + innerRadius * Math.sin(startRad)

          path = `M ${ix1} ${iy1} L ${x1} ${y1} A ${radius} ${radius} 0 ${largeArcFlag} 1 ${x2} ${y2} L ${ix2} ${iy2} A ${innerRadius} ${innerRadius} 0 ${largeArcFlag} 0 ${ix1} ${iy1} Z`
        }

        const slice = {
          path,
          color: item.color,
          label: item.name,
          value: item.value,
          percentage
        }

        currentAngle = endAngle
        return slice
      })
    },
    centerText() {
      return this.total > 0 ? this.total.toString() : '0'
    },
    innerRadius() {
      return this.donut ? 20 : 0
    }
  },
  methods: {
    handleMouseMove(event) {
      // Only update when hovering a slice (not globally)
      if (this.hoveredSlice !== null) {
        this.tooltipPosition = {
          x: event.clientX,
          y: event.clientY - 10
        }
      }
    },
    handleSliceEnter(event, index) {
      this.hoveredSlice = index
      this.handleMouseMove(event)
    },
    handleSliceLeave() {
      this.hoveredSlice = null
    }
  }
}
</script>
