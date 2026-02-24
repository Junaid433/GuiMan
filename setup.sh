#!/bin/bash

echo "🚀 GuiMan Setup Script"
echo "======================="
echo ""

echo "📦 Checking dependencies..."

if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js v16 or higher."
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "✅ Node.js version: $(node --version)"
echo "✅ npm version: $(npm --version)"
echo "✅ Rust version: $(rustc --version)"
echo ""

echo "📦 Installing system dependencies..."
sudo pacman -S --needed webkit2gtk-4.1 base-devel curl wget openssl appmenu-gtk-module gtk3 libappindicator-gtk3 librsvg libvips

echo ""
echo "📦 Installing npm dependencies..."
npm install

echo ""
echo "✅ Setup complete!"
echo ""
echo "🎯 Next steps:"
echo "  1. Run 'npm run tauri dev' for development mode"
echo "  2. Open Settings in GuiMan and install the polkit policy"
echo "     (This removes password prompts for every operation)"
echo ""
echo "⚠️  Important: Install the polkit policy from Settings to avoid"
echo "   entering your password on every package operation!"
echo ""
echo "Happy package managing! 🎉"
