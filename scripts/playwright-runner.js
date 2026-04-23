const { chromium } = require('playwright-extra');
const stealth = require('playwright-extra-plugin-stealth');

// ============ STEALTH LEVELS ============
const STEALTH_LEVELS = {
  BASIC: 1,        // Just stealth plugin (fast)
  MODERATE: 2,     // + Human delays, viewport spoofing
  AGGRESSIVE: 3,   // + Canvas, WebGL, font spoofing
  MAXIMUM: 4,      // + Proxy, session rotation
};

// Platform strictness mapping
const PLATFORM_STRICTNESS = {
  shopify: STEALTH_LEVELS.MODERATE,
  shopee: STEALTH_LEVELS.AGGRESSIVE,
  lazada: STEALTH_LEVELS.AGGRESSIVE,
  tokopedia: STEALTH_LEVELS.MAXIMUM,
  tiktok: STEALTH_LEVELS.MAXIMUM,
  cj: STEALTH_LEVELS.MODERATE,
  midtrans: STEALTH_LEVELS.BASIC,
};

// ============ UTILITY FUNCTIONS ============

function randomDelay(min = 50, max = 200) {
  const delay = Math.floor(Math.random() * (max - min + 1)) + min;
  return new Promise(resolve => setTimeout(resolve, delay));
}

function randomInt(min, max) {
  return Math.floor(Math.random() * (max - min + 1)) + min;
}

// ============ STEALTH TECHNIQUES ============

