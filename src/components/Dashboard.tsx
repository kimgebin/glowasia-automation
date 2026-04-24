import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface DashboardProps {
  status: {
    shopify_connected: boolean;
    shopee_connected: boolean;
    lazada_connected: boolean;
    tokopedia_connected: boolean;
    tiktok_connected: boolean;
    cj_connected: boolean;
    telegram_connected: boolean;
    etsy_connected: boolean;
    automation_running: boolean;
    automation_state: string;
  };
}

interface Stats {
  orders_today: number;
  revenue_today: number;
  shipped_today: number;
  delivered_today: number;
  paypal_balance?: number;
  midtrans_balance?: number;
  shopify_balance?: number;
  cj_pending_orders?: number;
  gmail_unread?: number;
}

export default function Dashboard({ status, onNavigate }: DashboardProps & { onNavigate?: (tab: string) => void }) {
  const [stats, setStats] = useState<Stats>({
    orders_today: 0,
    revenue_today: 0,
    shipped_today: 0,
    delivered_today: 0,
    paypal_balance: 0,
    midtrans_balance: 0,
    shopify_balance: 0,
    cj_pending_orders: 0,
    gmail_unread: 0,
  });
  const [productCount, setProductCount] = useState(0);
  const [loading, setLoading] = useState(true);
  const [automationActive, setAutomationActive] = useState(false);

  useEffect(() => {
    const loadAll = async () => {
      try {
        const [statsData, count] = await Promise.all([
          invoke<Stats>('get_dashboard_stats').catch(() => null),
          invoke<number>('get_product_count').catch(() => null),
        ]);
        if (statsData) setStats(statsData);
        if (count !== null) setProductCount(count);
      } catch (e) {
        console.error('Load error:', e);
      } finally {
        setLoading(false);
      }
    };
    loadAll();
    // Safety fallback - force loading false after 10s
    const timeout = setTimeout(() => setLoading(false), 10000);
    return () => clearTimeout(timeout);
  }, []);

  const startAutomation = async () => {
    try {
      await invoke('start_automation');
      setAutomationActive(true);
    } catch (e) {
      console.error('Failed to start automation:', e);
    }
  };

  const stopAutomation = async () => {
    try {
      await invoke('stop_automation');
      setAutomationActive(false);
    } catch (e) {
      console.error('Failed to stop automation:', e);
    }
  };

  const platforms = [
    { name: 'Shopify', connected: status.shopify_connected, icon: '🛒' },
    { name: 'Shopee', connected: status.shopee_connected, icon: '🛍️' },
    { name: 'Lazada', connected: status.lazada_connected, icon: '📦' },
    { name: 'Tokopedia', connected: status.tokopedia_connected, icon: '🏪' },
    { name: 'TikTok Shop', connected: status.tiktok_connected, icon: '🎵' },
    { name: 'CJ Dropshipping', connected: status.cj_connected, icon: '🚚' },
    { name: 'Etsy', connected: status.etsy_connected, icon: '🛍️' },
  ];

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-center">
          <div className="animate-spin text-4xl mb-4">⚙️</div>
          <p className="text-gray-400">Loading dashboard...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Financial Overview */}
      <div className="bg-gray-800 rounded-lg p-4">
        <h3 className="text-lg font-bold mb-4">💰 Financial Overview</h3>
        <div className="grid grid-cols-3 md:grid-cols-6 gap-3">
          <div className="bg-gray-700 rounded-lg p-3">
            <p className="text-gray-400 text-xs">💰 PayPal</p>
            <p className="text-lg font-bold text-green-400">${stats.paypal_balance?.toFixed(2) || '0.00'}</p>
          </div>
          <div className="bg-gray-700 rounded-lg p-3">
            <p className="text-gray-400 text-xs">💳 Midtrans</p>
            <p className="text-lg font-bold text-green-400">Rp {(stats.midtrans_balance || 0).toLocaleString('id-ID')}</p>
          </div>
          <div className="bg-gray-700 rounded-lg p-3">
            <p className="text-gray-400 text-xs">📊 Shopify</p>
            <p className="text-lg font-bold text-green-400">${stats.shopify_balance?.toFixed(2) || '0.00'}</p>
          </div>
          <div className="bg-gray-700 rounded-lg p-3">
            <p className="text-gray-400 text-xs">📦 CJ Pending</p>
            <p className="text-lg font-bold text-yellow-400">{stats.cj_pending_orders || 0}</p>
          </div>
          <div className="bg-gray-700 rounded-lg p-3">
            <p className="text-gray-400 text-xs">📈 Products</p>
            <p className="text-lg font-bold text-blue-400">{productCount || 0}</p>
          </div>
          <div className="bg-gray-700 rounded-lg p-3">
            <p className="text-gray-400 text-xs">📧 Gmail</p>
            <p className="text-lg font-bold text-red-400">{stats.gmail_unread || 0}</p>
          </div>
        </div>
      </div>

      {/* Stats Cards */}
      <div className="grid grid-cols-4 gap-4">
        <div className="bg-gray-800 rounded-lg p-4">
          <p className="text-gray-400 text-sm">Orders Today</p>
          <p className="text-3xl font-bold text-blue-400">{stats.orders_today}</p>
        </div>
        <div className="bg-gray-800 rounded-lg p-4">
          <p className="text-gray-400 text-sm">Revenue Today</p>
          <p className="text-3xl font-bold text-green-400">${stats.revenue_today.toFixed(2)}</p>
        </div>
        <div className="bg-gray-800 rounded-lg p-4">
          <p className="text-gray-400 text-sm">Shipped</p>
          <p className="text-3xl font-bold text-yellow-400">{stats.shipped_today}</p>
        </div>
        <div className="bg-gray-800 rounded-lg p-4">
          <p className="text-gray-400 text-sm">Delivered</p>
          <p className="text-3xl font-bold text-purple-400">{stats.delivered_today}</p>
        </div>
      </div>

      {/* Auto-Pilot Control */}
      <div className="bg-gradient-to-r from-blue-900 to-purple-900 rounded-lg p-6">
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-2xl font-bold">🤖 Auto-Pilot Mode</h2>
            <p className="text-gray-300">Status: {status.automation_state}</p>
          </div>
          <button
            onClick={automationActive ? stopAutomation : startAutomation}
            className={`px-6 py-3 rounded-lg font-bold text-lg ${
              automationActive
                ? 'bg-red-600 hover:bg-red-700'
                : 'bg-green-600 hover:bg-green-700'
            }`}
          >
            {automationActive ? '⏹️ Stop Auto-Pilot' : '🚀 Start Auto-Pilot'}
          </button>
        </div>
        
        <div className="mt-4 grid grid-cols-5 gap-4 text-center">
          <div className="bg-black/30 rounded-lg p-3">
            <p className="text-2xl">✅</p>
            <p className="text-xs">Auto-detect orders</p>
          </div>
          <div className="bg-black/30 rounded-lg p-3">
            <p className="text-2xl">✅</p>
            <p className="text-xs">Auto-forward to CJ</p>
          </div>
          <div className="bg-black/30 rounded-lg p-3">
            <p className="text-2xl">✅</p>
            <p className="text-xs">Auto-tracking update</p>
          </div>
          <div className="bg-black/30 rounded-lg p-3">
            <p className="text-2xl">✅</p>
            <p className="text-xs">Telegram notify</p>
          </div>
          <div className="bg-black/30 rounded-lg p-3">
            <p className="text-2xl">✅</p>
            <p className="text-xs">Auto-reconnect</p>
          </div>
        </div>
      </div>

      {/* Platform Status */}
      <div className="bg-gray-800 rounded-lg p-6">
        <h3 className="text-lg font-bold mb-4">📱 Platform Connections</h3>
        <div className="grid grid-cols-3 gap-4">
          {platforms.map((platform) => (
            <div
              key={platform.name}
              className={`p-4 rounded-lg flex items-center space-x-3 ${
                platform.connected ? 'bg-green-900/50' : 'bg-gray-700/50'
              }`}
            >
              <span className="text-2xl">{platform.icon}</span>
              <div>
                <p className="font-bold">{platform.name}</p>
                <p className={`text-sm ${platform.connected ? 'text-green-400' : 'text-red-400'}`}>
                  {platform.connected ? '● Connected' : '○ Disconnected'}
                </p>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Telegram Status */}
      <div className={`p-4 rounded-lg flex items-center justify-between ${
        status.telegram_connected ? 'bg-green-900/50' : 'bg-gray-700/50'
      }`}>
        <div className="flex items-center space-x-3">
          <span className="text-2xl">📨</span>
          <div>
            <p className="font-bold">Telegram Notifications</p>
            <p className={`text-sm ${status.telegram_connected ? 'text-green-400' : 'text-red-400'}`}>
              {status.telegram_connected ? '● Active' : '○ Not configured'}
            </p>
          </div>
        </div>
        <button
          onClick={() => onNavigate?.('settings')}
          className="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg"
        >
          Configure
        </button>
      </div>
    </div>
  );
}
