# GLOWASIA Copilot - Setup Guide

## Table of Contents
1. [Prerequisites](#prerequisites)
2. [Installation](#installation)
3. [First-Time Setup](#first-time-setup)
4. [Platform Configuration](#platform-configuration)
5. [Verification](#verification)
6. [Next Steps](#next-steps)

---

## Prerequisites

### System Requirements

| Requirement | Minimum | Recommended |
|-------------|---------|-------------|
| macOS Version | 10.15 (Catalina) | 12 (Monterey) or later |
| RAM | 4 GB | 8 GB |
| Free Storage | 500 MB | 1 GB |
| Display | 1280x720 | 1920x1080 |
| Internet | Stable broadband | Stable broadband |

### Required Accounts

Before setting up GLOWASIA Copilot, ensure you have accounts ready for:

- [ ] **Shopify Store** - Your main e-commerce store
- [ ] **CJ Dropshipping Account** - For product sourcing and fulfillment
- [ ] **Google Account** - For Google Sheets integration
- [ ] **Telegram Bot** - For notifications (optional but recommended)

---

## Installation

### Option 1: Download Pre-built App (Recommended)

#### Step 1: Download the App

1. Visit [GitHub Releases](https://github.com/kimgebin/glowasia-automation/releases)
2. Download the latest release: `glowasia-automation_vX.X.X_x86_64-apple-darwin.tar.gz`
3. Save to your Downloads folder

#### Step 2: Extract the Archive

```bash
cd ~/Downloads
tar -xzf glowasia-automation_v*.tar.gz
```

#### Step 3: Install to Applications

```bash
# Option A: Copy to Applications (recommended)
cp -R "GLOWASIA Copilot.app" /Applications/

# Option B: Or drag manually via Finder
# Open Finder → Downloads → Drag "GLOWASIA Copilot.app" to Applications
```

#### Step 4: Launch the App

1. Open **Spotlight** (⌘ + Space)
2. Type `GLOWASIA Copilot`
3. Press **Enter**

> **First Launch Note**: macOS may ask for permission to open an app from the internet. Click "Open" in the dialog.

---

### Option 2: Build from Source

#### Step 1: Install Prerequisites

```bash
# Install Node.js (if not already installed)
brew install node

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Git
brew install git
```

#### Step 2: Clone the Repository

```bash
git clone https://github.com/kimgebin/glowasia-automation.git
cd glowasia-automation
```

#### Step 3: Install Dependencies

```bash
npm install
```

#### Step 4: Build the Application

```bash
npm run tauri build -- --bundles app
```

The built application will be at:
```
src-tauri/target/release/bundle/macos/GLOWASIA Copilot.app
```

#### Step 5: Install

```bash
cp -R "src-tauri/target/release/bundle/macos/GLOWASIA Copilot.app" /Applications/
```

---

## First-Time Setup

### Launching GLOWASIA Copilot

1. Open the application from Applications or Spotlight
2. The dashboard will display with all platform indicators showing "Not Connected"
3. The status bar at the bottom shows system health

### Initial Configuration Wizard

#### Step 1: Open Settings

- Click the **Settings** (⚙️) icon in the left sidebar
- Or use keyboard shortcut **⌘ + ,**

#### Step 2: Navigate to Credentials

- Click the **Credentials** (🔐) tab in the Settings panel

#### Step 3: Add Your First Credential

1. Select the platform from the dropdown (e.g., Shopify)
2. Enter the required credentials:
   - **Shopify**: API Key, API Secret Key, Shop URL
   - **CJ Dropshipping**: Username, Password
   - **Telegram**: Bot Token, Chat ID
3. Click **Save**
4. The credential is encrypted and stored locally

#### Step 4: Test Connection

1. Return to the **Dashboard**
2. Find the platform card
3. Click **Test Connection**
4. If successful, the indicator turns green

---

## Platform Configuration

### Shopify Setup

#### 1. Create a Custom App in Shopify

1. Log in to your [Shopify Admin](https://admin.shopify.com)
2. Go to **Settings** → **Apps and sales channels**
3. Click **Develop apps for your store**
4. Click **Allow custom app development**
5. Click **Create an app**
6. Name it "GLOWASIA Copilot"
7. Under **API credentials**, click **Install app**
8. Copy the **Admin API access token** (save securely - shown only once!)

#### 2. Configure API Access

1. Go to **Configuration** tab
2. Select the following scopes:
   - `read_orders`
   - `write_orders`
   - `read_products`
   - `write_products`
   - `read_fulfillments`
   - `write_fulfillments`
3. Click **Save**

#### 3. Add to GLOWASIA Copilot

1. In GLOWASIA Copilot → Settings → Credentials
2. Select **Shopify**
3. Enter:
   - **Shop URL**: `yourstore.myshopify.com`
   - **API Key**: Your Admin API access token
4. Click **Save** → **Test Connection**

---

### CJ Dropshipping Setup

#### 1. Create CJ Account

1. Go to [CJ Dropshipping](https://www.cjdropshipping.com)
2. Register for an account (or log in if you have one)
3. Complete store authorization

#### 2. Get CJ API Credentials

1. Log in to CJ Dashboard
2. Go to **My Account** → **Platform Settings**
3. Find your **Affiliate Token** or API credentials
4. Copy the token

#### 3. Add to GLOWASIA Copilot

1. In GLOWASIA Copilot → Settings → Credentials
2. Select **CJ Dropshipping**
3. Enter your CJ affiliate token/credentials
4. Click **Save** → **Test Connection**

---

### Telegram Bot Setup

#### 1. Create a Telegram Bot

1. Open Telegram and search for **@BotFather**
2. Send `/newbot`
3. Follow prompts to name your bot
4. Copy the **Bot Token** (format: `123456789:ABCdefGhIJKlmNoPQRsTUVwxyZ`)

#### 2. Get Your Chat ID

1. Search for **@userinfobot** in Telegram
2. Start the bot
3. It will reply with your **Chat ID** (a number like `123456789`)

#### 3. Add to GLOWASIA Copilot

1. In GLOWASIA Copilot → Settings → Credentials
2. Select **Telegram**
3. Enter:
   - **Bot Token**: Your bot token from BotFather
   - **Chat ID**: Your chat ID from userinfobot
4. Click **Save** → **Test Connection**

#### 4. Verify Notifications

1. Send `/start` to your bot
2. You should receive a welcome message
3. Test by triggering an automation action

---

### Google Sheets Setup

#### 1. Create a Google Sheet

1. Go to [Google Sheets](https://sheets.google.com)
2. Create a new spreadsheet
3. Name it "GLOWASIA Orders Tracker"
4. Create these sheets/tabs:
   - **Orders** - Main order tracking
   - **Products** - Product inventory
   - **Settings** - Configuration values

#### 2. Share the Sheet

1. Click **Share** in your Google Sheet
2. Click **Get link**
3. Set permissions to **Anyone with the link** (or "Anyone can edit" for auto-updates)
4. Copy the Sheet ID from the URL:
   ```
   https://docs.google.com/spreadsheets/d/[SHEET_ID]/edit
   ```

#### 3. Add to GLOWASIA Copilot

1. In GLOWASIA Copilot → Settings → Credentials
2. Select **Google Sheets**
3. Enter the **Sheet ID**
4. Click **Save** → **Test Connection**

---

## Verification

### Checklist

After setup, verify each item:

- [ ] App launches successfully
- [ ] Settings panel opens (⌘ + ,)
- [ ] Credentials tab is accessible
- [ ] All platform credentials are saved
- [ ] Test Connection shows green for each platform
- [ ] Dashboard shows platform status cards
- [ ] Telegram notifications are working (if configured)

### Running Your First Automation

1. Ensure all credentials are saved and tested
2. Go to **Dashboard**
3. Toggle **Automation** to **ON**
4. Watch the Activity Log for live updates
5. Create a test order in Shopify
6. Verify it appears in Google Sheets
7. Verify you receive a Telegram notification

---

## Next Steps

- Read the [Credentials Guide](CREDENTIALS.md) for advanced credential management
- Read the [Automation Guide](AUTOMATION.md) to understand workflows
- Read the [Troubleshooting Guide](TROUBLESHOOTING.md) if you encounter issues

---

## Need Help?

1. Check [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for common issues
2. Check existing [GitHub Issues](https://github.com/kimgebin/glowasia-automation/issues)
3. Create a new issue with:
   - macOS version
   - App version
   - Steps to reproduce
   - Expected vs actual behavior
