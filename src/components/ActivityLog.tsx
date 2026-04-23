import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface LogEntry {
  id: number;
  timestamp: string;
  level: string;
  message: string;
  platform?: string;
}

export default function ActivityLog() {
  const [logs, setLogs] = useState<LogEntry[]>([]);

  useEffect(() => {
    loadLogs();
    const interval = setInterval(loadLogs, 5000);
    return () => clearInterval(interval);
  }, []);

  const loadLogs = async () => {
    try {
      const entries = await invoke<LogEntry[]>('get_activity_logs');
      setLogs(entries);
    } catch (e) {
      console.error('Failed to load logs:', e);
    }
  };

  const getLevelColor = (level: string) => {
    switch (level.toLowerCase()) {
      case 'error':
        return 'text-red-400';
      case 'warning':
        return 'text-yellow-400';
      case 'success':
        return 'text-green-400';
      default:
        return 'text-gray-400';
    }
  };

  return (
    <div className="bg-gray-800 rounded-lg p-6">
      <h3 className="text-lg font-bold mb-4">📋 Activity Log</h3>
      <div className="space-y-2 max-h-96 overflow-y-auto">
        {logs.map((log) => (
          <div key={log.id} className="flex items-start space-x-3 py-2 border-b border-gray-700">
            <span className="text-gray-500 text-sm whitespace-nowrap">
              {new Date(log.timestamp).toLocaleTimeString()}
            </span>
            <span className={`font-bold ${getLevelColor(log.level)}`}>
              [{log.level.toUpperCase()}]
            </span>
            {log.platform && (
              <span className="bg-blue-900 text-blue-300 px-2 rounded text-sm">
                {log.platform}
              </span>
            )}
            <span className="text-gray-300">{log.message}</span>
          </div>
        ))}
        {logs.length === 0 && (
          <p className="text-gray-400 text-center py-8">No activity logs yet.</p>
        )}
      </div>
    </div>
  );
}
