# GLOWASIA Copilot - Development Guide

## Table of Contents
1. [Getting Started](#getting-started)
2. [Development Setup](#development-setup)
3. [Project Structure](#project-structure)
4. [Frontend Development](#frontend-development)
5. [Backend Development (Rust)](#backend-development-rust)
6. [Adding New Platforms](#adding-new-platforms)
7. [Building & Testing](#building--testing)
8. [Code Standards](#code-standards)
9. [Contributing](#contributing)

---

## Getting Started

### Prerequisites

Before contributing, ensure you have:

- **Node.js** 18+ (for frontend)
- **Rust** 1.70+ (for backend)
- **Git** for version control
- **Xcode Command Line Tools** for macOS development

### Tech Stack

| Component | Technology | Version |
|-----------|------------|---------|
| Framework | Tauri v2 | 2.x |
| Frontend | React | 18.x |
| Build Tool | Vite | 5.x |
| Language | TypeScript | 5.x |
| Styling | TailwindCSS | 3.x |
| Backend | Rust | 1.70+ |
| Database | SQLite (rusqlite) | 3.x |
| Browser Automation | Playwright | 1.x |

---

## Development Setup

### Step 1: Clone the Repository

```bash
git clone https://github.com/kimgebin/glowasia-automation.git
cd glowasia-automation
```

### Step 2: Install Frontend Dependencies

```bash
npm install
```

### Step 3: Install Rust Dependencies

```bash
cd src-tauri
cargo fetch
cd ..
```

### Step 4: Configure Environment

Create a `.env` file if needed:

```bash
# Optional: Override default settings
GLOWASIA_DEBUG=true
GLOWASIA_LOG_LEVEL=debug
```

### Step 5: Run Development Server

```bash
npm run tauri dev
```

This starts:
- Frontend dev server at `http://localhost:1420`
- Tauri app window with hot-reload

---

## Project Structure

```
glowasia-automation/
├── src/                          # React Frontend
│   ├── components/               # UI Components
│   │   ├── Dashboard.tsx        # Main dashboard view
│   │   ├── Settings.tsx         # Settings panel
│   │   ├── StatusBar.tsx        # Bottom status bar
│   │   ├── ActivityLog.tsx      # Activity log display
│   │   ├── PlatformCard.tsx     # Platform status card
│   │   └── CredentialForm.tsx   # Credential input form
│   ├── hooks/                   # React Hooks
│   │   ├── useCredentials.ts    # Credentials management hook
│   │   ├── useAutomation.ts     # Automation control hook
│   │   └── usePlatformStatus.ts # Platform status hook
│   ├── pages/                   # Page Components
│   │   ├── DashboardPage.tsx    # Main dashboard
│   │   └── SettingsPage.tsx     # Settings page
│   ├── App.tsx                  # Main App component
│   ├── main.tsx                 # Entry point
│   └── index.css                # Global styles (Tailwind)
│
├── src-tauri/                   # Rust Backend
│   ├── src/
│   │   ├── lib.rs              # Main library, Tauri commands
│   │   ├── main.rs             # App entry point
│   │   ├── db.rs               # SQLite database operations
│   │   ├── credentials.rs      # Credentials management
│   │   ├── browser.rs          # Playwright browser automation
│   │   └── storage/            # Storage modules
│   ├── Cargo.toml              # Rust dependencies
│   ├── tauri.conf.json         # Tauri configuration
│   └── capabilities/           # Tauri capability files
│
├── public/                      # Static assets
├── dist/                        # Built frontend (gitignored)
├── package.json                 # Node dependencies
├── vite.config.ts              # Vite configuration
├── tailwind.config.js          # TailwindCSS configuration
└── tsconfig.json               # TypeScript configuration
```

---

## Frontend Development

### Adding a New Component

1. Create file in `src/components/`:

```tsx
// src/components/MyComponent.tsx
import React from 'react';

interface MyComponentProps {
  title: string;
}

export const MyComponent: React.FC<MyComponentProps> = ({ title }) => {
  return (
    <div className="bg-card rounded-lg p-4">
      <h2 className="text-textPrimary">{title}</h2>
    </div>
  );
};
```

2. Import in `App.tsx` or parent component

### Using Tauri Commands

Frontend communicates with Rust via Tauri commands:

```typescript
// src/hooks/useMyFeature.ts
import { invoke } from '@tauri-apps/api/core';

export const useMyFeature = () => {
  const getData = async () => {
    try {
      const result = await invoke<string>('get_data');
      return JSON.parse(result);
    } catch (error) {
      console.error('Failed to get data:', error);
      throw error;
    }
  };

  return { getData };
};
```

### Rust Command Definition

```rust
// src-tauri/src/lib.rs
#[tauri::command]
async fn get_data() -> Result<String, String> {
    // Your logic here
    Ok("data".to_string())
}
```

### Styling with Tailwind

The project uses TailwindCSS with custom theme:

```tsx
// Using custom theme colors
<div className="bg-background text-textPrimary">
  <button className="bg-primary hover:bg-primary/80 px-4 py-2 rounded">
    Click Me
  </button>
</div>
```

#### Theme Colors

| Name | Hex | Usage |
|------|-----|-------|
| background | #0a0a0f | Main app background |
| card | #141420 | Card/container background |
| primary | #e8b4b8 | GLOWASIA pink accent |
| secondary | #6366f1 | Indigo accent |
| success | #10b981 | Success states |
| warning | #f59e0b | Warning states |
| error | #ef4444 | Error states |
| textPrimary | #ffffff | Main text |
| textSecondary | #9ca3af | Muted text |

---

## Backend Development (Rust)

### Adding a New Tauri Command

1. Define command in `src-tauri/src/lib.rs`:

```rust
#[tauri::command]
async fn my_command(param: String) -> Result<MyResponse, String> {
    // Validate input
    if param.is_empty() {
        return Err("Parameter cannot be empty".to_string());
    }

    // Process
    let result = do_something(&param).await?;

    Ok(MyResponse {
        success: true,
        data: result,
    })
}
```

2. Register in `lib.rs` builder:

```rust
fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            my_command,
            // ... other commands
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Database Operations

```rust
// src-tauri/src/db.rs
use rusqlite::{Connection, Result};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("path/to/db.sqlite")?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS credentials (
            id INTEGER PRIMARY KEY,
            platform TEXT NOT NULL,
            api_key TEXT,
            api_secret TEXT
        )",
        [],
    )?;
    
    Ok(conn)
}
```

### Credential Storage

```rust
// src-tauri/src/credentials.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Credential {
    pub id: i64,
    pub platform: String,
    pub account_name: String,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub access_token: Option<String>,
    pub shop_url: Option<String>,
    pub additional_data: Option<String>,
}

impl Credential {
    pub fn new(platform: &str, account_name: &str) -> Self {
        Self {
            id: 0,
            platform: platform.to_string(),
            account_name: account_name.to_string(),
            api_key: None,
            api_secret: None,
            access_token: None,
            shop_url: None,
            additional_data: None,
        }
    }
}
```

### Browser Automation

```rust
// src-tauri/src/browser.rs
use playwright::Playwright;

pub async fn login_shopify(
    shop_url: &str,
    api_key: &str
) -> Result<(), String> {
    let playwright = Playwright::init().await.map_err(|e| e.to_string())?;
    let browser = playwright.chromium().launch().await.map_err(|e| e.to_string())?;
    let context = browser.context().await.map_err(|e| e.to_string())?;
    let page = context.new_page().await.map_err(|e| e.to_string())?;
    
    // Navigate and login logic here
    Ok(())
}
```

---

## Adding New Platforms

### Step 1: Update Credential Types

```rust
// src-tauri/src/credentials.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Shopify,
    Shopee,
    Lazada,
    Tokopedia,
    TikTokShop,
    Telegram,
    GoogleSheets,
    Midtrans,
    GitHub,
    CjDropshipping,
    // ADD NEW PLATFORM HERE
}
```

### Step 2: Add Platform Configuration

```rust
// Platform configuration in settings UI
const PLATFORMS: &[(&str, &str, &[&str])] = &[
    ("shopify", "Shopify", &["api_key", "shop_url"]),
    ("shopee", "Shopee", &["partner_id", "partner_key", "shop_id"]),
    // ADD NEW PLATFORM HERE
];
```

### Step 3: Add API Integration

Create new module: `src-tauri/src/platforms/myplatform.rs`

```rust
// src-tauri/src/platforms/myplatform.rs
pub struct MyPlatform {
    api_key: String,
    api_secret: String,
}

impl MyPlatform {
    pub fn new(api_key: &str, api_secret: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
        }
    }

    pub async fn test_connection(&self) -> Result<bool, String> {
        // Test connection logic
        Ok(true)
    }

    pub async fn fetch_orders(&self) -> Result<Vec<Order>, String> {
        // Fetch orders logic
        Ok(vec![])
    }
}
```

### Step 4: Register Platform Module

```rust
// src-tauri/src/lib.rs (or new file src-tauri/src/platforms/mod.rs)
mod platforms;
// or
mod platforms::myplatform;
```

---

## Building & Testing

### Build Commands

```bash
# Development build
npm run tauri build -- --debug

# Production build
npm run tauri build

# Build only frontend
npm run build

# Build only backend
cd src-tauri && cargo build --release
```

### Testing

```bash
# Frontend tests (if configured)
npm run test

# Rust tests
cd src-tauri && cargo test

# Integration tests
npm run test:integration
```

### Debug Build

```bash
# Enable verbose logging
RUST_LOG=debug npm run tauri build -- --debug

# Attach debugger
# In Xcode: Debug → Attach to Process → GLOWASIA Copilot
```

---

## Code Standards

### TypeScript Standards

1. **Use strict TypeScript** - No `any` types
2. **Prefer interfaces over types** for object shapes
3. **Use named exports** for utilities
4. **Document complex logic** with comments

```typescript
// Good
interface Order {
  id: string;
  customer: Customer;
  products: Product[];
  total: number;
  status: OrderStatus;
}

// Avoid
const order: any = { ... };
```

### Rust Standards

1. **Use Result for error handling**
2. **Document public functions** with doc comments
3. **Use appropriate error messages**
4. **Follow Rust idioms** (ownership, borrowing)

```rust
/// Fetches orders from Shopify for the given shop.
/// 
/// # Arguments
/// * `shop_url` - The Shopify store URL
/// * `api_key` - The Admin API access token
/// 
/// # Errors
/// Returns an error if the API request fails or credentials are invalid.
pub async fn fetch_orders(shop_url: &str, api_key: &str) -> Result<Vec<Order>, String> {
    // Implementation
}
```

### React Component Standards

1. **Use functional components with hooks**
2. **Define interfaces for props**
3. **Use named exports for components**
4. **Keep components focused** (single responsibility)

---

## Contributing

### Workflow

1. **Fork** the repository
2. **Create a branch** for your feature: `git checkout -b feature/my-feature`
3. **Make changes** following the code standards
4. **Test** your changes
5. **Commit** with clear messages: `git commit -m "Add feature: description"`
6. **Push** to your fork: `git push origin feature/my-feature`
7. **Open a Pull Request**

### Commit Message Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Example:
```
feat(shopify): add order filtering by status

- Add status filter parameter
- Update API request to include status
- Add tests for status filtering
```

### Pull Request Template

```markdown
## Description
[Describe your changes]

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
[Describe how you tested the changes]

## Checklist
- [ ] Code follows project standards
- [ ] Tests pass
- [ ] Documentation updated (if applicable)
- [ ] No console errors or warnings
```

---

## Additional Resources

- [Tauri v2 Documentation](https://tauri.app/v2/)
- [React Documentation](https://react.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [TailwindCSS Documentation](https://tailwindcss.com/docs)
- [Playwright Documentation](https://playwright.dev/)

---

## Need Help?

- Check existing [GitHub Issues](https://github.com/kimgebin/glowasia-automation/issues)
- Ask questions in [Discussions](https://github.com/kimgebin/glowasia-automation/discussions)
