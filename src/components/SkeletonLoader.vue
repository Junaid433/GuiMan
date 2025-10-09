<template>
  <div class="animate-pulse">
    <!-- Card skeleton -->
    <div v-if="type === 'card'" class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
      <div class="flex items-start justify-between mb-4">
        <div class="flex items-start gap-3 flex-1">
          <!-- Icon skeleton -->
          <div class="w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded-lg flex-shrink-0"></div>

          <!-- Content skeleton -->
          <div class="flex-1 space-y-2">
            <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-3/4"></div>
            <div class="h-3 bg-gray-200 dark:bg-gray-700 rounded w-1/2"></div>
            <div class="h-3 bg-gray-200 dark:bg-gray-700 rounded w-full"></div>
            <div class="h-3 bg-gray-200 dark:bg-gray-700 rounded w-2/3"></div>
          </div>
        </div>
      </div>

      <!-- Bottom section skeleton -->
      <div class="flex items-center justify-between">
        <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-20"></div>
        <div class="flex gap-2">
          <div class="h-6 bg-gray-200 dark:bg-gray-700 rounded w-16"></div>
          <div class="h-6 bg-gray-200 dark:bg-gray-700 rounded w-16"></div>
        </div>
      </div>
    </div>

    <!-- Table row skeleton -->
    <div v-else-if="type === 'table-row'" class="bg-white dark:bg-gray-900 divide-y divide-gray-200 dark:divide-gray-700">
      <div v-for="i in count" :key="i" class="px-4 py-3 flex items-center gap-4">
        <div class="w-4 h-4 bg-gray-200 dark:bg-gray-700 rounded"></div>
        <div class="flex-1 space-y-1">
          <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-1/4"></div>
          <div class="h-3 bg-gray-200 dark:bg-gray-700 rounded w-1/3"></div>
        </div>
        <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-16"></div>
        <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-20"></div>
        <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-24"></div>
        <div class="flex gap-2">
          <div class="h-6 bg-gray-200 dark:bg-gray-700 rounded w-16"></div>
          <div class="h-6 bg-gray-200 dark:bg-gray-700 rounded w-16"></div>
        </div>
      </div>
    </div>

    <!-- Dashboard card skeleton -->
    <div v-else-if="type === 'dashboard-card'" class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
      <div class="flex items-center justify-between">
        <div class="space-y-2">
          <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-24"></div>
          <div class="h-8 bg-gray-200 dark:bg-gray-700 rounded w-16"></div>
        </div>
        <div class="w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded-lg"></div>
      </div>
    </div>

    <!-- Analytics skeleton -->
    <div v-else-if="type === 'analytics'" class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
      <div class="flex items-center justify-between mb-6">
        <div class="h-6 bg-gray-200 dark:bg-gray-700 rounded w-32"></div>
        <div class="flex gap-2">
          <div class="h-8 bg-gray-200 dark:bg-gray-700 rounded w-20"></div>
          <div class="h-8 bg-gray-200 dark:bg-gray-700 rounded w-20"></div>
          <div class="h-8 bg-gray-200 dark:bg-gray-700 rounded w-20"></div>
          <div class="h-8 bg-gray-200 dark:bg-gray-700 rounded w-20"></div>
        </div>
      </div>

      <!-- Chart placeholder -->
      <div class="flex flex-col lg:flex-row gap-6">
        <div class="flex-1">
          <div class="w-full h-64 bg-gray-200 dark:bg-gray-700 rounded-lg flex items-center justify-center">
            <div class="text-center">
              <div class="w-16 h-16 bg-gray-300 dark:bg-gray-600 rounded-full mx-auto mb-4"></div>
              <div class="h-4 bg-gray-300 dark:bg-gray-600 rounded w-24 mx-auto"></div>
            </div>
          </div>
        </div>
        <div class="lg:w-64 space-y-2">
          <div v-for="i in 5" :key="i" class="flex items-center justify-between p-2">
            <div class="flex items-center gap-2">
              <div class="w-3 h-3 bg-gray-200 dark:bg-gray-700 rounded-full"></div>
              <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-16"></div>
            </div>
            <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-8"></div>
          </div>
        </div>
      </div>
    </div>

    <!-- List skeleton -->
    <div v-else-if="type === 'list'" class="space-y-4">
      <div v-for="i in count" :key="i" class="bg-white dark:bg-gray-800 rounded-lg p-4">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 bg-gray-200 dark:bg-gray-700 rounded-lg"></div>
          <div class="flex-1 space-y-2">
            <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-1/3"></div>
            <div class="h-3 bg-gray-200 dark:bg-gray-700 rounded w-1/2"></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'SkeletonLoader',
  props: {
    type: {
      type: String,
      default: 'card',
      validator: value => ['card', 'table-row', 'dashboard-card', 'analytics', 'list'].includes(value)
    },
    count: {
      type: Number,
      default: 1
    }
  }
}
</script>
