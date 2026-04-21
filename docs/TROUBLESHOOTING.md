# GLOWASIA Copilot - Troubleshooting Guide

## Table of Contents
1. [Getting Help](#getting-help)
2. [Installation Issues](#installation-issues)
3. [Launch Issues](#launch-issues)
4. [Connection Issues](#connection-issues)
5. [Automation Issues](#automation-issues)
6. [Credentials Issues](#credentials-issues)
7. [Performance Issues](#performance-issues)
8. [Error Messages](#error-messages)

---

## Getting Help

### Before You Start

When reporting an issue, gather this information:

- [ ] macOS version (Apple Menu → About This Mac)
- [ ] App version (GLOWASIA Copilot → About)
- [ ] Steps to reproduce
- [ ] Expected behavior
- [ ] Actual behavior
- [ ] Screenshots (if applicable)

### Where to Get Help

1. **This Guide**: Search for your error message or symptom
2. **[GitHub Issues](https://github.com/kimgebin/glowasia-automation/issues)**: Search existing issues
3. **New Issue**: Create with the information above

---

## Installation Issues

### Issue: Download Failed

**Symptoms**: Cannot download the app from GitHub

**Solutions**:
1. Check internet connection
2. Try a different browser
3. Use command line download:
   ```bash
   curl -L -o glowasia.zip "https://github.com/kimgebin/glowasia-automation/releases/latest/download/glowasia-automation_vX.X.X_x86_64-apple-darwin.tar.gz"
   ```
4. Check if antivirus is blocking the download
5. Try again in incognito/private mode

---

### Issue: Cannot Open Because "App is Damaged"

**Symptoms**: macOS says the app is damaged or cannot be opened

**Solutions**:

#### Solution 1: Remove Quarantine Attribute (Most Common)

```bash
xattr -cr "/Applications/GLOWASIA Copilot.app"
```

#### Solution 2: Allow in System Preferences

1. Go to **System Preferences** → **Security & Privacy**
2. Click the lock to make changes
3. Look for "GLOWASIA Copilot was blocked" message
4. Click **Open Anyway**

#### Solution 3: Check Gatekeeper Settings

```bash
# Check current gatekeeper status
spctl --status

# If disabled, enable it temporarily and use Solution 1
sudo spctl --enable
```

---

### Issue: App Does Not Support This Platform

**Symptoms**: "This version of macOS is not supported"

**Solutions**:
1. Update macOS to 10.15 (Catalina) or later
2. Or build from source with older SDK (see DEVELOPMENT.md)

---

## Launch Issues

### Issue: App Won't Launch

**Symptoms**: Double-clicking the app does nothing

**Solutions**:

1. **Check macOS Version**
   ```bash
   sw_vers
   ```
   Must be 10.15 or later

2. **Check for Multiple Instances**
   ```bash
   # Kill any running instances
   pkill "GLOWASIA Copilot"
   ```

3. **Try Launching from Terminal**
   ```bash
   open "/Applications/GLOWASIA Copilot.app"
   ```
   Note any error messages

4. **Reinstall the App**
   ```bash
   # Remove existing
   rm -rf "/Applications/GLOWASIA Copilot.app"
   
   # Re-copy from downloaded archive
   ```

5. **Check Console Logs**
   ```bash
   # Open Console app and search for "glowasia"
   ```

---

### Issue: App Launches But Window is Blank

**Symptoms**: Window opens but content is white/empty

**Solutions**:

1. **Wait for Loading**
   - First launch may take 10-20 seconds

2. **Force Quit and Relaunch**
   ```bash
   pkill "GLOWASIA Copilot"
   open "/Applications/GLOWASIA Copilot.app"
   ```

3. **Check GPU Settings**
   - System Preferences → Displays → Graphics
   - Try unchecking "Automatic graphics switching"

4. **Reset App Data**
   ```bash
   # Backup and remove app state
   mkdir ~/Desktop/glowasia-backup
   cp -r ~/Library/Application\ Support/com.glowasia.copilot ~/Desktop/glowasia-backup/
   rm -rf ~/Library/Application\ Support/com.glowasia.copilot
   ```

---

### Issue: App Crashes on Launch

**Symptoms**: App starts then immediately crashes

**Solutions**:

1. **Check Crash Reports**
   ```bash
   # Find recent crash logs
   ls -la ~/Library/Logs/DiagnosticReports/ | grep glowasia
   cat ~/Library/Logs/DiagnosticReports/glowasia-*.crash
   ```

2. **Safe Mode**
   - Restart Mac holding Shift key
   - Launch GLOWASIA Copilot
   - If it works, the issue is a startup item or extension

3. **Reinstall**
   ```bash
   rm -rf "/Applications/GLOWASIA Copilot.app"
   # Redownload and reinstall
   ```

---

## Connection Issues

### Issue: Platform Connection Failed

**Symptoms**: Test Connection returns red X for Shopify/Shopee/etc.

**Solutions**:

#### General Checks
1. Verify internet connection
2. Check if platform API is operational (platform status pages)
3. Verify credentials are correct
4. Try re-entering credentials

#### Platform-Specific

**Shopify**:
1. Check if API token is expired
2. Regenerate token in Shopify Admin
3. Verify API scopes are correct

**Shopee**:
1. Verify Partner ID and Partner Key
2. Check if shop is authorized
3. Verify shop_id is correct

**CJ Dropshipping**:
1. Check if account is active
2. Verify affiliate token
3. Check if CJ website is accessible

**Telegram**:
1. Test bot via @BotFather
2. Verify bot token format: `123456789:ABCdef...`
3. Check chat_id is correct
4. Send /start to your bot

**Google Sheets**:
1. Verify sheet ID in URL
2. Check sharing settings (anyone with link can edit)
3. Ensure sheet is not deleted

---

### Issue: Telegram Notifications Not Working

**Symptoms**: No notifications despite configuration

**Solutions**:

1. **Test Bot Manually**
   - Send a message to your bot via Telegram
   - If no response, bot token is wrong

2. **Check Chat ID**
   - Use @userinfobot to get your Chat ID
   - Enter without the "-" prefix for negative IDs

3. **Verify Configuration**
   ```
   Settings → Credentials → Telegram
   - Bot Token: should match @BotFather token
   - Chat ID: your numeric chat ID
   ```

4. **Test Connection**
   - Click "Test Connection" in Credentials
   - Should receive test message

5. **Check Notification Settings**
   ```
   Settings → Notifications
   - Enable notifications is ON
   - Events are selected
   ```

---

### Issue: Google Sheets Not Updating

**Symptoms**: Orders appear but don't update in Sheets

**Solutions**:

1. **Check Sheet ID**
   - Sheet ID is between `/d/` and `/edit` in URL
   - Example: `https://docs.google.com/spreadsheets/d/ABC123.../edit`
   - ID is: `ABC123...`

2. **Check Sharing Permissions**
   - Sheet must be shared with "Anyone with link can edit"
   - Or share with specific email from Google account

3. **Check Sheet Structure**
   - Default expected sheets: "Orders", "Products", "Settings"
   - Create missing sheets if needed

4. **Check for Duplicate Ranges**
   - Ensure no locked cells blocking writes

---

## Automation Issues

### Issue: Automation Toggle Won't Turn ON

**Symptoms**: Toggle stays gray/off

**Solutions**:

1. **Check Credentials**
   - At least one platform credential must be configured
   - Settings → Credentials → Check all platforms

2. **Check Platform Status**
   - At least one platform must show "Connected"
   - Test connections and fix failed ones

3. **View Error Details**
   - Dashboard shows tooltip on hover
   - Activity Log may show specific error

---

### Issue: Orders Not Being Detected

**Symptoms**: New orders not appearing in Activity Log

**Solutions**:

1. **Verify Shopify Connection**
   - Settings → Credentials → Test Shopify
   - Check if orders exist in Shopify Admin

2. **Check Poll Interval**
   - Default is 30 seconds
   - May take up to 2x interval for detection

3. **Check Order Status**
   - Only "paid" orders are processed
   - Pending payment orders are skipped

4. **Check Automation Mode**
   - Verify Auto-Pilot mode is enabled
   - Check if Manual mode and toggle is ON

5. **Check API Rate Limits**
   - Too many requests may trigger temporary block
   - Wait 5 minutes and try again

---

### Issue: Order Fulfillment Not Working

**Symptoms**: CJ orders not being created

**Solutions**:

1. **Verify CJ Credentials**
   - Settings → Credentials → Test CJ
   - Check if CJ account is active

2. **Check Customer Address**
   - CJ may reject incomplete addresses
   - Verify shipping address format

3. **Check Product Availability**
   - Product may be out of stock on CJ
   - Try different product

4. **Check CJ Website**
   - Manual login to CJ to verify account status
   - CJ may have maintenance

5. **Check API Logs**
   - Activity Log shows CJ API errors
   - Look for specific error messages

---

### Issue: Tracking Numbers Not Fetched

**Symptoms**: CJ shipped but no tracking number

**Solutions**:

1. **Wait for Processing**
   - CJ typically takes 24-48 hours to ship
   - Tracking may take additional 24 hours

2. **Check CJ Order Status**
   - Log in to CJ manually
   - Check order status in CJ dashboard

3. **Check Fulfillment Status**
   - Some platforms show "fulfilled" before tracking
   - Tracking may sync separately

---

## Credentials Issues

### Issue: Credentials Not Saving

**Symptoms**: Click Save but credentials don't persist

**Solutions**:

1. **Check Write Permissions**
   ```bash
   ls -la ~/.local/share/glowasia-automation/
   # Should show read/write for your user
   ```

2. **Check Disk Space**
   - Low disk space may prevent saving
   - Free up space and try again

3. **Check for Running Instance**
   - Another instance may have lock on database
   - Close all instances and retry

4. **Reset Database**
   ```bash
   # Backup first
   cp ~/.local/share/glowasia-automation/credentials.db ~/Desktop/
   
   # Remove and let app recreate
   rm ~/.local/share/glowasia-automation/credentials.db
   ```

---

### Issue: Credential Database Locked

**Symptoms**: "Database is locked" error

**Solutions**:

1. **Close All Instances**
   ```bash
   pkill "GLOWASIA Copilot"
   ```

2. **Remove Lock File**
   ```bash
   rm -f ~/.local/share/glowasia-automation/credentials.db-wal
   rm -f ~/.local/share/glowasia-automation/credentials.db-shm
   ```

3. **Restart App**

---

### Issue: Forgot Credentials

**Symptoms**: Lost API keys or tokens

**Solutions**:

1. **Retrieve from Platform**
   - Shopify: Regenerate in Partners Dashboard
   - CJ: Reset in CJ account settings
   - Telegram: Create new bot via @BotFather

2. **Check Backup Files**
   - Export backup from Settings → Credentials → Export
   - Import from backup file

3. **Manual Recovery**
   - Each platform has its own credential recovery process
   - Contact platform support if needed

---

## Performance Issues

### Issue: App Running Slowly

**Symptoms**: UI lags, slow response

**Solutions**:

1. **Close Other Apps**
   - Free up RAM by closing unused applications

2. **Check Activity Monitor**
   ```bash
   # Open Activity Monitor
   open -a Activity\ Monitor
   
   # Check CPU and Memory for GLOWASIA Copilot
   ```

3. **Reduce Poll Frequency**
   - Settings → Automation → Poll Interval
   - Increase from 30s to 60s or higher

4. **Clear Logs**
   ```bash
   # Backup and clear large log files
   cp ~/.local/share/glowasia-automation/logs/activity.log ~/Desktop/
   echo "" > ~/.local/share/glowasia-automation/logs/activity.log
   ```

---

### Issue: High CPU Usage

**Symptoms**: Fan running fast, battery drain

**Solutions**:

1. **Normal Behavior**
   - Automation polling uses CPU
   - This is expected behavior

2. **Check for Infinite Loop**
   - Activity Log shows rapid repeated actions
   - May indicate API error causing retry loop

3. **Pause Automation**
   - Turn automation OFF when not needed
   - Use scheduled mode instead

---

## Error Messages

### "Failed to connect to Shopify"

| Cause | Solution |
|-------|----------|
| Invalid API token | Regenerate token in Shopify Admin |
| Network issue | Check internet connection |
| Rate limited | Wait 5 minutes, try again |
| Shop URL wrong | Verify `yourstore.myshopify.com` format |

### "Database error"

| Cause | Solution |
|-------|----------|
| No write permission | Check `~/.local/share/glowasia-automation/` permissions |
| Corrupted database | Backup and delete database, restart app |
| Disk full | Free up disk space |

### "Telegram bot token invalid"

| Cause | Solution |
|-------|----------|
| Wrong token format | Token should be `123456789:ABCdef...` |
| Bot deleted | Create new bot via @BotFather |
| Token revoked | Generate new token in @BotFather |

### "Google Sheets ID not found"

| Cause | Solution |
|-------|----------|
| Wrong Sheet ID | Copy ID from spreadsheet URL |
| Sheet deleted | Restore from Google Drive trash |
| Wrong sharing settings | Enable "Anyone with link can edit" |

### "CJ API error: Account suspended"

| Cause | Solution |
|-------|----------|
| CJ account issue | Log in to CJ website, check status |
| Payment overdue | Complete CJ payment setup |
| Policy violation | Contact CJ support |

---

## Getting More Help

### Enable Debug Mode

1. Go to Settings → Advanced
2. Enable "Debug Mode"
3. Restart app
4. Collect logs from Activity Log

### Export Logs

1. Go to Settings → Advanced
2. Click "Export Logs"
3. Save to a known location
4. Include with bug report

### Create a GitHub Issue

When creating an issue, include:

```markdown
## Environment
- macOS Version: [e.g., 14.0]
- App Version: [e.g., 1.0.0]
- Device: [e.g., MacBook Pro 2021]

## Steps to Reproduce
1. [Step 1]
2. [Step 2]
3. [Step 3]

## Expected Behavior
[What should happen]

## Actual Behavior
[What actually happened]

## Logs
[Copy relevant logs from Activity Log]

## Screenshots
[If applicable]
```

---

## Status Indicators

| Indicator | Color | Meaning |
|-----------|-------|---------|
| ✅ Connected | Green | Platform responding normally |
| ⚠️ Warning | Yellow | Platform responding slowly or some errors |
| ❌ Error | Red | Platform not responding |
| ⏸️ Paused | Gray | Automation paused for this platform |
| ❓ Unknown | Gray with ? | Status not yet checked |