const TECHNIQUES = {
  // Level 1: Basic stealth
  applyBasicStealth: async (browser, options = {}) => {
    return browser;
  },

  // Level 2: Human-like behavior
  applyModerateStealth: async (page, options = {}) => {
    await page.addInitScript(() => {
      // Hide webdriver property
      Object.defineProperty(navigator, 'webdriver', {
        get: () => undefined,
        configurable: true
      });

      // Fake chrome runtime
      window.chrome = { runtime: {} };

      // Fake plugins (common ones)
      Object.defineProperty(navigator, 'plugins', {
        get: () => [1, 2, 3, 4, 5],
      });

      // Fake languages
      Object.defineProperty(navigator, 'languages', {
        get: () => ['en-US', 'en', 'id-ID', 'ms-MY', 'th-TH'],
      });
    });

    // Human-like delays
    page.setDefaultTimeout(30000);
    page.setDefaultNavigationTimeout(30000);

    return page;
  },

  // Level 3: Aggressive fingerprint spoofing
  applyAggressiveStealth: async (page, options = {}) => {
    await TECHNIQUES.applyModerateStealth(page, options);

    await page.addInitScript(() => {
      // ========== Canvas Fingerprint Spoofing ==========
      const origGetContext = HTMLCanvasElement.prototype.getContext;
      HTMLCanvasElement.prototype.getContext = function(type, ...args) {
        const context = origGetContext.call(this, type, ...args);
        if (type === '2d') {
          const origToDataURL = this.toDataURL;
          this.toDataURL = function(...args) {
            // Add tiny random noise to canvas
            const imageData = context.getImageData(0, 0, this.width, this.height);
            for (let i = 0; i < imageData.data.length; i += 4) {
              imageData.data[i] = Math.min(255, imageData.data[i] + (Math.random() * 0.5));
              imageData.data[i + 1] = Math.min(255, imageData.data[i + 1] + (Math.random() * 0.5));
              imageData.data[i + 2] = Math.min(255, imageData.data[i + 2] + (Math.random() * 0.5));
            }
            context.putImageData(imageData, 0, 0);
            return origToDataURL.apply(this, args);
          };
        }
        return context;
      };

      // ========== WebGL Fingerprint Spoofing ==========
      const origGetParameter = WebGLRenderingContext.prototype.getParameter;
      WebGLRenderingContext.prototype.getParameter = function(parameter) {
        // Spoof GPU vendor
        if (parameter === 37445) return 'Intel Inc.';
        if (parameter === 37446) return 'Intel Iris OpenGL Engine';
        // Spoof WebGL renderer
        if (parameter === 7937) return 'Apple M1 Pro';
        return origGetParameter.apply(this, arguments);
      };

      // ========== Font Spoofing ==========
      document.fonts.addFontFace = function() { return Promise.resolve(); };
      const fakeFonts = [
        'Arial', 'Arial Black', 'Comic Sans MS', 'Courier New', 'Georgia',
        'Helvetica', 'Impact', 'Lucida Console', 'Lucida Sans Unicode',
        'Palatino Linotype', 'Tahoma', 'Times New Roman', 'Trebuchet MS',
        'Verdana', 'MS Gothic', 'MS PGothic', 'Meiryo', 'Segoe UI'
      ];
      Object.defineProperty(Object.getPrototypeOf(document.fonts), 'status', {
        get: () => 'loaded',
      });

      // ========== Connection Spoofing ==========
      Object.defineProperty(navigator, 'connection', {
        get: () => ({
          effectiveType: '4g',
          downlink: randomInt(5, 20),
          rtt: randomInt(20, 100),
          saveData: false,
        }),
      });

      // ========== Device Memory Spoofing ==========
      Object.defineProperty(navigator, 'deviceMemory', {
        get: () => randomInt(4, 16),
      });

      // ========== Hardware Concurrency Spoofing ==========
      Object.defineProperty(navigator, 'hardwareConcurrency', {
        get: () => randomInt(4, 16),
      });

      // ========== Platform Spoofing ==========
      Object.defineProperty(navigator, 'platform', {
        get: () => 'MacIntel',
      });

      // ========== Do Not Track Spoofing ==========
      Object.defineProperty(navigator, 'doNotTrack', {
        get: () => '1',
      });
    });

    return page;
  },

  // Level 4: Maximum evasion
  applyMaximumStealth: async (page, options = {}) => {
    await TECHNIQUES.applyAggressiveStealth(page, options);

    await page.addInitScript(() => {
      // ========== Audio Context Spoofing ==========
      const origCreateAnalyser = AudioContext.prototype.createAnalyser;
      if (origCreateAnalyser) {
        AudioContext.prototype.createAnalyser = function(...args) {
          const analyser = origCreateAnalyser.apply(this, args);
          analyser.getByteFrequencyData = function(array) {
            const origData = new Uint8Array(array.length);
            origData.fill(0);
            // Add subtle noise
            for (let i = 0; i < array.length; i++) {
              origData[i] = Math.random() * 10;
            }
            array.set(origData);
            return array;
          };
          return analyser;
        };
      }

      // ========== WebRTC Leak Prevention ==========
      const origRTCPeerConnection = window.RTCPeerConnection;
      window.RTCPeerConnection = function(...args) {
        const pc = new origRTCPeerConnection(...args);
        pc.createDataChannel = function(){};
        return pc;
      };
      window.RTCPeerConnection.prototype = origRTCPeerConnection.prototype;

      // ========== Battery API Spoofing ==========
      if (navigator.getBattery) {
        navigator.getBattery = () => Promise.resolve({
          charging: true,
          chargingTime: 0,
          dischargingTime: Infinity,
          level: 1,
        });
      }

      // ========== Gamepad Spoofing ==========
      Object.defineProperty(navigator, 'getGamepads', {
        get: () => () => Array(4).fill(null),
      });

      // ========== IndexedDB Block ==========
      try {
        const origOpen = indexedDB.open;
        indexedDB.open = function(...args) {
          const req = origOpen.apply(this, args);
          req.onsuccess = () => {
            // Block future operations
            const origStoreNames = req.result.objectStoreNames;
            Object.defineProperty(req.result, 'objectStoreNames', {
              get: () => origStoreNames,
            });
          };
          return req;
        };
      } catch (e) {}
    });

    return page;
  },
};

// ============ DETECTION CHECK ============

async function checkIfDetected(page) {
  try {
    const indicators = await page.evaluate(() => {
      const results = {
        webdriver: !!navigator.webdriver,
        chromeRuntime: !navigator.chrome?.runtime,
        automationControlled: window.navigator.webdriver === true,
        headless: navigator.userAgent.includes('Headless'),
        permissions: Notification.permission === 'denied',
      };

      // Check for automation-specific variables
      const automationVars = ['callSelenium', 'callPhantom', '_phantom', '__webdriver_evaluate'];
      for (const varName of automationVars) {
        if (window[varName] !== undefined) {
          results.automationVar = varName;
        }
      }

      return results;
    });

    // Return true if ANY indicator is suspicious
    return indicators.webdriver ||
           indicators.automationControlled ||
           indicators.automationVar;
  } catch (e) {
    // If check fails, assume detected
    return true;
  }
}

// ============ ADAPTIVE EXECUTION ============

