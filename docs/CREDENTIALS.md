# Credentials Management

## Overview

GLOWASIA Copilot stores all credentials in a local SQLite database that persists across app updates. This means your API keys and tokens remain safe even when you update or reinstall the application.

## Database Location

```
~/.local/share/glowasia-automation/credentials.db
```

The database file is stored in the user's local application data directory, which:
- Persists across app updates
- Survives app reinstalls
- Is protected by macOS file permissions
- Is specific to your user account

---

## Supported Platforms

| Platform | Required Fields |
|----------|----------------|
| Shopify | API Key, Shop URL |
| Shopee | Partner ID, Partner Key, Shop ID |
| Lazada | API Key, Secret, User ID |
| Tokopedia | Client ID, Client Secret |
| TikTok Shop | App Key, App Secret |
| Telegram | Bot Token, Chat ID |
| CJ Dropshipping | API Key, Secret |
| Google Sheets | Service Account Email, Private Key, Spreadsheet ID |
| Midtrans | Server Key, Client Key |

---

## Adding Credentials

### Step-by-Step Guide

1. **Open Settings**
   - Click the ⚙️ icon in the sidebar
   - Or use keyboard shortcut `Cmd +,`

2. **Navigate to Credentials Tab**
   - Click the 🔐 icon in the settings panel

3. **Add New Credential**
   - Click "Add New" button
   - Select platform from dropdown
   - Fill in required fields
   - Click "Save"

### Field Validation

Each platform validates its fields differently:
- **Shopify**: Validates shop URL format (must end in .myshopify.com or .shopify.com)
- **Shopee**: Validates Partner ID is numeric
- **Telegram**: Validates bot token format (must contain `:`)
- **API Keys**: Validates minimum length requirements

---

## Editing Credentials

1. Go to Settings → Credentials
2. Click on the credential entry you want to edit
3. Modify the fields
4. Click "Save" to update

---

## Deleting Credentials

1. Go to Settings → Credentials
2. Click the trash icon next to the credential
3. Confirm deletion
4. Credential is permanently removed

**Warning**: Deleting credentials cannot be undone. Make sure to export a backup first if you might need these credentials again.

---

## Export/Import

### Why Export?

- Backup credentials before reinstalling macOS
- Transfer credentials to a new computer
- Keep a secure copy of all API keys
- Migrate to a new installation

### Export Backup

1. Go to Settings → Credentials
2. Click "Export Backup"
3. Choose save location
4. Save as JSON file

**Important**: Export files are unencrypted. Keep them safe and secure.

### Import Backup

1. Go to Settings → Credentials
2. Click "Import Backup"
3. Select JSON file
4. Confirm import

**Note**: Importing will merge with existing credentials. If a platform credential already exists, it will be updated with the imported data.

---

## Security Best Practices

### 🔐 Local Storage

- Credentials are stored locally only
- Never uploaded to external servers
- Protected by macOS file permissions
- Each platform only receives its own required authentication data

### ✅ Recommended Practices

1. **Never share credentials** with anyone, including support personnel
2. **Use separate accounts** for development and production environments
3. **Regularly export backups** and store them securely
4. **Delete old credentials** for platforms you no longer use
5. **Rotate API keys** periodically (every 3-6 months)

### 🚫 What NOT to Do

- Don't email credentials to anyone
- Don't share screenshots of credential fields
- Don't store credentials in plain text files
- Don't use the same API key across multiple applications

---

## Troubleshooting

### Credential Not Saving

**Solutions:**
1. Check write permissions for `~/.local/share/glowasia-automation/`
2. Ensure no other instance of the app is running
3. Try deleting the database file and re-adding credentials
4. Check available disk space

### Connection Failed After Adding Credentials

**Solutions:**
1. Verify credentials are correct (no typos)
2. Check if platform API is down (status pages)
3. Try re-adding credentials
4. For Shopify: Ensure API token hasn't expired
5. Check internet connection

### Export File Corrupted

**Solutions:**
1. Try re-exporting the file
2. Use the most recent backup
3. Manually re-enter credentials if backup is unavailable

---

## Database Schema

The credentials database uses the following structure:

```sql
-- credentials table
CREATE TABLE credentials (
    id TEXT PRIMARY KEY,
    platform TEXT NOT NULL,
    name TEXT,
    data TEXT NOT NULL, -- JSON encrypted credentials
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- app_state table
CREATE TABLE app_state (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
```

All credential data is stored in the `data` column as encrypted JSON.

---

*For setup guides for each platform, see [SETUP.md](SETUP.md)*