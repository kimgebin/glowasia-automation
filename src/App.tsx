import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import Dashboard from './components/Dashboard';
import StatusBar from './components/StatusBar';
import ActivityLog from './components/ActivityLog';
import Settings from './components/Settings';
import Reports from './components/Reports';
import WhatsNew from './components/WhatsNew';
import Products from './components/Products';

function App() {
  const [activeTab, setActiveTab] = useState('dashboard');
  const [showWhatsNew, setShowWhatsNew] = useState(false);
  const [productCount, setProductCount] = useState(0);

  useEffect(() => {
    loadProductCount();
  }, []);

  const loadProductCount = async () => {
    try {
      const count = await invoke<number>('get_product_count');
      setProductCount(count);
    } catch (e) {
      console.error('Failed to load product count:', e);
    }
  };

  const [systemStatus, setSystemStatus] = useState({
    shopify_connected: false,
    shopee_connected: false,
    lazada_connected: false,
    tokopedia_connected: false,
    tiktok_connected: false,
    cj_connected: false,
    telegram_connected: false,
    etsy_connected: false,
    automation_running: false,
    automation_state: 'idle'
  });

  useEffect(() => {
    checkWhatsNew();
    loadSystemStatus();
    const interval = setInterval(loadSystemStatus, 5000);
    return () => clearInterval(interval);
  }, []);

  const checkWhatsNew = async () => {
    try {
      const shouldShow = await invoke<boolean>('should_show_whats_new');
      if (shouldShow) {
        setShowWhatsNew(true);
      }
    } catch (e) {
      console.error('Failed to check whats new:', e);
    }
  };

  const loadSystemStatus = async () => {
    try {
      const status = await invoke<typeof systemStatus>('get_system_status');
      setSystemStatus(status);
    } catch (e) {
      console.error('Failed to load status:', e);
    }
    loadProductCount();
  };

  return (
    <div className="min-h-screen bg-gray-900 text-white">
      {/* Whats New Popup */}
      {showWhatsNew && <WhatsNew onClose={() => setShowWhatsNew(false)} />}

      {/* Sidebar */}
      <div className="fixed left-0 top-0 h-full w-16 bg-gray-800 flex flex-col items-center py-4 space-y-4">
        <button
          onClick={() => setActiveTab('dashboard')}
          className={`p-3 rounded-lg ${activeTab === 'dashboard' ? 'bg-blue-600' : 'hover:bg-gray-700'}`}
        >
          📊
        </button>
        <button
          onClick={() => setActiveTab('activity')}
          className={`p-3 rounded-lg ${activeTab === 'activity' ? 'bg-blue-600' : 'hover:bg-gray-700'}`}
        >
          📋
        </button>
        <button
          onClick={() => setActiveTab('products')}
          className={`p-3 rounded-lg relative ${activeTab === 'products' ? 'bg-blue-600' : 'hover:bg-gray-700'}`}
        >
          📦
          {productCount > 0 && (
            <span className="absolute -top-1 -right-1 bg-purple-600 text-xs rounded-full w-5 h-5 flex items-center justify-center">
              {productCount > 99 ? '99+' : productCount}
            </span>
          )}
        </button>
        <button
          onClick={() => setActiveTab('reports')}
          className={`p-3 rounded-lg ${activeTab === 'reports' ? 'bg-blue-600' : 'hover:bg-gray-700'}`}
        >
          📈
        </button>
        <button
          onClick={() => setActiveTab('settings')}
          className={`p-3 rounded-lg ${activeTab === 'settings' ? 'bg-blue-600' : 'hover:bg-gray-700'}`}
        >
          ⚙️
        </button>
      </div>

      {/* Main Content */}
      <div className="ml-16">
        <header className="bg-gray-800 border-b border-gray-700 px-6 py-4">
          <h1 className="text-xl font-bold">🤖 GLOWASIA Copilot</h1>
          <p className="text-sm text-gray-400">100% Auto-Pilot Dropshipping</p>
        </header>

        <main className="p-6 pb-16">
          {activeTab === 'dashboard' && <Dashboard status={systemStatus} onNavigate={setActiveTab} />}
          {activeTab === 'activity' && <ActivityLog />}
          {activeTab === 'reports' && <Reports />}
          {activeTab === 'settings' && <Settings />}
          {activeTab === 'products' && <Products />}
        </main>

        <StatusBar status={systemStatus} />
      </div>
    </div>
  );
}

export default App;
