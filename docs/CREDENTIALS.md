# GLOWASIA Copilot - Credentials Management

## Table of Contents
1. [Overview](#overview)
2. [How Credentials Work](#how-credentials-work)
3. [Supported Platforms](#supported-platforms)
4. [Adding Credentials](#adding-credentials)
5. [Managing Credentials](#managing-credentials)
6. [Security Best Practices](#security-best-practices)
7. [Backup & Restore](#backup--restore)
8. [Troubleshooting](#troubleshooting)

---

## Overview

GLOWASIA Copilot uses a **persistent credentials database** that stores all your API keys, tokens, and sensitive information securely on your local machine. This database survives app updates - you never need to re-enter credentials when updating the application.

### Key Features

- ✅ **Persistent Storage**: Credentials survive app updates
- ✅ **Local Encryption**: All data stored locally in SQLite
- ✅ **Platform Support**: 10+ platforms supported
- ✅ **Multiple Accounts**: Multiple accounts per platform
- ✅ **Export/Import**: Backup and restore functionality
- ✅ **Secure Delete**: Securely remove credentials when needed

---

## How Credentials Work

### Database Location

```
~/.local/share/glowasia-automation/
├── credentials.db    ← All API keys & tokens (PERSISTENT)
├── app_state.json    ← App settings
└── logs/
    └── activity.log
```

### Data Structure

Each credential entry stores:

| Field | Description |
|-------|-------------|
| `platform` | Platform name (shopify, cj_dropshipping, etc.) |
| `account_name` | Friendly name for this account |
| `api_key` | Primary API key or username |
| `api_secret` | API secret or password |
| `access_token` | OAuth access token (if applicable) |
| `refresh_token` | OAuth refresh token (if applicable) |
| `shop_url` | Store URL for Shopify-style platforms |
| `additional_data` | JSON blob for platform-specific data |
| `created_at` | When credential was added |
| `updated_at` | Last modification time |

### Persistence Guarantee

The credentials database is stored in your user's local directory, NOT inside the application bundle. This means:

1. **App Updates**: Updating the app does NOT affect credentials
2. **App Reinstall**: Reinstalling the app does NOT affect credentials
3. **App Delete**: Deleting the app does NOT affect credentials
4. **System Upgrades**: macOS upgrades do NOT affect credentials

---

## Supported Platforms

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

---

## Adding Credentials

### Via Settings UI (Recommended)

#### Step 1: Open Settings

1. Click the **Settings** (⚙️) icon in the sidebar
2. Or press **⌘ + ,**

#### Step 2: Navigate to Credentials

1. Click the **Credentials** (🔐) tab
2. You'll see a list of platforms

#### Step 3: Add New Credential

1. Click the **+** button or "Add Credential"
2. Select the **Platform** from dropdown
3. Enter an **Account Name** (e.g., "My Shopify Store")
4. Fill in the required fields
5. Click **Save**

#### Step 4: Verify

1. Click **Test Connection** next to the credential
2. Green checkmark = Success
3. Red X = Check credentials

---

### Via Import (Advanced)

You can import credentials from a JSON backup file.

#### JSON Format

```json
{
  "version": 1,
  "credentials": [
    {
      "platform": "shopify",
      "account_name": "My Store",
      "api_key": "shpat_xxxxx",
      "shop_url": "mystore.myshopify.com"
    },
    {
      "platform": "telegram",
      "account_name": "Main Bot",
      "api_key": "123456:ABCdef",
      "additional_data": {
        "chat_id": "123456789"
      }
    }
  ]
}
```

#### Import Steps

1. Settings → Credentials tab
2. Click **Import** button
3. Select your `.json` backup file
4. Confirm import
5. All credentials are added

---

## Managing Credentials

### Editing Credentials

1. Go to Settings → Credentials
2. Click on the credential entry
3. Modify fields
4. Click **Save**

### Deleting Credentials

#### Normal Delete

1. Go to Settings → Credentials
2. Hover over credential entry
3. Click **Delete** (trash icon)
4. Confirm deletion

#### Secure Delete (Recommended when selling device)

1. Go to Settings → Credentials
2. Hover over credential entry
3. Click **Delete** while holding **⌘ + ⇧**
4. Confirm secure deletion
5. Credential is overwritten before deletion

### Multiple Accounts

You can add multiple accounts for the same platform:

1. Go to Settings → Credentials
2. Click **Add** with a different account name
3. Each account is listed separately
4. Select which account to use for automation

---

## Security Best Practices

### 🔒 Essential Rules

1. **Never share credentials** with anyone
2. **Use separate accounts** for development and production
3. **Enable 2FA** on all platform accounts
4. **Regularly rotate** API keys and tokens
5. **Export backups** after adding new credentials

### 🔑 Credential Storage

- All credentials are stored locally in `credentials.db`
- The database is NOT encrypted at rest (standard SQLite)
- Physical access to your device = access to credentials
- Always use screen lock when leaving device unattended

### 📤 Sharing Guidelines

**NEVER share:**
- API keys or tokens
- Database files (`credentials.db`)
- Export JSON files containing credentials

**SAFE to share:**
- Screenshots of the dashboard (no credentials visible)
- Anonymous error messages
- Feature suggestions

---

## Backup & Restore

### Exporting Credentials

#### When to Export

- Before updating the app
- After adding new credentials
- Weekly/monthly routine

#### How to Export

1. Go to Settings → Credentials
2. Click **Export** button
3. Choose save location
4. File is saved as `glowasia-credentials-backup-YYYY-MM-DD.json`

#### What Gets Exported

```json
{
  "version": 1,
  "exported_at": "2026-04-22T04:00:00Z",
  "credentials": [
    {
      "platform": "shopify",
      "account_name": "My Store",
      "api_key": "shpat_xxxxx",
      "shop_url": "mystore.myshopify.com",
      "additional_data": {}
    }
  ]
}
```

### Importing Credentials

1. Go to Settings → Credentials
2. Click **Import** button
3. Select backup `.json` file
4. Review credentials to import
5. Click **Confirm Import**
6. Existing credentials are merged (duplicates skipped)

### Restoring After Fresh Install

1. Install the new version of GLOWASIA Copilot
2. Launch the app
3. Go to Settings → Credentials
4. Click **Import**
5. Select your backup file
6. All credentials are restored

---

## Troubleshooting

### Credentials Not Saving

**Symptoms**: Click save but credentials don't persist

**Solutions**:
1. Check write permissions for `~/.local/share/glowasia-automation/`
2. Ensure no other instance of the app is running
3. Try deleting `credentials.db` and re-adding (backup first!)
4. Restart the app

### Connection Test Fails

**Symptoms**: Test Connection returns red X

**Solutions**:
1. Verify credentials are correct (check for typos)
2. Check internet connection
3. Verify platform API is operational
4. Try re-generating API keys on the platform
5. Check if platform requires API whitelist update

### Duplicate Credentials

**Symptoms**: Same platform shows multiple times

**Solutions**:
1. Go to Settings → Credentials
2. Identify duplicate entries
3. Delete extras (keep most recent)
4. Use account names to differentiate

### Lost Credentials

**Symptoms**: Forgot credentials, need to retrieve

**Solutions**:
1. Check if you have a backup file
2. Retrieve from platform dashboard (Shopify, CJ, etc.)
3. Generate new API keys if needed
4. Re-add to GLOWASIA Copilot

---

## Database Details

### Schema

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

### Direct Database Access

**For debugging only** - DO NOT modify directly:

```bash
# View database
sqlite3 ~/.local/share/glowasia-automation/credentials.db "SELECT * FROM credentials;"

# Backup database
cp ~/.local/share/glowasia-automation/credentials.db ~/Desktop/credentials-backup.db
```

---

## Need Help?

1. Check [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for common issues
2. Check existing [GitHub Issues](https://github.com/kimgebin/glowasia-automation/issues)
3. Create a new issue with details
