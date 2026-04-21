# GLOWASIA Copilot

**100% Auto-Pilot Dropshipping Automation Desktop Application**

[![License](https://img.shields.io/badge/license-Proprietary-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-macOS-yellow.svg)](README.md)
[![Version](https://img.shields.io/badge/version-1.0.0-green.svg)](RELEASES)

---

## 🎯 Overview

GLOWASIA Copilot is a powerful desktop application designed for e-commerce entrepreneurs in the ASEAN market. It automates dropshipping operations across multiple platforms, enabling 100% auto-pilot business operations.

### Key Benefits

- 🚀 **Save Time** - Automate repetitive tasks like order fulfillment and inventory updates
- 💰 **Reduce Costs** - Zero upfront investment for automation infrastructure
- 📈 **Scale Fast** - Manage multiple stores from one dashboard
- 🔒 **Secure** - Local credential storage, never exposed to third parties

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
- Real-time statistics monitoring (auto-refresh every 5 seconds)
- Activity log with live updates
- Platform connection status with visual indicators
- Quick actions panel for common tasks

### Automation
- **Auto Product Import**: Sync products from CJ Dropshipping to all platforms simultaneously
- **Auto Price Update**: Keep prices synchronized with markup rules across all stores
- **Auto Order Fulfillment**: Process orders automatically without manual intervention
- **Auto Stock Sync**: Update inventory in real-time across all platforms
- **Telegram Notifications**: Get instant alerts for important events and order updates

### Credentials Management
- **Persistent Storage**: Credentials survive app updates and rebuilds
- **Secure Database**: Local SQLite database at `~/.local/share/glowasia-automation/credentials.db`
- **Export/Import**: Backup and restore credentials as JSON file
- **Multiple Accounts**: Support for multiple accounts per platform
- **Individual Platform Fields**: Each platform has specific credential fields (API keys, tokens, URLs, etc.)

### Auto-Update System
- Patch-based updates via GitHub Releases (no full reinstall needed)
- Silent background updates (configurable)
- One-click manual update check
- Rollback support if update fails
- Version changelog display

---

## 📋 System Requirements

### Minimum Requirements
- **OS**: macOS 10.15 (Catalina) or later
- **RAM**: 4 GB
- **Storage**: 500 MB free space
- **Display**: 1280x720 minimum resolution

### Recommended Requirements
- **OS**: macOS 12 (Monterey) or later
- **RAM**: 8 GB
- **Storage**: 1 GB free space
- **Display**: 1920x1080 or higher resolution

---

## 🚀 Installation

### Option 1: Download Pre-built App (Recommended)

1. Go to [Releases](https://github.com/kimgebin/glowasia-automation/releases)
2. Download latest release: `glowasia-automation_vX.X.X_x86_64-apple-darwin.tar.gz`
3. Extract the archive (double-click)
4. Drag `GLOWASIA Copilot.app` to Applications folder
5. Launch from Applications or Spotlight (Cmd+Space)

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
   - Add credentials for each platform you use (see platform-specific guides below)

3. **Test Connections**
   - Dashboard → Click "Test Connection" for each platform
   - Green indicator = Connected
   - Red indicator = Check credentials

### Credentials Database Location

```
~/.local/share/glowasia-automation/
├── credentials.db    ← All API keys & tokens stored here (PERSISTS!)
├── app_state.json    ← App settings
└── logs/
    └── activity.log
```

**Important**: This database PERSISTS across app updates. You don't need to re-enter credentials when updating the app.

---

## 📖 Platform Setup Guides

### Shopify Setup

1. **Create a Custom App in Shopify**
   - Go to Shopify Admin → Settings → Apps and sales channels
   - Click "Develop app" → "Create an app"
   - Name it "GLOWASIA Copilot"
   - Add these API scopes:
     - `read_products`, `write_products`
     - `read_orders`, `write_orders`
     - `read_inventory`, `write_inventory`

2. **Get API Credentials**
   - In your app → "API credentials" tab
   - Copy the Admin API access token
   - Note your Shop URL (e.g., `yourstore.myshopify.com`)

3. **Add to GLOWASIA Copilot**
   - Settings → Credentials → Add New
   - Select "Shopify"
   - Enter: API Key = Admin API Token, Shop URL = yourstore.myshopify.com

### Shopee Setup

1. **Register as Shopee Partner** (if not already)
   - Go to https://partner.shopeemobile.com/
   - Register as partner

2. **Get Credentials**
   - Get Partner ID and Partner Key from Shopee Partner Portal
   - Authorize your shop to get Shop ID

3. **Add to GLOWASIA Copilot**
   - Settings → Credentials → Add New
   - Select "Shopee"
   - Enter: Partner ID, Partner Key, Shop ID

### Telegram Bot Setup

1. **Create a Bot**
   - Open Telegram → Search for @BotFather
   - Send `/newbot`
   - Give it a name and username
   - Copy the bot token (looks like `123456789:ABCdefGhIJKlmNoPQRsTUVwxyz`)

2. **Get Your Chat ID**
   - Search for @userinfobot in Telegram
   - Start a chat → It will show your numeric Chat ID

3. **Add to GLOWASIA Copilot**
   - Settings → Credentials → Add New
   - Select "Telegram"
   - Enter: Bot Token, Chat ID

### Other Platforms

Similar credential setup for:
- **Lazada**: API Key, Secret, User ID
- **Tokopedia**: Client ID, Client Secret
- **TikTok Shop**: App Key, App Secret
- **CJ Dropshipping**: API Key, Secret
- **Midtrans**: Server Key, Client Key

---

## 📂 Project Structure

```
glowasia-automation/
├── src/                    # React frontend (TypeScript)
│   ├── components/         # UI components
│   │   ├── Dashboard.tsx   # Main dashboard with stats
│   │   ├── Settings.tsx    # Settings panel (credentials, updates)
│   │   ├── StatusBar.tsx   # Bottom status bar (auto-refresh)
│   │   ├── ActivityLog.tsx # Activity log display
│   │   └── Reports.tsx     # Reports & analytics
│   ├── hooks/             # React custom hooks
│   │   └── useAutomation.ts # Automation logic hook
│   ├── pages/             # Page components
│   │   └── App.tsx        # Main app container
│   ├── App.tsx            # App root component
│   ├── main.tsx           # Entry point
│   └── index.css          # Global styles
│
├── src-tauri/             # Rust backend (Tauri)
│   ├── src/
│   │   ├── lib.rs         # Main library + Tauri commands
│   │   ├── main.rs        # App entry point
│   │   ├── db.rs          # Database operations
│   │   ├── credentials.rs # Credentials management (persistent)
│   │   ├── browser.rs     # Browser automation (Playwright)
│   │   ├── update.rs      # Auto-update system
│   │   └── storage/       # Storage modules
│   │       └── mod.rs
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
┌─────────────────────────────────────────────────────────────────────┐
│                         GLOWASIA Copilot                            │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────────────────┐      ┌────────────────────────────┐  │
│  │      React Frontend     │ ←──→ │      Rust Backend (Tauri)   │  │
│  │      (TypeScript)       │      │      (WebView + Commands)   │  │
│  │                         │      │                             │  │
│  │  - Dashboard (real-time)│      │  - Tauri Commands          │  │
│  │  - Settings (credentials)│     │  - SQLite Database         │  │
│  │  - Status Bar (5s auto) │      │  - Browser Automation      │  │
│  │  - Activity Log         │      │  - Auto Updater            │  │
│  └─────────────────────────┘      └───────────┬────────────────┘  │
│                                                │                   │
│                        ┌────────────────────────┼────────────────┐ │
│                        ↓                        ↓                │ │
│              ┌─────────────────┐    ┌────────────────────┐     │ │
│              │  Credentials DB  │    │   Platform APIs    │     │ │
│              │  (SQLite)        │    │   (Shopify, etc)  │     │ │
│              │                  │    └────────────────────┘     │ │
│              │  PERSISTENT      │                               │ │
│              │  ~/.local/share/ │                               │ │
│              └─────────────────┘                               │ │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘

Data Flow:
1. User configures credentials → Stored in credentials.db
2. App reads credentials → Connects to platforms
3. Platform events → App processes → Updates all stores
4. Alerts → Telegram notification
5. Orders → Auto-fulfilled via CJ Dropshipping
```

---

## 🔐 Security

### Data Storage
- All credentials stored locally in `~/.local/share/glowasia-automation/credentials.db`
- Database uses SQLite (file-level security via macOS permissions)
- No credentials sent to external servers except to respective platforms
- Each platform only receives its own required authentication data

### Best Practices
1. **Never share credentials** with anyone, including support personnel
2. **Use separate accounts** for development and production environments
3. **Regularly update** the app for security fixes
4. **Export backup** of credentials periodically
5. **Delete old credentials** for platforms you no longer use

### Backup & Restore
```
Settings → Credentials → Export Backup
→ Creates JSON file with all credentials

Settings → Credentials → Import Backup  
→ Restores from JSON file
```

---

## 🔄 Updating

### Automatic Updates
The app checks for updates on every launch (if auto-check enabled in Settings).

### Manual Update Check
1. Open Settings → Updates tab (🔄)
2. Click "Check for Updates"
3. If update available, click "Download & Install"
4. App will restart with new version

### Update Process
```
Check Update → Download Patch → Verify SHA256 → Install → Restart
                     ↓
              If failed → Rollback to previous version
```

### Version History
- **v1.0.0** (2026-04-22): Initial release with all major features

---

## 🐛 Troubleshooting

### App Won't Launch
```
Solutions:
1. Check macOS version (needs 10.15+)
2. Try reinstalling the app
3. Check System Preferences → Security & Privacy
4. Check if another instance is running (Activity Monitor)
```

### Credentials Not Saving
```
Solutions:
1. Check write permissions for ~/.local/share/
2. Ensure no other instance of app is running
3. Try deleting credentials.db and re-adding
4. Check disk space
```

### Platform Connection Failed
```
Solutions:
1. Verify credentials are correct (no typos)
2. Check internet connection
3. Check platform API status (is it down?)
4. Try re-adding credentials
5. For Shopify: Ensure API token hasn't expired
```

### Auto-Update Not Working
```
Solutions:
1. Check GitHub token is configured
2. Verify internet connection
3. Manually download release from GitHub
4. Check release asset format matches expected
```

---

## 📄 License

Copyright © 2026 GLOWASIA. All rights reserved.

This application is proprietary software. Redistribution or commercial use without permission is prohibited.

See [LICENSE](LICENSE) file for full license terms.

---

## 🤝 Support

For issues and feature requests:
1. Check [Troubleshooting](#-troubleshooting) section above
2. Check existing GitHub Issues
3. Create new issue with:
   - macOS version
   - App version (from Settings → About)
   - Steps to reproduce
   - Expected vs actual behavior
   - Screenshots if applicable

---

## 🙏 Acknowledgments

- Built with [Tauri](https://tauri.app/) (Rust + WebView)
- React frontend with TypeScript
- SQLite for persistent storage
- Playwright for browser automation
- Inspired by the goal of 100% e-commerce automation for ASEAN entrepreneurs

---

*Made with ❤️ by GLOWASIA Team*