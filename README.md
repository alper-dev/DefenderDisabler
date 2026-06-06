# Defender Disabler

A lightweight Windows background service that prevents Windows Defender from automatically re-enabling itself.

## Features

- Event-driven architecture with near-zero resource usage
- Monitors registry keys for Defender state changes
- Instantly disables Defender when detected as enabled
- System tray icon with status display
- Windows toast notifications
- Automatic startup with Windows
- Tamper Protection detection and retry logic
- Proper installer with uninstaller
- Supports both x86 and x64 Windows

## Requirements

- Windows 10/11
- Administrator privileges (for installation and service operation)

## Installation

1. Download the latest `defender-disabler-setup.exe` from [Releases](https://github.com/alper-dev/DefenderDisabler/releases)
2. Run the installer as Administrator
3. The service will start automatically and disable Defender

## Usage

After installation:
- The service runs in the background as a Windows Service
- A system tray icon shows the current Defender status
- Right-click the tray icon to see options:
  - **Defender Status**: Shows current state (ON/OFF)
  - **View Logs**: Opens the log folder and highlights the log file
  - **Exit**: Stops the service and closes the tray app

## Tamper Protection

If Tamper Protection is enabled, the service will:
1. Show a toast notification warning you
2. Retry every 30 seconds
3. Show reminder notifications every 5 minutes

To disable Tamper Protection:
1. Open Windows Security
2. Go to Virus & threat protection
3. Click Manage settings
4. Toggle Tamper Protection OFF

## How It Works

1. The service monitors these registry keys for changes:
   - `HKLM\SOFTWARE\Microsoft\Windows Defender\Real-Time Protection`
   - `HKLM\SOFTWARE\Microsoft\Windows Defender\Features`
   - `HKLM\SOFTWARE\Microsoft\Windows Defender`
   - `HKLM\SOFTWARE\Policies\Microsoft\Windows Defender`
   - `HKLM\SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection`

2. When a change is detected, it checks if Defender is enabled
3. If enabled, it immediately disables real-time protection

## Logging

Logs are stored at:
```
C:\Program Files\Defender Disabler\defender-disabler.log
```

## Uninstalling

1. Open Windows Settings > Apps > Installed apps
2. Find "Defender Disabler" and click Uninstall
3. Or run the uninstaller from the Start Menu

## Building from Source

### Prerequisites

- Rust (install from https://rustup.rs)
- Inno Setup 7.0+ (for building the installer)

### Build Steps

```bash
# Clone the repository
git clone https://github.com/alper-dev/DefenderDisabler.git
cd DefenderDisabler

# Build release binaries (x64)
cargo build --release

# Build release binaries (x86)
cargo build --release --target i686-pc-windows-msvc

# Build installer (requires Inno Setup)
iscc installer.iss
```

## Technical Details

- Written in Rust for minimal resource usage
- Uses `RegNotifyChangeKeyValue` API for event-driven registry monitoring
- Implements Windows Service API for proper service management
- Communication between service and tray app via named pipes
- Service runs as SYSTEM account for registry access
- Tray app runs per-user for UI interaction

## License

MIT License

## Disclaimer

This tool is for educational and personal use only. Disabling Windows Defender may leave your system vulnerable to malware. Use at your own risk.
