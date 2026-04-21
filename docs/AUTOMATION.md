# GLOWASIA Copilot - Automation Workflows

## Table of Contents
1. [Overview](#overview)
2. [Automation Modes](#automation-modes)
3. [Core Workflows](#core-workflows)
4. [Platform-Specific Workflows](#platform-specific-workflows)
5. [Notification System](#notification-system)
6. [Scheduling](#scheduling)
7. [Monitoring](#monitoring)

---

## Overview

GLOWASIA Copilot automates your dropshipping business by handling repetitive tasks across multiple platforms. The automation system works 24/7, polling for updates and executing predefined actions.

### Automation Goals

- ⏰ **Save Time**: No manual order processing
- 🔄 **Reduce Errors**: Automated actions are consistent
- 📈 **Scale Faster**: Handle unlimited orders without burnout
- 🔔 **Stay Informed**: Real-time notifications for important events

---

## Automation Modes

### Manual Mode

- Start/stop automation with a toggle button
- See each action as it happens in Activity Log
- Full control over when automation runs

**Best for**: Testing, initial setup, troubleshooting

### Auto-Pilot Mode

- Automation runs continuously
- Orders processed automatically
- Notifications sent for major events
- System checks for updates periodically

**Best for**: 24/7 store operations

### Scheduled Mode

- Run automation at specific times
- Daily summary at 8 PM
- Weekly analytics on Sundays
- Custom scheduling for each workflow

**Best for**: Regular reports and batch operations

---

## Core Workflows

### Workflow 1: Auto Order Import

**Purpose**: Automatically detect and import orders from all connected platforms

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│  Platforms  │────▶│  GLOWASIA    │────▶│  Google     │
│  (Orders)   │     │  Copilot     │     │  Sheets     │
└─────────────┘     └──────────────┘     └─────────────┘
                           │
                           ▼
                    ┌──────────────┐
                    │  Telegram    │
                    │  Notification│
                    └──────────────┘
```

#### Steps

1. **Poll** all connected platforms every 30 seconds
2. **Detect** new orders by comparing order IDs
3. **Extract** order details (customer, products, payment status)
4. **Log** to Google Sheets with status "Pending"
5. **Notify** via Telegram: "New order: #12345 - $149.99"

#### Configuration

- Poll interval: 30s (configurable, min 10s)
- Notification: ON/OFF
- Auto-retry: 3 attempts on failure

---

### Workflow 2: Auto Fulfillment

**Purpose**: Automatically fulfill orders through CJ Dropshipping

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│  Shopify    │────▶│  GLOWASIA    │────▶│  CJ         │
│  (New Order) │     │  Copilot     │     │  Dropshipping│
└─────────────┘     └──────────────┘     └─────────────┘
                           │
                    ┌──────┴──────┐
                    ▼             ▼
             ┌───────────┐  ┌───────────┐
             │  Fetch    │  │  Update   │
             │  Tracking │  │  Shopify  │
             └───────────┘  └───────────┘
                    │
                    ▼
             ┌───────────┐
             │  Notify   │
             │  Customer │
             └───────────┘
```

#### Steps

1. **Detect** new order in Shopify
2. **Check** if payment is confirmed
3. **Login** to CJ Dropshipping
4. **Create** CJ order with customer shipping details
5. **Track** CJ order status
6. **Fetch** tracking number when shipped
7. **Update** Shopify fulfillment status
8. **Notify** customer via Telegram

#### Supported Platforms

- Shopify → CJ
- Shopee → CJ
- Lazada → CJ
- Tokopedia → CJ
- TikTok Shop → CJ

---

### Workflow 3: Auto Product Import

**Purpose**: Import products from CJ Dropshipping to your stores

```
┌─────────────┐     ┌──────────────┐     ┌─────────────────┐
│  CJ         │────▶│  GLOWASIA    │────▶│  Shopify        │
│  Products   │     │  Copilot     │     │  (New Products) │
└─────────────┘     └──────────────┘     └─────────────────┘
                           │
                    ┌──────┴──────┐
                    ▼             ▼
             ┌───────────┐  ┌───────────┐
             │  Price    │  │  Auto     │
             │  Markup   │  │  Publish  │
             └───────────┘  └───────────┘
```

#### Steps

1. **Browse** CJ product catalog
2. **Select** products to import
3. **Set** price markup rules:
   - Fixed markup: +$5
   - Percentage markup: +50%
   - Tiered pricing by cost
4. **Select** target platforms
5. **Import** products with descriptions and images
6. **Auto-publish** to selected stores

#### Markup Rule Examples

| Cost | Markup Type | Value | Final Price |
|------|--------------|-------|-------------|
| $10 | +50% | 0.5 | $15.00 |
| $10 | +Fixed | $5 | $15.00 |
| $10-$20 | Tiered | +30% | $13-$26 |
| $10 | +75% | 0.75 | $17.50 |

---

### Workflow 4: Auto Price Sync

**Purpose**: Keep prices synchronized across all platforms

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│  Shopify    │────▶│  GLOWASIA    │◀────│  Shopee     │
│  (Price $20) │     │  Copilot     │     │  (Price $20) │
└─────────────┘     └──────────────┘     └─────────────┘
                           │
                           ▼
                    ┌─────────────┐
                    │  Compare &  │
                    │  Update     │
                    └─────────────┘
```

#### Steps

1. **Monitor** product prices on all platforms
2. **Detect** price changes on source (CJ or primary store)
3. **Calculate** new prices using markup rules
4. **Update** all connected platforms
5. **Log** changes to Activity Log
6. **Notify** via Telegram for large changes

#### Sync Options

- **One-way**: Source → Targets (recommended)
- **Two-way**: Bidirectional sync
- **Manual**: Only when triggered

---

### Workflow 5: Auto Stock Sync

**Purpose**: Update inventory levels across all platforms

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│  CJ         │────▶│  GLOWASIA    │────▶│  All Stores │
│  (Stock:10) │     │  Copilot     │     │  (Stock:10) │
└─────────────┘     └──────────────┘     └─────────────┘
```

#### Steps

1. **Fetch** stock levels from CJ Dropshipping
2. **Map** CJ SKUs to platform SKUs
3. **Calculate** available stock (CJ stock - pending orders)
4. **Update** inventory on all platforms
5. **Set** "Out of Stock" status when quantity = 0
6. **Notify** when critical stock threshold reached

#### Stock Thresholds

| Threshold | Action |
|-----------|--------|
| 0 | Mark out of stock |
| 5 | Low stock alert |
| 10 | Warning notification |

---

## Platform-Specific Workflows

### Shopify Workflow

#### Order Processing
1. Poll orders every 30 seconds
2. Filter by payment status (paid only)
3. Extract customer and product details
4. Calculate shipping costs
5. Update order status in Google Sheets

#### Fulfillment
1. Get fulfillment status
2. When shipped, capture tracking number
3. Update Google Sheets with tracking
4. Send Telegram notification

### Shopee Workflow

#### Auto-Listing
1. Select products from CJ catalog
2. Generate Shopee-compatible descriptions
3. Upload images (auto-resize for Shopee)
4. Set pricing with Shopee fees considered
5. Publish to Shopee store

#### Order Sync
1. Poll Shopee orders every 30 seconds
2. Sync status with Google Sheets
3. Handle Shopee-specific status codes

### Lazada Workflow

#### Multi-Country Listing
1. Select products to list
2. Generate descriptions per country (ID, MY, TH, PH, SG)
3. Adjust pricing per country (currency, fees)
4. Upload to each Lazada marketplace
5. Track per-country performance

### Tokopedia Workflow

#### Order Management
1. Poll Tokopedia orders
2. Sync with Google Sheets pipeline
3. Handle Tokopedia-specific statuses

### TikTok Shop Workflow

#### Viral Content Integration
1. Sync product catalog with TikTok Shop
2. Monitor TikTok order metrics
3. Auto-generate performance reports

### Telegram Bot Commands

| Command | Action |
|---------|--------|
| `/start` | Welcome message, bot status |
| `/orders` | List recent orders |
| `/status` | System status |
| `/stats` | Daily/weekly statistics |
| `/help` | List available commands |
| `/pause` | Pause automation |
| `/resume` | Resume automation |

---

## Notification System

### Telegram Notifications

#### New Order Alert
```
🛒 NEW ORDER

Order #: #12345
Customer: John Doe
Total: $149.99
Products: 2 items
Platform: Shopify

📦 Processing...
```

#### Order Shipped Alert
```
📦 ORDER SHIPPED

Order #: #12345
Tracking: J&T ETB123456789
Carrier: J&T Express
ETA: 7-12 days

👤 Customer notified
```

#### Daily Summary (8 PM)
```
📊 DAILY SUMMARY

Orders Today: 15
Revenue: $2,345.67
Shipped: 8
Delivered: 12

Top Product: K-Beauty Serum
Pending Orders: 3

🤖 GLOWASIA Copilot running normally
```

### Notification Settings

| Event | Default | Configurable |
|-------|---------|--------------|
| New Order | ON | Yes |
| Order Shipped | ON | Yes |
| Delivery Confirmed | ON | Yes |
| Daily Summary | ON | Yes |
| Weekly Analytics | ON | Yes |
| System Errors | ON | Yes (cannot disable) |

---

## Scheduling

### Default Schedule

| Time | Task |
|------|------|
| Every 30s | Poll for new orders |
| Every 5 min | Check CJ fulfillment status |
| 8:00 PM | Daily summary notification |
| Sunday 9 PM | Weekly analytics |
| Every 6 hours | Auto-update check |

### Custom Scheduling

1. Go to Settings → Automation
2. Click **Schedule**
3. Configure:
   - Poll intervals per platform
   - Notification times
   - Batch operation times
4. Save schedule

---

## Monitoring

### Activity Log

The Activity Log shows all automation actions in real-time:

```
[04:30:15] ✓ Shopify: Detected 2 new orders
[04:30:16] ✓ Order #12345 added to Google Sheets
[04:30:17] ✓ Telegram: New order notification sent
[04:30:18] ✓ CJ: Order #12345 created successfully
[04:35:22] ✓ CJ: Tracking #J&T123456789 received
[04:35:23] ✓ Shopify: Fulfillment updated
[04:35:24] ✓ Customer notified via Telegram
```

### Dashboard Stats

| Metric | Description |
|--------|-------------|
| Orders Today | New orders in last 24 hours |
| Revenue Today | Total revenue in last 24 hours |
| Shipped Today | Orders shipped in last 24 hours |
| Delivered Today | Orders delivered in last 24 hours |
| Pending | Orders awaiting fulfillment |
| System Status | Connected/Disconnected indicators |

### Health Indicators

| Status | Color | Meaning |
|--------|-------|---------|
| Connected | Green | Platform responding normally |
| Warning | Yellow | Platform responding slowly |
| Error | Red | Platform not responding |
| Paused | Gray | Automation paused |
| Unknown | Gray with ? | Status not yet checked |

---

## Troubleshooting

### Automation Not Running

**Check**:
1. Is the toggle set to ON?
2. Are credentials configured?
3. Is the app window open?

**Solutions**:
1. Turn toggle OFF then ON
2. Check credentials in Settings
3. Restart the app

### Orders Not Syncing

**Check**:
1. Is Shopify credential valid?
2. Is Google Sheets ID correct?
3. Are there API rate limits?

**Solutions**:
1. Test connection in Settings
2. Re-authorize Shopify
3. Wait for rate limit reset

### Telegram Not Receiving

**Check**:
1. Is Telegram credential configured?
2. Is the bot token valid?
3. Is the chat ID correct?

**Solutions**:
1. Test Telegram connection
2. Send test message via @BotFather
3. Re-add chat ID

---

## Next Steps

- Read [SETUP.md](SETUP.md) for initial configuration
- Read [CREDENTIALS.md](CREDENTIALS.md) for credential management
- Read [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for issues

---

## Need Help?

1. Check existing [GitHub Issues](https://github.com/kimgebin/glowasia-automation/issues)
2. Create a new issue with automation logs
