# 🛡️ Defender Disabler

> Automatically disables Windows Defender Real-time Protection from the system tray.

## ✨ Features

- 🔄 **Auto-Disable** - Monitors and disables Real-time Protection every 5 seconds
- 🪟 **System Tray** - Runs hidden with balloon notifications
- ⚡ **Lightweight** - Uses ~20 MB RAM with ~0% CPU usage

## 📦 Installation

### Option 1: Download Release
Download the latest installer from the [Releases](../../releases) page.

### Option 2: Build from Source

- Open `DefenderDisabler.sln` in Visual Studio
- Build in `Release` mode
- Run the installer in `DefenderDisablerInstaller\bin\Release\DefenderDisablerInstaller.msi`

## 🚀 Usage

1. **Run as Administrator** (right-click → "Run as Administrator")
2. Check system tray for the Defender Disabler icon
3. Right-click tray icon → **Exit** to close

> ⚠️ If not run as administrator, the app will show an error and exit.

## 🔧 Requirements

- Windows 7 or later
- .NET Framework 4.8
- Administrator privileges

## ❗ Disclaimer

**For educational purposes only.** Disabling Windows Defender reduces system security. Use at your own risk. Not recommended for production systems.