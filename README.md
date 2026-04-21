# GLOWASIA Copilot

**100% Auto-Pilot Dropshipping Automation Desktop Application**

[![License](https://img.shields.io/badge/license-Proprietary-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-macOS-yellow.svg)](README.md)
[![Version](https://img.shields.io/badge/version-1.0.0-green.svg)](RELEASES)

---

## 🎯 Overview

GLOWASIA Copilot is a powerful desktop application designed for e-commerce entrepreneurs in the ASEAN market. It automates dropshipping operations across multiple platforms, enabling 100% auto-pilot business operations.

### Supported Platforms

| Platform | Status | Features |
|----------|--------|----------|
| Shopify | ✅ Active | Product sync, order processing, inventory |
| Shopee | ✅ Active | Auto-listing, price sync, order fulfillment |
| Lazada | ✅ Active | Multi-country listing, logistics |
| Tokopedia | ✅ Active | Order management, stock sync |
| TikTok Shop | ✅ Active | Viral content integration, fast checkout |
| CJ Dropshipping | ✅ Active | Auto-fulfillment, ePacket shipping |
| Telegram | ✅ Active | Notifications, alerts, bot commands |
| Google Sheets | ✅ Active | Inventory tracker, order log |
| Midtrans | 🔄 Setup | Payment gateway integration |

---

## ✨ Features

### Dashboard
- Real-time statistics monitoring
- Activity log with live updates
- Platform connection status
- Quick actions panel

### Automation
- **Auto Product Import**: Sync products from CJ Dropshipping to all platforms
- **Auto Price Update**: Keep prices synchronized across all stores
- **Auto Order Fulfillment**: Process orders automatically
- **Auto Stock Sync**: Update inventory in real-time
- **Telegram Notifications**: Get alerts for important events

### Credentials Management
- **Persistent Storage**: Credentials survive app updates
- **Secure Database**: Local SQLite database
- **Export/Import**: Backup and restore credentials
- **Multiple Accounts**: Support for multiple accounts per platform

### Auto-Update System
- Patch-based updates via GitHub Releases
- Silent background updates
- One-click manual update check
- Rollback support if update fails

---

## 📋 System Requirements

### Minimum Requirements
- **OS**: macOS 10.15 (Catalina) or later
- **RAM**: 4 GB
- **Storage**: 500 MB free space
- **Display**: 1280x720 minimum

### Recommended Requirements
- **OS**: macOS 12 (Monterey) or later
- **RAM**: 8 GB
- **Storage**: 1 GB free space
- **Display**: 1920x1080 or higher

---

## 🚀 Installation

### Option 1: Download Pre-built App (Recommended)

1. Go to [Releases](https://github.com/kimgebin/glowasia-automation/releases)
2. Download latest release: `glowasia-automation_vX.X.X_x86_64-apple-darwin.tar.gz`
3. Extract the archive
4. Drag `GLOWASIA Copilot.app` to Applications folder
5. Launch from Applications or Spotlight

### Option 2: Build from Source

```bash
# Clone the repository
git clone https://github.com/kimgebin/glowasia-automation.git
cd glowasia-automation

# Install dependencies
npm install

# Build the application
npm run tauri build -- --bundles app

# The built app will be at:
# src-tauri/target/release/bundle/macos/GLOWASIA Copilot.app
```

---

## 🔧 Configuration

### First Time Setup

1. **Launch the Application**
   - Open GLOWASIA Copilot from Applications
   - Dashboard will show "Not Connected" status initially

2. **Configure Credentials**
   - Click **Settings** (⚙️ icon in sidebar)
   - Go to **Credentials** tab (🔐 icon)
   - Add credentials for each platform you use:
     - **Shopify**: API Key, Secret Key, Shop URL
     - **Shopee**: Partner ID, Partner Key, Shop ID
     - **Telegram**: Bot Token, Chat ID

3. **Test Connections**
   - Dashboard → Click "Test Connection" for each platform
   - Green indicator = Connected
   - Red indicator = Check credentials

### Credentials Database Location

```
~/.local/share/glowasia-automation/
├── credentials.db    ← All API keys & tokens stored here
├── app_state.json    ← App settings
└── logs/
    └── activity.log
```

**Important**: This database PERSISTS across app updates. You don't need to re-enter credentials when updating the app.

---

## 📖 User Guide

### Dashboard

The main dashboard shows:
- **Platform Status**: Which platforms are connected
- **Recent Activity**: Last 10 actions performed
- **Quick Stats**: Orders today, products synced, etc.
- **Status Bar**: Real-time connection indicator (auto-refresh every 5s)

### Managing Platforms

#### Shopify Setup
1. Create a custom app in Shopify Admin
2. Get API credentials from Partners Dashboard
3. Add credentials in Settings → Credentials
4. Test connection

#### Shopee Setup
1. Register as Shopee Partner
2. Get Partner ID and Partner Key
3. Authorize your shop
4. Add credentials

#### Telegram Bot Setup
1. Create bot via @BotFather
2. Get bot token
3. Start chat with @userinfobot to get your Chat ID
4. Add credentials

### Automation Workflows

#### Auto Product Import
```
CJ Dropshipping → App → Shopify/Shopee/Lazada/Tokopedia/TikTok
     ↓
  1. Add products to import queue
  2. Set price markup rules
  3. Select target platforms
  4. Click "Start Import"
  5. Products appear on all stores
```

#### Auto Order Fulfillment
```
Customer Order → Platform → App (auto-detect) → CJ Dropshipping
     ↓
  1. Order received from any platform
  2. App auto-forwards to CJ
  3. CJ ships with ePacket
  4. Tracking number auto-updated
  5. Customer notified via Telegram
```

---

## 🔐 Security

### Data Storage
- All credentials stored locally in `credentials.db`
- Database uses SQLite with file-level encryption
- No credentials sent to external servers (except to respective platforms)

### Best Practices
1. **Never share credentials** with anyone
2. **Use separate accounts** for development and production
3. **Regularly update** the app for security fixes
4. **Export backup** of credentials periodically

---

## 🔄 Updating

### Automatic Updates
The app checks for updates on every launch (if auto-check enabled).

### Manual Update Check
1. Open Settings → Updates tab (🔄)
2. Click "Check for Updates"
3. If update available, click "Download & Install"
4. App will restart with new version

### Rollback
If update fails:
1. Delete the app
2. Download previous release from GitHub
3. Restore `credentials.db` from backup (if needed)

---

## 🐛 Troubleshooting

### App Won't Launch
```
1. Check macOS version (needs 10.15+)
2. Try reinstalling
3. Check System Preferences → Security
```

### Credentials Not Saving
```
1. Check write permissions for ~/.local/share/
2. Ensure no other instance running
3. Try deleting credentials.db and re-add
```

### Platform Connection Failed
```
1. Verify credentials are correct
2. Check internet connection
3. Check platform API status
4. Try re-adding credentials
```

---

## 📂 Project Structure

```
glowasia-automation/
├── src/                    # React frontend
│   ├── components/         # UI components
│   │   ├── Dashboard.tsx   # Main dashboard
│   │   ├── Settings.tsx    # Settings panel (credentials, updates)
│   │   ├── StatusBar.tsx   # Bottom status bar
│   │   └── ActivityLog.tsx # Activity log
│   ├── hooks/             # React hooks
│   ├── pages/             # Page components
│   ├── App.tsx            # Main app component
│   └── main.tsx           # Entry point
│
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── lib.rs         # Main library + Tauri commands
│   │   ├── main.rs        # App entry point
│   │   ├── db.rs          # Database operations
│   │   ├── credentials.rs # Credentials management
│   │   ├── browser.rs     # Browser automation
│   │   └── storage/       # Storage modules
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
│
├── public/                # Static assets
├── package.json           # Node dependencies
├── SPEC.md               # Technical specification
├── README.md             # This file
└── LICENSE               # License file
```

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    GLOWASIA Copilot                        │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌─────────────────┐        ┌─────────────────────────┐      │
│  │   React UI      │ ←────→ │   Rust Backend (Tauri)   │      │
│  │   (TypeScript)  │        │   (WebView + Commands)   │      │
│  └─────────────────┘        └───────────┬─────────────┘      │
│                                          │                    │
│                    ┌─────────────────────┼────────────────┐    │
│                    ↓                     ↓                │    │
│          ┌─────────────────┐   ┌────────────────────┐     │    │
│          │  Credentials DB │   │   Platform APIs    │     │    │
│          │  (SQLite)       │   │   (Shopify, etc)   │     │    │
│          │  PERSISTENT     │   └────────────────────┘     │    │
│          └─────────────────┘                             │    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## 📄 License

Copyright © 2026 GLOWASIA. All rights reserved.

This application is proprietary software. See LICENSE file for details.

---

## 🤝 Support

For issues and feature requests:
1. Check [Troubleshooting](docs/TROUBLESHOOTING.md) section
2. Check existing [GitHub Issues](https://github.com/kimgebin/glowasia-automation/issues)
3. Create new issue with:
   - macOS version
   - App version
   - Steps to reproduce
   - Expected vs actual behavior

---

## 📈 Changelog

### v1.0.0 (2026-04-22)
- Initial release
- Dashboard with real-time stats
- Shopify, Shopee, Lazada, Tokopedia, TikTok Shop integration
- CJ Dropshipping auto-fulfillment
- Telegram notifications
- Google Sheets integration
- Auto-update system via GitHub Releases
- **NEW**: Persistent credentials database

---

*Made with ❤️ by GLOWASIA Team*
