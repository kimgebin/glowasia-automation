interface StatusBarProps {
  status: {
    automation_state: string;
    automation_running: boolean;
    [key: string]: unknown;
  };
}

export default function StatusBar({ status }: StatusBarProps) {
  return (
    <div className="fixed bottom-0 left-16 right-0 bg-gray-800 border-t border-gray-700 px-6 py-2 flex items-center justify-between">
      <div className="flex items-center space-x-4">
        <span className={`w-3 h-3 rounded-full ${status.automation_running ? 'bg-green-500' : 'bg-gray-500'}`}></span>
        <span className="text-sm">
          Auto-Pilot: {status.automation_running ? 'Active' : 'Inactive'}
        </span>
      </div>
      <div className="text-sm text-gray-400">
        State: {status.automation_state}
      </div>
    </div>
  );
}
