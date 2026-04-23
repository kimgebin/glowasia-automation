import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface Product {
  id: number;
  title: string;
  description: string | null;
  price: number;
  cost: number | null;
  images: string | null;
  category: string | null;
  sku: string | null;
  status: string;
  platform_links: string | null;
  created_at: string | null;
  updated_at: string | null;
}

interface ProductFormProps {
  product: Product | null;
  onClose: () => void;
  onSave: () => void;
}

export default function ProductForm({ product, onClose, onSave }: ProductFormProps) {
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [price, setPrice] = useState('');
  const [cost, setCost] = useState('');
  const [images, setImages] = useState('');
  const [category, setCategory] = useState('');
  const [sku, setSku] = useState('');
  const [status, setStatus] = useState('active');
  const [markupPercentage, setMarkupPercentage] = useState(30);
  const [saving, setSaving] = useState(false);
  const [syncingTo, setSyncingTo] = useState<string | null>(null);

  useEffect(() => {
    if (product) {
      setTitle(product.title);
      setDescription(product.description || '');
      setPrice(product.price.toString());
      setCost(product.cost?.toString() || '');
      try {
        const imgs = product.images ? JSON.parse(product.images) : [];
        setImages(imgs.join('\n'));
      } catch {
        setImages(product.images || '');
      }
      setCategory(product.category || '');
      setSku(product.sku || '');
      setStatus(product.status);
    }
    loadMarkup();
  }, [product]);

  const loadMarkup = async () => {
    try {
      const pct = await invoke<number>('get_markup_percentage');
      setMarkupPercentage(pct);
    } catch (e) {
      console.error('Failed to load markup:', e);
    }
  };

  const calculatePrice = () => {
    const costVal = parseFloat(cost);
    if (!isNaN(costVal) && costVal > 0) {
      setPrice((costVal * (1 + markupPercentage / 100)).toFixed(2));
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!title.trim() || !price) return;

    setSaving(true);
    try {
      const productData = {
        title: title.trim(),
        description: description.trim() || null,
        price: parseFloat(price),
        cost: cost ? parseFloat(cost) : null,
        images: images.trim() ? JSON.stringify(images.split('\n').filter(url => url.trim())) : null,
        category: category.trim() || null,
        sku: sku.trim() || null,
        status,
      };

      if (product) {
        await invoke('update_product', { id: product.id, product: productData });
      } else {
        await invoke('add_product', { product: productData });
      }
      onSave();
    } catch (e) {
      console.error('Failed to save product:', e);
      alert(`Failed to save: ${e}`);
    } finally {
      setSaving(false);
    }
  };

  const getPlatformLinks = (): string[] => {
    if (!product?.platform_links) return [];
    try {
      return Object.keys(JSON.parse(product.platform_links));
    } catch {
      return [];
    }
  };

  const handleSyncToPlatform = async (platform: string) => {
    if (!product) return;
    setSyncingTo(platform);
    try {
      // For now, just mark it as synced - actual API integration would go here
      const fakeLinkId = `${platform}_listing_${Date.now()}`;
      await invoke('sync_product_to_platform', {
        productId: product.id,
        platform,
        linkId: fakeLinkId
      });
      alert(`Synced to ${platform}! (Demo mode - actual API integration pending)`);
    } catch (e) {
      console.error(`Failed to sync to ${platform}:`, e);
      alert(`Sync failed: ${e}`);
    } finally {
      setSyncingTo(null);
    }
  };

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-xl font-bold">
          {product ? '✏️ Edit Product' : '➕ Add New Product'}
        </h3>
        <button onClick={onClose} className="text-gray-400 hover:text-white text-2xl">&times;</button>
      </div>

      <form onSubmit={handleSubmit} className="space-y-4">
        <div className="grid grid-cols-2 gap-4">
          <div className="col-span-2">
            <label className="block text-sm font-medium text-gray-400 mb-1">Title *</label>
            <input
              type="text"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              required
              className="w-full px-4 py-2 bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="Product title"
            />
          </div>

          <div className="col-span-2">
            <label className="block text-sm font-medium text-gray-400 mb-1">Description</label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              rows={3}
              className="w-full px-4 py-2 bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="Product description"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-400 mb-1">Cost (CJ Price)</label>
            <div className="flex space-x-2">
              <input
                type="number"
                step="0.01"
                value={cost}
                onChange={(e) => setCost(e.target.value)}
                className="flex-1 px-4 py-2 bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                placeholder="0.00"
              />
              <button
                type="button"
                onClick={calculatePrice}
                className="px-3 py-2 bg-gray-600 hover:bg-gray-700 rounded-lg text-sm"
                title="Calculate price from cost + markup"
              >
                Calc
              </button>
            </div>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-400 mb-1">Selling Price *</label>
            <input
              type="number"
              step="0.01"
              value={price}
              onChange={(e) => setPrice(e.target.value)}
              required
              className="w-full px-4 py-2 bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="0.00"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-400 mb-1">SKU</label>
            <input
              type="text"
              value={sku}
              onChange={(e) => setSku(e.target.value)}
              className="w-full px-4 py-2 bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="SKU-001"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-400 mb-1">Category</label>
            <input
              type="text"
              value={category}
              onChange={(e) => setCategory(e.target.value)}
              className="w-full px-4 py-2 bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="Category"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-400 mb-1">Status</label>
            <select
              value={status}
              onChange={(e) => setStatus(e.target.value)}
              className="w-full px-4 py-2 bg-gray-700 rounded-lg focus:outline-none"
            >
              <option value="active">Active</option>
              <option value="draft">Draft</option>
              <option value="archived">Archived</option>
            </select>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-400 mb-1">Markup %</label>
            <input
              type="number"
              value={markupPercentage}
              onChange={(e) => setMarkupPercentage(Number(e.target.value))}
              className="w-full px-4 py-2 bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              min="0"
              max="500"
            />
          </div>

          <div className="col-span-2">
            <label className="block text-sm font-medium text-gray-400 mb-1">
              Image URLs (one per line)
            </label>
            <textarea
              value={images}
              onChange={(e) => setImages(e.target.value)}
              rows={3}
              className="w-full px-4 py-2 bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="https://example.com/image1.jpg&#10;https://example.com/image2.jpg"
            />
            {images && (
              <div className="mt-2 flex flex-wrap gap-2">
                {images.split('\n').filter(url => url.trim()).map((url, idx) => (
                  <img
                    key={idx}
                    src={url.trim()}
                    alt={`Preview ${idx + 1}`}
                    className="h-16 w-16 object-cover rounded"
                    onError={(e) => { (e.target as HTMLImageElement).style.display = 'none'; }}
                  />
                ))}
              </div>
            )}
          </div>
        </div>

        {/* Platform Sync (only for editing) */}
        {product && (
          <div className="border-t border-gray-700 pt-4">
            <h4 className="font-medium mb-3">Sync to Platforms</h4>
            <div className="flex flex-wrap gap-2">
              {['etsy', 'shopify'].map((platform) => {
                const isSynced = getPlatformLinks().includes(platform);
                const isSyncing = syncingTo === platform;
                return (
                  <button
                    key={platform}
                    type="button"
                    onClick={() => handleSyncToPlatform(platform)}
                    disabled={isSyncing}
                    className={`px-4 py-2 rounded-lg flex items-center space-x-2 ${
                      isSynced
                        ? 'bg-green-900/50 text-green-400 border border-green-700'
                        : 'bg-gray-700 hover:bg-gray-600'
                    }`}
                  >
                    <span>{platform === 'etsy' ? '🛍️' : '🛒'}</span>
                    <span className="capitalize">{platform}</span>
                    {isSynced && <span>✓</span>}
                    {isSyncing && <span className="animate-spin">⏳</span>}
                  </button>
                );
              })}
            </div>
          </div>
        )}

        <div className="flex justify-end space-x-3 pt-4">
          <button
            type="button"
            onClick={onClose}
            className="px-6 py-2 bg-gray-600 hover:bg-gray-700 rounded-lg"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={saving || !title.trim() || !price}
            className="px-6 py-2 bg-green-600 hover:bg-green-700 rounded-lg disabled:opacity-50"
          >
            {saving ? 'Saving...' : product ? 'Update Product' : 'Add Product'}
          </button>
        </div>
      </form>
    </div>
  );
}
