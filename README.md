# GuiMan - Arch Linux Package Manager

A modern, beautiful desktop GUI application for managing Arch Linux packages built with **Tauri** (Rust backend) and **Vue.js** (frontend).

![GuiMan](https://img.shields.io/badge/Arch-Linux-blue?logo=arch-linux)
![Tauri](https://img.shields.io/badge/Tauri-1.5-blue)
![Vue.js](https://img.shields.io/badge/Vue.js-3-green)

## Features

### Core Functionality
- **Search Packages**: Search through pacman repositories in real-time
- **Install/Remove Packages**: Manage packages with a single click or batch operations
- **System Updates**: Update all packages with live progress tracking
- **Installed Packages**: View all installed packages
- **Updates Available**: Check for package updates
- **Orphan Detection**: Find and remove orphaned packages
- **Transaction History**: View recent package operations from pacman logs
- **Package Details**: Click any package name to view detailed information
- **Cache Management**: Clean package cache with one click

### Settings & Configuration
- **Persistent Configuration**: All settings saved automatically
- **Window State Memory**: Remembers window size and position between sessions
- **Confirmation Dialogs**: Optional confirmation before install/remove
- **Auto-Refresh**: Automatically refresh package list at configurable intervals
- **Check Updates on Startup**: Option to check for updates when app starts
- **Compact View**: Toggle between normal and compact display modes
- **Show/Hide Descriptions**: Control package description visibility
- **AUR Support** (Experimental): Toggle AUR helper integration (yay/paru)
- **Max Concurrent Downloads**: Configure download parallelism

### UI/UX Features
- **Modern Dark/Light Theme**: Toggle between themes with persistent preference
- **Live Log Streaming**: Watch installation/removal progress in real-time
- **Multi-Select**: Select multiple packages for batch operations
- **Responsive Design**: Clean, minimal interface with Tailwind CSS
- **Status Indicators**: Visual feedback for package states
- **Color-Coded Repositories**: Easy identification of package sources
- **Interactive Package Names**: Click to view detailed package information
- **Smart Confirmations**: Optional safety confirmations for critical operations

## Architecture

### Backend (Rust + Tauri)

**Tauri Commands:**
- `search_package(query: String)` - Search pacman repositories
- `install_package(pkg: String)` - Install package with live streaming
- `remove_package(pkg: String)` - Remove package with live streaming
- `update_system()` - Update all packages
- `list_installed()` - List all installed packages
- `list_orphans()` - List orphaned packages
- `get_package_history()` - Get transaction history
- `check_updates()` - Check for available updates
- `clean_cache()` - Clean package cache
- `get_package_info(pkg: String)` - Get detailed package information
- `export_package_list()` - Export list of installed packages
- `get_cache_size()` - Get current cache size

**Key Technologies:**
- Tauri for native app wrapper
- tokio for async runtime
- pkexec for privilege escalation
- Process streaming for live logs

### Frontend (Vue 3)

**Component Structure:**
- `App.vue` - Main application component with state management
- `Sidebar.vue` - Navigation menu with settings access
- `SearchBar.vue` - Search and system update actions
- `PackageTable.vue` - Package listing with actions and details
- `StatusBar.vue` - Selection status and batch actions
- `LogModal.vue` - Live log viewer with auto-scroll
- `SettingsModal.vue` - Comprehensive settings panel
- `ConfirmDialog.vue` - Safety confirmation dialogs
- `PackageDetailsModal.vue` - Detailed package information viewer

**State Management:**
- ConfigManager utility for persistent settings
- Reactive refs for package data and UI state
- Event listeners for Rust backend events
- LocalStorage for configuration persistence
- Auto-refresh with configurable intervals

## Installation

### Prerequisites

1. **Arch Linux** (or Arch-based distro)
2. **Node.js** (v16+) and npm
3. **Rust** (latest stable)
4. **Tauri dependencies**:
```bash
sudo pacman -S webkit2gtk base-devel curl wget openssl appmenu-gtk-module gtk3 libappindicator-gtk3 librsvg libvips
```

### Build Instructions

1. **Clone the repository**:
```bash
cd /run/media/junaid/512\ GiB\ HDD/Programming\ Arena/Rust/guiman
```

2. **Install frontend dependencies**:
```bash
npm install
```

3. **Run in development mode**:
```bash
npm run tauri dev
```

4. **Build for production**:
```bash
npm run tauri build
```

The built application will be in `src-tauri/target/release/bundle/`

## Usage

### Running the Application

Development mode:
```bash
npm run tauri dev
```

Production:
```bash
./src-tauri/target/release/guiman
```

### Permissions & Password Management

The app uses `pkexec` for privilege escalation when installing/removing packages. 

**You have two options:**

1. **With Password Prompts** (Default)
   - Enter your password for each package operation
   - More secure if you prefer manual confirmation

2. **Password-Free Mode** (Optional - Install from Settings)
   - Go to Settings → System Integration → Install Policy
   - Enter password once, never asked again for your session
   - Same method used by GNOME Software, KDE Discover, etc.

**Your choice!** Both options are secure. Install the policy if you find password prompts annoying.

### Navigation

- **Installed**: View all installed packages
- **Available**: Browse available packages (use search)
- **Updates**: Check for package updates
- **Orphans**: View orphaned packages
- **History**: View recent package operations

### Operations

1. **Search**: Type package name and press Enter or click Search
2. **Install**: Click Install button on any package (confirmation optional)
3. **Remove**: Click Remove button on installed packages (confirmation optional)
4. **Batch Operations**: Select multiple packages and use bottom bar actions
5. **System Update**: Click "Update System" button in top bar
6. **View Details**: Click on any package name to see detailed information
7. **Settings**: Access comprehensive settings from sidebar
8. **Clean Cache**: Remove old package files from settings panel
9. **Optional**: Install polkit policy from Settings to skip password prompts

### Settings Panel

Access settings by clicking the gear icon in the sidebar:

- **System Integration**: Optional polkit policy for password-free operations
- **General Settings**: Confirmations, notifications, descriptions
- **Auto-Refresh**: Configure automatic package list updates
- **AUR Support**: Enable experimental AUR helper integration
- **Cache Management**: View and clean package cache
- **Advanced**: Configure max concurrent downloads
- **Reset**: Restore all settings to defaults

## Design Decisions

### Why Tauri?
- Smaller binary size compared to Electron
- Native performance with Rust
- Better security model
- Lower resource usage

### Why Vue.js?
- Lightweight and fast
- Simple component model
- Excellent reactivity system
- Easy to learn and maintain

### Live Streaming Architecture
- Used Rust's async tokio runtime for non-blocking operations
- Event-based communication from Rust to Vue
- Separate event channels for install/remove/update logs
- Real-time UI updates via Vue's reactive system

### Security Considerations
- Uses `pkexec` instead of hardcoded sudo
- No password storage
- Commands validated before execution
- Sandboxed Tauri environment

## Future Enhancements

- ✅ ~~AUR helper integration (yay/paru)~~ (Added - Experimental)
- ✅ ~~Package dependency visualization~~ (Added in Package Details)
- ✅ ~~Package information modal with full details~~ (Added)
- ✅ ~~Export/import package lists~~ (Added)
- ✅ ~~Scheduled update checks~~ (Added - Auto-refresh)
- Search filters (by repo, installed status, etc.)
- Desktop notifications for updates
- Backup/restore functionality
- Package file browser
- Dependency graph visualization
- Rollback functionality
- Custom package repositories

## Project Structure

```
guiman/
├── src/                    # Vue.js frontend
│   ├── components/         # Vue components
│   │   ├── Sidebar.vue
│   │   ├── SearchBar.vue
│   │   ├── PackageTable.vue
│   │   ├── StatusBar.vue
│   │   ├── LogModal.vue
│   │   ├── SettingsModal.vue
│   │   ├── ConfirmDialog.vue
│   │   └── PackageDetailsModal.vue
│   ├── utils/             # Utility functions
│   │   └── config.js      # Configuration manager
│   ├── App.vue            # Main app component
│   ├── main.js            # Vue entry point
│   └── style.css          # Global styles
├── src-tauri/             # Tauri/Rust backend
│   ├── src/
│   │   └── main.rs        # Rust Tauri commands
│   ├── Cargo.toml         # Rust dependencies
│   ├── tauri.conf.json    # Tauri configuration
│   └── build.rs           # Build script
├── package.json           # Node dependencies
├── vite.config.js         # Vite bundler config
├── tailwind.config.js     # Tailwind CSS config
├── postcss.config.js      # PostCSS config
└── setup.sh               # Setup script
```

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

MIT License

## Credits

Built with ❤️ for the Arch Linux community