async function executeWithAdaptiveStealth(platform, action, args, maxLevel = 4) {
  let currentLevel = 1;
  let lastError = null;

  while (currentLevel <= maxLevel) {
    try {
      console.log(`[${platform}] Attempting ${action} with stealth level ${currentLevel}...`);

      const result = await executeAction(platform, action, args, currentLevel);

      // Check if detected
      if (result.page && await checkIfDetected(result.page)) {
        console.log(`[${platform}] Detected at level ${currentLevel}, escalating...`);
        currentLevel++;
        await result.browser?.close();
        continue;
      }

      return result;

    } catch (error) {
      lastError = error;

      // Check if it's a detection error
      if (error.message?.includes('detected') ||
          error.message?.includes('blocked') ||
          error.message?.includes('captcha')) {
        console.log(`[${platform}] Error at level ${currentLevel}: ${error.message}`);
        currentLevel++;
        continue;
      }

      // Non-detection error, don't retry
      throw error;
    }
  }

  throw new Error(`Failed after ${maxLevel} stealth levels: ${lastError?.message}`);
}

// ============ ACTION EXECUTION ============

async function executeAction(platform, action, args, stealthLevel) {
  // Launch browser with appropriate args
  const browserArgs = [
    '--disable-blink-features=AutomationControlled',
    '--no-sandbox',
    '--disable-setuid-sandbox',
    '--disable-dev-shm-usage',
    '--disable-accelerated-2d-canvas',
    '--no-first-run',
    '--no-zygote',
    '--window-size=1920,1080',
    '--start-maximized',
  ];

  if (stealthLevel >= 3) {
    browserArgs.push('--disable-web-security');
    browserArgs.push('--disable-features=IsolateOrigins,site-per-process');
  }

  const browser = await chromium.launch({
    headless: false,
    args: browserArgs,
  });

  const context = await browser.newContext({
    viewport: { width: 1920, height: 1080 },
    userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
    locale: 'en-US',
    timezoneId: 'Asia/Jakarta',
  });

  const page = await context.newPage();

  // Apply stealth based on level
  if (stealthLevel >= 1) {
    chromium.use(stealth());
  }
  if (stealthLevel >= 2) {
    await TECHNIQUES.applyModerateStealth(page);
  }
  if (stealthLevel >= 3) {
    await TECHNIQUES.applyAggressiveStealth(page);
  }
  if (stealthLevel >= 4) {
    await TECHNIQUES.applyMaximumStealth(page);
  }

  // Execute platform-specific action
  const result = await PLATFORM_ACTIONS[platform]?.[action]?.(page, context, args);

  return { ...result, browser, page, context };
}

// ============ PLATFORM ACTIONS ============

