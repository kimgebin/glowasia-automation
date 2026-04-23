export default function Reports() {
  return (
    <div className="space-y-6">
      <h3 className="text-lg font-bold">📈 Reports & Analytics</h3>
      <div className="grid grid-cols-2 gap-6">
        <div className="bg-gray-800 rounded-lg p-6">
          <h4 className="font-bold mb-4">Weekly Orders</h4>
          <div className="space-y-2">
            <div className="flex justify-between">
              <span>Monday</span>
              <span className="text-blue-400">12</span>
            </div>
            <div className="flex justify-between">
              <span>Tuesday</span>
              <span className="text-blue-400">8</span>
            </div>
            <div className="flex justify-between">
              <span>Wednesday</span>
              <span className="text-blue-400">15</span>
            </div>
            <div className="flex justify-between">
              <span>Thursday</span>
              <span className="text-blue-400">10</span>
            </div>
            <div className="flex justify-between">
              <span>Friday</span>
              <span className="text-blue-400">22</span>
            </div>
          </div>
        </div>

        <div className="bg-gray-800 rounded-lg p-6">
          <h4 className="font-bold mb-4">Revenue by Platform</h4>
          <div className="space-y-2">
            <div className="flex justify-between">
              <span>🛒 Shopify</span>
              <span className="text-green-400">$1,234</span>
            </div>
            <div className="flex justify-between">
              <span>🛍️ Shopee</span>
              <span className="text-green-400">$856</span>
            </div>
            <div className="flex justify-between">
              <span>📦 Lazada</span>
              <span className="text-green-400">$432</span>
            </div>
            <div className="flex justify-between">
              <span>🏪 Tokopedia</span>
              <span className="text-green-400">$321</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
