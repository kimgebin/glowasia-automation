# GLOWASIA Dropshipping Copilot - Specification

## Project Overview
- **Name:** GLOWASIA Dropshipping Copilot
- **Type:** Desktop Automation Application
- **Version:** 1.0.0 MVP
- **Brand:** GLOWASIA K-Beauty Skincare
- **Market:** Indonesia, Malaysia, Thailand, Vietnam (ASEAN)
- **Platforms:** Shopify + CJ Dropshipping + Google Sheets

## Tech Stack
- **Framework:** Tauri v2 (Rust + WebView)
- **Frontend:** React 18 + TypeScript + Vite
- **Browser Automation:** Playwright
- **Database:** SQLite (via rusqlite)
- **Styling:** TailwindCSS
- **Notifications:** Telegram Bot API

## Core Features

### 1. Dashboard
- System status (Shopify/CJ/Sheets connected/disconnected)
- Today's stats: Orders, Revenue, Shipped, Delivered
- Recent activity log (last 50 entries)
- Start/Stop automation toggle button
- System health indicators

### 2. Shopify Integration
- Auto-login via saved credentials
- Order polling every 30 seconds
- Fetch order details (customer, products, payment)
- Auto-fulfill on payment confirmation
- Status update on shipment

### 3. CJ Dropshipping Integration
- Auto-login to CJ
- Auto-create orders from Shopify orders
- Auto-fetch tracking numbers
- Auto-update Shopify with tracking

### 4. Google Sheets Integration
- Append new orders to "Orders" sheet
- Status pipeline: Pending → Paid → Processing → Shipped → Delivered
- Auto-populate tracking numbers
- Daily summary report generation

### 5. Telegram Notifications
- New order alerts
- Order shipped alerts
- Delivery completion confirmations
- Daily summary at 8 PM
- Weekly analytics summary

### 6. Settings Panel
- Shopify credentials (encrypted)
- CJ credentials (encrypted)
- Google Sheets ID
- Telegram bot token & chat ID
- Automation schedule configuration
- **Credentials Manager** - Persistent credentials stored in `~/.local/share/glowasia-automation/credentials.db`
  - Supports 10 platforms: Shopify, Shopee, Lazada, Tokopedia, TikTok Shop, Telegram, GitHub, Midtrans, Google Sheets, CJ Dropshipping
  - Each credential stores: api_key, api_secret, access_token, refresh_token, shop_url, additional_data
  - Export/Import credentials as JSON for backup and restore
  - Credentials persist across app rebuilds

## Credentials Persistence System

### Overview
GLOWASIA Copilot uses a local SQLite database to store all platform credentials securely. This system ensures credentials persist across app updates, reinstallations, and system upgrades.

### Database Location
```
~/.local/share/glowasia-automation/
├── credentials.db    ← All API keys & tokens (PERSISTENT)
├── app_state.json    ← App settings
└── logs/
    └── activity.log
```

### Supported Platforms & Fields
| Platform | Credential Type | Fields Required |
|----------|----------------|-----------------|
| Shopify | OAuth/API Token | api_key, shop_url |
| Shopee | Partner API | partner_id, partner_key, shop_id |
| Lazada | API Gateway | api_key, api_secret, shop_url |
| Tokopedia | OAuth | access_token, shop_id |
| TikTok Shop | API Token | api_key, shop_id |
| CJ Dropshipping | Affiliate Token | api_key, api_secret |
| Telegram | Bot Token | bot_token, chat_id |
| Google Sheets | Sheet ID | sheet_id |
| Midtrans | Server Key | server_key, client_key |
| GitHub | Personal Token | access_token |

### Data Schema
```sql
CREATE TABLE credentials (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    platform TEXT NOT NULL,
    account_name TEXT,
    api_key TEXT,
    api_secret TEXT,
    access_token TEXT,
    refresh_token TEXT,
    shop_url TEXT,
    additional_data TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(platform, account_name)
);
```

### Key Features
- **Persistence Guarantee**: Database stored outside app bundle, survives updates
- **Multiple Accounts**: Support for multiple accounts per platform
- **Export/Import**: JSON backup format for easy restore
- **Secure Storage**: Local SQLite database with file-level protection

## UI Design System