const PLATFORM_ACTIONS = {
  shopify: {
    login: async (page, context, args) => {
      const { shop_url, email, password } = args;

      await page.goto(`https://${shop_url}/admin`);
      await randomDelay(2000, 4000);

      if (email) {
        await page.fill('#account_email', email);
        await randomDelay(500, 1000);
        await page.keyboard.press('Enter');
        await randomDelay(3000, 5000);
      }

      if (password) {
        await page.fill('#account_password', password);
        await randomDelay(500, 1000);
        await page.keyboard.press('Enter');
      }

      await randomDelay(3000, 6000);

      const cookies = await context.cookies();
      return { success: true, cookies };
    },

    getOrders: async (page, context, args) => {
      const { shop_url, cookies } = args;

      if (cookies) {
        await context.addCookies(cookies);
      }

      await page.goto(`https://${shop_url}/admin/orders`);
      await randomDelay(3000, 5000);

      const orders = await page.evaluate(() => {
        const rows = document.querySelectorAll('.orders-table tr, .ui-sortable tr');
        return Array.from(rows).map(row => {
          const cells = row.querySelectorAll('td');
          return {
            id: cells[0]?.textContent?.trim(),
            customer: cells[1]?.textContent?.trim(),
            total: cells[2]?.textContent?.trim(),
            status: cells[3]?.textContent?.trim(),
          };
        }).filter(o => o.id);
      });

      return { success: true, orders };
    },
  },

  shopee: {
    login: async (page, context, args) => {
      const { email, password } = args;

      await page.goto('https://seller.shopee.co.id/');
      await randomDelay(2000, 4000);

      await page.click('text=Masuk', { timeout: 5000 }).catch(() =>
        page.click('a[href*="login"]', { timeout: 5000 })
      );
      await randomDelay(1500, 3000);

      await page.fill('input[type="email"], input[name="loginKey"]', email);
      await randomDelay(300, 700);
      await page.fill('input[type="password"]', password);
      await randomDelay(300, 700);

      await page.click('button[type="submit"]');
      await randomDelay(5000, 10000);

      const cookies = await context.cookies();
      return { success: true, cookies };
    },

    getOrders: async (page, context, args) => {
      await page.goto('https://seller.shopee.co.id/orders');
      await randomDelay(3000, 5000);

      const orders = await page.evaluate(() => {
        // Adapt to actual Shopee HTML structure
        const items = document.querySelectorAll('.order-list .order-item');
        return Array.from(items).map(item => ({
          id: item.querySelector('.order-id')?.textContent,
          status: item.querySelector('.order-status')?.textContent,
        }));
      });

      return { success: true, orders };
    },
  },

  cj: {
    login: async (page, context, args) => {
      const { email, password } = args;

      await page.goto('https://www.cjdropshipping.com/');
      await randomDelay(2000, 4000);

      await page.click('a[href*="login"], text=Log In', { timeout: 5000 });
      await randomDelay(1500, 3000);

      await page.fill('input[name="email"], input[type="email"]', email);
      await randomDelay(300, 600);
      await page.fill('input[name="password"], input[type="password"]', password);
      await randomDelay(300, 600);

      await page.click('button[type="submit"], button:has-text("Sign In")');
      await randomDelay(3000, 6000);

      const cookies = await context.cookies();
      return { success: true, cookies };
    },

    submitOrder: async (page, context, args) => {
      const { cookies, orderData } = args;

      if (cookies) {
        await context.addCookies(cookies);
      }

      await page.goto('https://www.cjdropshipping.com/create-order');
      await randomDelay(2000, 4000);

      // Fill product URL
      await page.fill('input[name="product_url"], input[placeholder*="Product"]', orderData.productUrl);
      await randomDelay(500, 1000);

      await page.click('button:has-text("Add"), button:has-text("Add Product")');
      await randomDelay(1500, 3000);

      // Fill customer details
      await page.fill('input[name="customer_name"]', orderData.customerName);
      await randomDelay(200, 500);
      await page.fill('input[name="address"], textarea[name="address"]', orderData.address);
      await randomDelay(200, 500);
      await page.fill('input[name="phone"], input[name="tel"]', orderData.phone);
      await randomDelay(200, 500);

      await page.click('button:has-text("Submit"), button:has-text("Place Order")');
      await randomDelay(3000, 6000);

      const tracking = await page.textContent('.tracking-number, .order-tracking, [class*="tracking"]').catch(() => null);

      return { success: true, tracking };
    },
  },

  // Add other platforms as needed
  lazada: {
    login: async (page, context, args) => {
      const { email, password } = args;

      await page.goto('https://sellercenter.lazada.co.id/');
      await randomDelay(2000, 4000);

      await page.fill('input[name="username"], input[type="email"]', email);
      await randomDelay(300, 700);
      await page.fill('input[name="password"]', password);
      await randomDelay(300, 700);

      await page.click('button[type="submit"]');
      await randomDelay(5000, 8000);

      return { success: true, cookies: await context.cookies() };
    },
  },

  tokopedia: {
    login: async (page, context, args) => {
      const { email, password } = args;

      await page.goto('https://seller.tokopedia.com/');
      await randomDelay(2000, 4000);

      await page.fill('input[name="email"], input[placeholder*="Email"]', email);
      await randomDelay(300, 700);
      await page.fill('input[name="password"]', password);
      await randomDelay(300, 700);

      await page.click('button[type="submit"]');
      await randomDelay(5000, 10000);

      return { success: true, cookies: await context.cookies() };
    },
  },

  tiktok: {
    login: async (page, context, args) => {
      const { email, password } = args;

      await page.goto('https://seller-usage.tiktok.com/');
      await randomDelay(2000, 4000);

      await page.fill('input[name="username"], input[type="email"]', email);
      await randomDelay(300, 700);
      await page.fill('input[name="password"]', password);
      await randomDelay(300, 700);

      await page.click('button[type="submit"]');
      await randomDelay(5000, 10000);

      return { success: true, cookies: await context.cookies() };
    },
  },
};

// ============ MAIN EXECUTION ============

const platform = process.argv[2];
const action = process.argv[3];
const args = JSON.parse(process.argv[4] || '{}');

async function main() {
  try {
    if (!PLATFORM_ACTIONS[platform]) {
      throw new Error(`Unknown platform: ${platform}`);
    }

    if (!PLATFORM_ACTIONS[platform][action]) {
      throw new Error(`Unknown action: ${action} for platform: ${platform}`);
    }

    const result = await executeWithAdaptiveStealth(platform, action, args);

    console.log(JSON.stringify({
      success: true,
      data: {
        cookies: result.cookies,
        orders: result.orders,
        tracking: result.tracking,
        page: null, // Don't serialize page object
      }
    }));

    await result.browser?.close();

  } catch (error) {
    console.log(JSON.stringify({
      success: false,
      error: error.message,
      platform,
      action,
    }));
    process.exit(1);
  }
}

main();
