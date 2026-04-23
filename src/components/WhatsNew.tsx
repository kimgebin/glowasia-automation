import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface WhatsNewProps {
  onClose: () => void;
}

export default function WhatsNew({ onClose }: WhatsNewProps) {
  const [changelog, setChangelog] = useState('');
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadChangelog();
  }, []);

  const loadChangelog = async () => {
    try {
      const text = await invoke<string>('get_changelog');
      setChangelog(text);
    } catch (e) {
      console.error('Failed to load changelog:', e);
      setChangelog("## What's New\n\nNew version available!");
    } finally {
      setLoading(false);
    }
  };

  const handleClose = async () => {
    try {
      await invoke('mark_version_seen');
    } catch (e) {
      console.error('Failed to mark version seen:', e);
    }
    onClose();
  };

  return (
    <div className="fixed inset-0 bg-black/70 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-xl max-w-lg w-full mx-4 max-h-[80vh] overflow-hidden shadow-2xl border border-gray-700">
        {/* Header */}
        <div className="bg-gradient-to-r from-purple-600 to-blue-600 px-6 py-4">
          <div className="flex items-center justify-between">
            <div>
              <h2 className="text-xl font-bold text-white">🆕 What's New!</h2>
              <p className="text-purple-200 text-sm">GLOWASIA Copilot updated</p>
            </div>
            <button
              onClick={handleClose}
              className="text-white/80 hover:text-white text-2xl leading-none"
            >
              ×
            </button>
          </div>
        </div>

        {/* Content */}
        <div className="p-6 overflow-y-auto max-h-96">
          {loading ? (
            <div className="animate-pulse text-gray-400">Loading...</div>
          ) : (
            <div className="prose prose-invert prose-sm max-w-none">
              {changelog.split('\n').map((line, i) => {
                if (line.startsWith('## ')) {
                  return <h2 key={i} className="text-lg font-bold text-white mt-4 mb-2">{line.replace('## ', '')}</h2>;
                }
                if (line.startsWith('### ')) {
                  return <h3 key={i} className="text-md font-semibold text-blue-400 mt-3 mb-1">{line.replace('### ', '')}</h3>;
                }
                if (line.startsWith('- ')) {
                  return (
                    <div key={i} className="flex items-start gap-2 my-1">
                      <span className="text-blue-400 mt-1">•</span>
                      <span className="text-gray-300 text-sm">{line.replace('- ', '')}</span>
                    </div>
                  );
                }
                if (line.startsWith('*') && line.endsWith('*')) {
                  return <p key={i} className="text-gray-500 text-xs italic my-2">{line.replace(/\*/g, '')}</p>;
                }
                if (line.trim() === '---') {
                  return <hr key={i} className="border-gray-700 my-3" />;
                }
                if (line.trim() === '') {
                  return <div key={i} className="h-2" />;
                }
                return <p key={i} className="text-gray-300 text-sm">{line}</p>;
              })}
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="px-6 py-4 bg-gray-900 border-t border-gray-700">
          <button
            onClick={handleClose}
            className="w-full py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors"
          >
            Got it! 🚀
          </button>
        </div>
      </div>
    </div>
  );
}