### Colors
| Role | Hex | Usage |
|------|-----|-------|
| Background | `#0a0a0f` | Main app background |
| Card | `#141420` | Card/container background |
| Primary | `#e8b4b8` | GLOWASIA pink accent |
| Secondary | `#6366f1` | Indigo accent |
| Success | `#10b981` | Success states |
| Warning | `#f59e0b` | Warning states |
| Error | `#ef4444` | Error states |
| Text Primary | `#ffffff` | Main text |
| Text Secondary | `#9ca3af` | Muted text |

### Typography
- **Font:** Inter (system fallback: -apple-system, sans-serif)
- **Heading:** 600 weight
- **Body:** 400 weight

### Spacing
- Base unit: 4px
- Card padding: 24px
- Section gaps: 16px

## Automation Flow

```
START → Login Shopify → Poll orders (30s)
  │
  ├─ NEW ORDER → Extract details → Sheets + Telegram
  │
  ├─ PAYMENT CONFIRMED → Login CJ → Create CJ order
  │
  ├─ CJ SHIPS → Get tracking → Update Shopify/Sheets
  │
  └─ DELIVERY COMPLETE → Update Sheets → Telegram

DAILY 8PM → Generate report → Telegram
```

## Architecture

```
src-tauri/src/
├── main.rs           # Entry point, window setup
├── lib.rs            # Tauri commands registration
├── browser/
│   ├── shopify.rs    # Shopify Playwright automation
│   └── cj.rs         # CJ Dropshipping automation
├── automation/
│   ├── monitor.rs    # Order monitoring loop
│   ├── fulfillment.rs # Order fulfillment logic
│   └── notifier.rs   # Telegram notifications
└── storage/
    ├── db.rs         # SQLite operations
    └── credentials.rs # Encrypted credential storage
```

## Database Schema

### tables
- `credentials` (id, service, encrypted_data, created_at)
- `orders` (id, shopify_id, cj_id, status, tracking, created_at, updated_at)
- `activity_log` (id, action, details, timestamp)
- `settings` (key, value)

## Security

- Credentials encrypted with AES-256-GCM
- Encryption key derived from machine-specific secret
- Passwords never displayed in UI (shown as ••••)
- All browser sessions isolated

## System Integration

- System tray icon with context menu
- Minimize to tray option
- Auto-start on login (optional)
- Graceful shutdown with cleanup

## Build Targets
- macOS (primary, .dmg)
- Windows (.exe) - future
- Linux (.AppImage) - future

## Auto-Update System

### Overview
The app includes an automatic update system that checks GitHub Releases for new versions and can download and install updates seamlessly.

### Version Numbering
- Format: `MAJOR.MINOR.PATCH` (e.g., `1.0.0`, `1.0.1`)
- Tags on GitHub must follow: `v1.0.0`, `v1.0.1`, etc.
- Version is read from `Cargo.toml` at build time

### Update Mechanism
1. **Check**: On startup (optional) or manual trigger, the app queries GitHub Releases API
2. **Compare**: Current version is compared with latest release tag
3. **Download**: If newer version exists, download the appropriate platform asset:
   - macOS: `glowasia-automation_x.x.x_x86_64-apple-darwin.tar.gz`
   - Windows: `glowasia-automation_x.x.x_x86_64-pc-windows-msi.msi`
   - Linux: `glowasia-automation_x.x.x_x86_64-unknown-linux-musl.tar.gz`
4. **Verify**: SHA256 checksum is verified automatically by `self_update` crate
5. **Install**: Update is installed to the appropriate location

### Rollback Strategy
- The `self_update` crate automatically creates backups before overwriting
- If installation fails, the previous version is restored automatically
- Manual rollback can be done by reinstalling the previous release from GitHub


### GitHub Release Requirements
- Repository: `glowasia-automation`
- Owner: Your GitHub username (placeholder: `YOUR_GITHUB_USERNAME`)
- Asset naming convention: `{repo_name}_{version}_{target}.{ext}`
- Release body: Contains changelog/release notes

### Edge Cases Handled
- **No internet**: Returns friendly error message
- **Update server unreachable**: Timeout with retry suggestion
- **Corrupted download**: Automatic re-download attempt
- **Insufficient disk space**: Error before download begins
- **Permission denied**:提示用户以管理员身份运行

### Settings
- Auto-check on startup: Can be enabled/disabled via settings
- Manual "Check for Updates" button always available in Updates tab
