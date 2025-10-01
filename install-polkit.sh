#!/bin/bash

echo "🔐 Installing GuiMan Polkit Policy"
echo "=================================="
echo ""
echo "This will allow GuiMan to remember your password for 5 minutes"
echo "after the first authentication, so you don't have to enter it"
echo "for every package operation."
echo ""

if [ "$EUID" -ne 0 ]; then 
    echo "Running with sudo..."
    sudo cp polkit/com.guiman.pkexec.policy /usr/share/polkit-1/actions/
else
    cp polkit/com.guiman.pkexec.policy /usr/share/polkit-1/actions/
fi

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Polkit policy installed successfully!"
    echo ""
    echo "Now GuiMan will remember your authentication for the session."
    echo "You'll only need to enter your password once every 5 minutes."
else
    echo ""
    echo "❌ Failed to install polkit policy"
    echo "Please run: sudo ./install-polkit.sh"
fi

