#!/bin/bash

# GuiMan Installation Script
# For Arch-based distributions

set -e

REPO_URL="https://github.com/Junaid433/guiman"
LATEST_RELEASE_URL="$REPO_URL/releases/latest"

echo "🚀 GuiMan Installation Script"
echo "=============================="

# Check if running on Arch-based system
if ! command -v pacman &> /dev/null; then
    echo "❌ This script is designed for Arch-based distributions"
    echo "   Please install manually or use the AppImage"
    exit 1
fi

# Check for AUR helpers
AUR_HELPER=""
if command -v yay &> /dev/null; then
    AUR_HELPER="yay"
elif command -v paru &> /dev/null; then
    AUR_HELPER="paru"
fi

echo "🔍 Detected system: Arch Linux"
if [ -n "$AUR_HELPER" ]; then
    echo "🔍 Detected AUR helper: $AUR_HELPER"
fi

echo ""
echo "📦 Installation Options:"
echo "1. Install from AUR (Recommended)"
echo "2. Download AppImage"
echo "3. Build from source"
echo ""

read -p "Choose installation method (1-3): " choice

case $choice in
    1)
        if [ -z "$AUR_HELPER" ]; then
            echo "❌ No AUR helper found. Please install yay or paru first:"
            echo "   sudo pacman -S --needed git base-devel"
            echo "   git clone https://aur.archlinux.org/yay.git"
            echo "   cd yay && makepkg -si"
            exit 1
        fi
        
        echo "📦 Installing GuiMan from AUR..."
        $AUR_HELPER -S guiman
        ;;
        
    2)
        echo "📥 Downloading AppImage..."
        
        # Get latest release URL
        APPIMAGE_URL=$(curl -s $LATEST_RELEASE_URL | grep -o 'https://.*GuiMan.*AppImage' | head -1)
        
        if [ -z "$APPIMAGE_URL" ]; then
            echo "❌ Could not find AppImage download URL"
            exit 1
        fi
        
        # Download AppImage
        wget -O GuiMan.AppImage "$APPIMAGE_URL"
        chmod +x GuiMan.AppImage
        
        # Move to /usr/local/bin if user wants
        read -p "Install to /usr/local/bin? (y/N): " install_global
        if [[ $install_global =~ ^[Yy]$ ]]; then
            sudo mv GuiMan.AppImage /usr/local/bin/guiman
            echo "✅ GuiMan installed to /usr/local/bin/guiman"
        else
            echo "✅ GuiMan.AppImage downloaded to current directory"
            echo "   Run with: ./GuiMan.AppImage"
        fi
        ;;
        
    3)
        echo "🔨 Building from source..."
        
        # Check dependencies
        echo "📋 Checking build dependencies..."
        
        MISSING_DEPS=()
        for dep in git rust nodejs npm; do
            if ! command -v $dep &> /dev/null; then
                MISSING_DEPS+=($dep)
            fi
        done
        
        if [ ${#MISSING_DEPS[@]} -ne 0 ]; then
            echo "❌ Missing dependencies: ${MISSING_DEPS[*]}"
            echo "   Install with: sudo pacman -S ${MISSING_DEPS[*]}"
            exit 1
        fi
        
        # Clone and build
        git clone $REPO_URL
        cd guiman
        npm install
        npm run tauri build
        
        # Install
        sudo cp src-tauri/target/release/guiman /usr/local/bin/
        sudo cp guiman.desktop /usr/share/applications/
        sudo cp src-tauri/icons/icon.png /usr/share/pixmaps/guiman.png
        
        echo "✅ GuiMan built and installed successfully"
        ;;
        
    *)
        echo "❌ Invalid choice"
        exit 1
        ;;
esac

echo ""
echo "🎉 Installation complete!"
echo ""
echo "🚀 Launch GuiMan:"
echo "   - From applications menu"
echo "   - Or run: guiman"
echo ""
echo "📚 Documentation: $REPO_URL"
echo "🐛 Report issues: $REPO_URL/issues"
echo ""
echo "Thank you for using GuiMan! 🙏"
