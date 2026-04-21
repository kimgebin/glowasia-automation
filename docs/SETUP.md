# GLOWASIA Copilot - Setup Guide

## Table of Contents
1. [Prerequisites](#prerequisites)
2. [Installation](#installation)
3. [Initial Configuration](#initial-configuration)
4. [Platform Setup](#platform-setup)
5. [Testing](#testing)

---

## Prerequisites

### System Requirements
- macOS 10.15 (Catalina) or later
- 4GB RAM minimum (8GB recommended)
- 500MB free disk space
- Internet connection

### Required Accounts
Before setup, ensure you have accounts for:
- [ ] Shopify store (or create trial)
- [ ] Shopee seller account
- [ ] Telegram bot (create via @BotFather)
- [ ] CJ Dropshipping account

---

## Installation

### Step 1: Download
Download the latest release from:
https://github.com/kimgebin/glowasia-automation/releases

### Step 2: Install
1. Extract the downloaded archive
2. Drag GLOWASIA Copilot.app to Applications
3. On first launch, macOS may ask for permission

### Step 3: Initial Launch
1. Open GLOWASIA Copilot
2. Dashboard will show no platforms connected
3. Go to Settings → Credentials to add platforms

---

## Initial Configuration

### Creating Telegram Bot
1. Open Telegram
2. Search @BotFather
3. Send `/newbot`
4. Follow prompts, copy the token

### Getting Chat ID
1. Search @userinfobot
2. Start conversation
3. Bot shows your numeric Chat ID

---

## Platform Setup

### Shopify

**Step 1: Create a Custom App in Shopify**
1. Go to Shopify Admin → Settings → Apps and sales channels
2. Click "Develop app" → "Create an app"
3. Name it "GLOWASIA Copilot"
4. Configure API scopes:
   - `read_products`
   - `write_products`
   - `read_orders`
   - `write_orders`
   - `read_inventory`
   - `write_inventory`

**Step 2: Get API Credentials**
1. In your app → "API credentials" tab
2. Click "Install app" to get Admin API access token
3. Copy the Admin API access token
4. Note your Shop URL (e.g., `yourstore.myshopify.com`)

**Step 3: Add to GLOWASIA Copilot**
1. Settings → Credentials → Add New
2. Select "Shopify"
3. Enter:
   - API Key = Admin API access token
   - Shop URL = yourstore.myshopify.com

---

### Shopee

**Step 1: Register as Shopee Partner**
1. Go to https://partner.shopeemobile.com/
2. Register as partner if not already registered
3. Wait for approval (usually 1-2 business days)

**Step 2: Get Credentials**
1. Log into Shopee Partner Portal
2. Go to My Account → API Keys
3. Copy Partner ID and Partner Key
4. Authorize your shop to get Shop ID

**Step 3: Add to GLOWASIA Copilot**
1. Settings → Credentials → Add New
2. Select "Shopee"
3. Enter:
   - Partner ID
   - Partner Key
   - Shop ID

---

### Lazada

**Step 1: Register as Lazada Partner**
1. Go to https://open.lazada.com/
2. Register as partner
3. Create application

**Step 2: Get Credentials**
1. Go to Console → Your App
2. Copy App Key and App Secret
3. Get your User ID from account settings

**Step 3: Add to GLOWASIA Copilot**
1. Settings → Credentials → Add New
2. Select "Lazada"
3. Enter:
   - API Key = App Key
   - Secret = App Secret
   - User ID

---

### Tokopedia

**Step 1: Register as Tokopedia Partner**
1. Go to https://developer.tokopedia.com/
2. Register as partner
3. Create app to get credentials

**Step 2: Get Credentials**
1. Go to Console → Your App
2. Copy Client ID and Client Secret
3. Set up redirect URI

**Step 3: Add to GLOWASIA Copilot**
1. Settings → Credentials → Add New
2. Select "Tokopedia"
3. Enter:
   - Client ID
   - Client Secret

---

### TikTok Shop

**Step 1: Register as TikTok Seller**
1. Go to https://seller-uk.tiktok.com/ (or your region)
2. Register as seller
3. Access Developer Portal

**Step 2: Get Credentials**
1. Go to Developer Portal → Apps
2. Create new app
3. Copy App Key and App Secret

**Step 3: Add to GLOWASIA Copilot**
1. Settings → Credentials → Add New
2. Select "TikTok Shop"
3. Enter:
   - App Key
   - App Secret

---

### CJ Dropshipping

**Step 1: Create CJ Account**
1. Go to https://www.cjdropshipping.com/
2. Register for account
3. Complete seller verification

**Step 2: Get API Credentials**
1. Log into CJ dashboard
2. Go to My Account → API
3. Generate API Key and Secret

**Step 3: Add to GLOWASIA Copilot**
1. Settings → Credentials → Add New
2. Select "CJ Dropshipping"
3. Enter:
   - API Key
   - Secret

---

### Google Sheets

**Step 1: Create Google Cloud Project**
1. Go to https://console.cloud.google.com/
2. Create new project
3. Enable Google Sheets API

**Step 2: Get API Credentials**
1. Go to Credentials → Service Account
2. Create new service account
3. Download JSON key file
4. Share your Google Sheet with the service account email

**Step 3: Add to GLOWASIA Copilot**
1. Settings → Credentials → Add New
2. Select "Google Sheets"
3. Enter:
   - Service Account Email
   - Private Key (from JSON file)
   - Spreadsheet ID (from URL)

---

### Midtrans

**Step 1: Create Midtrans Account**
1. Go to https://midtrans.com/
2. Register as merchant
3. Complete verification

**Step 2: Get Credentials**
1. Log into Midtrans Dashboard
2. Go to Settings → Access Keys
3. Copy Server Key and Client Key

**Step 3: Add to GLOWASIA Copilot**
1. Settings → Credentials → Add New
2. Select "Midtrans"
3. Enter:
   - Server Key
   - Client Key

---

## Testing

After configuring credentials:

1. Go to Dashboard
2. Click "Test Connection" on each platform
3. Green indicator = Connected
4. Red indicator = Check credentials and try again

### Test Checklist
- [ ] Shopify connection successful
- [ ] Shopee connection successful
- [ ] Telegram bot responding
- [ ] CJ Dropshipping product sync working
- [ ] Google Sheets inventory updating

---

## Next Steps

After setup, see [AUTOMATION.md](AUTOMATION.md) for workflow guides.

For troubleshooting, see [README.md](../README.md#-troubleshooting) troubleshooting section.