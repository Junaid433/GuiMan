const DEFAULT_CONFIG = {
  theme: 'light',
  autoRefresh: false,
  refreshInterval: 300,
  confirmActions: true,
  showNotifications: true,
  notificationSound: true,
  notificationDuration: 5000, // ms
  checkUpdatesOnStartup: true,
  aurSupport: false,
  aurHelper: 'yay',
  maxConcurrentDownloads: 3,
  keepPackageCache: true,
  language: 'en',
  compactView: false,
  showDescriptions: true,
  viewMode: 'table', // 'table' or 'cards'
  sidebarCollapsed: false,
  downloadTimeout: 180,
  checkDiskSpace: true,
  verifyPackages: true,
  colorOutput: true,
  verboseOutput: false,
}

export class ConfigManager {
  constructor() {
    this.config = this.load()
  }

  load() {
    try {
      const saved = localStorage.getItem('guiman_config')
      if (saved) {
        return { ...DEFAULT_CONFIG, ...JSON.parse(saved) }
      }
    } catch (e) {
      console.error('Failed to load config:', e)
    }
    return { ...DEFAULT_CONFIG }
  }

  save(config) {
    try {
      this.config = { ...this.config, ...config }
      localStorage.setItem('guiman_config', JSON.stringify(this.config))
      return true
    } catch (e) {
      console.error('Failed to save config:', e)
      return false
    }
  }

  get(key) {
    return this.config[key]
  }

  set(key, value) {
    this.config[key] = value
    this.save(this.config)
  }

  reset() {
    this.config = { ...DEFAULT_CONFIG }
    localStorage.removeItem('guiman_config')
  }
}

export default new ConfigManager()

