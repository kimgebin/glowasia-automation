import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import ProductImport from './ProductImport';
import ProductForm from './ProductForm';

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

export default function Products() {
  const [products, setProducts] = useState<Product[]>([]);
  const [filteredProducts, setFilteredProducts] = useState<Product[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState('all');
  const [selectedProducts, setSelectedProducts] = useState<number[]>([]);
  const [showImport, setShowImport] = useState(false);
  const [showForm, setShowForm] = useState(false);
  const [editingProduct, setEditingProduct] = useState<Product | null>(null);
  const [markupPercentage, setMarkupPercentage] = useState(30);
  const [bulkAction, setBulkAction] = useState('');

  useEffect(() => {
    loadProducts();
    loadMarkupPercentage();
  }, []);

  useEffect(() => {
    filterProducts();
  }, [searchTerm, statusFilter, products]);

  const loadProducts = async () => {
    try {
      const data = await invoke<Product[]>('get_products');
      setProducts(data);
      setFilteredProducts(data);
    } catch (e) {
      console.error('Failed to load products:', e);
    } finally {
      setLoading(false);
    }
  };

  const loadMarkupPercentage = async () => {
    try {
      const pct = await invoke<number>('get_markup_percentage');
      setMarkupPercentage(pct);
    } catch (e) {
      console.error('Failed to load markup:', e);
    }
  };

  const filterProducts = () => {
    let filtered = [...products];
    
    if (searchTerm) {
      filtered = filtered.filter(p => 
        p.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
        p.sku?.toLowerCase().includes(searchTerm.toLowerCase()) ||
        p.category?.toLowerCase().includes(searchTerm.toLowerCase())
      );
    }
    
    if (statusFilter !== 'all') {
      filtered = filtered.filter(p => p.status === statusFilter);
    }
    
    setFilteredProducts(filtered);
  };

  const handleSelectAll = (checked: boolean) => {
    if (checked) {
      setSelectedProducts(filteredProducts.map(p => p.id));
    } else {
      setSelectedProducts([]);
    }
  };

  const handleSelectProduct = (id: number, checked: boolean) => {
    if (checked) {
      setSelectedProducts([...selectedProducts, id]);
    } else {
      setSelectedProducts(selectedProducts.filter(pid => pid !== id));
    }
  };

  const handleBulkAction = async () => {
    if (!bulkAction || selectedProducts.length === 0) return;
    
    try {
      if (bulkAction === 'delete') {
        await invoke('bulk_delete_products', { ids: selectedProducts });
      } else if (bulkAction === 'active' || bulkAction === 'draft' || bulkAction === 'archived') {
        await invoke('bulk_update_product_status', { ids: selectedProducts, status: bulkAction });
      }
      setSelectedProducts([]);
      setBulkAction('');
      loadProducts();
    } catch (e) {
      console.error('Bulk action failed:', e);
    }
  };

  const handleDeleteProduct = async (id: number) => {
    if (!confirm('Delete this product?')) return;
    try {
      await invoke('delete_product', { id });
      loadProducts();
    } catch (e) {
      console.error('Failed to delete:', e);
    }
  };

  const handleEditProduct = (product: Product) => {
    setEditingProduct(product);
    setShowForm(true);
  };

  const handleApplyMarkup = async () => {
    const updates: [number, number][] = [];
    for (const p of products.filter(p => p.cost && p.cost > 0 && p.status === 'active')) {
      const newPrice = p.cost! * (1 + markupPercentage / 100);
      updates.push([p.id, Math.round(newPrice * 100) / 100]);
    }
    if (updates.length > 0) {
      try {
        await invoke('bulk_update_product_prices', { prices: updates });
        loadProducts();
      } catch (e) {
        console.error('Failed to update prices:', e);
      }
    }
  };

  const getPlatformLinks = (links: string | null) => {
    if (!links) return [];
    try {
      return Object.keys(JSON.parse(links));
    } catch {
      return [];
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500"></div>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold">📦 Product Management</h2>
          <p className="text-gray-400">{products.length} products total</p>
        </div>
        <div className="flex space-x-3">
          <button
            onClick={() => setShowImport(true)}
            className="px-4 py-2 bg-purple-600 hover:bg-purple-700 rounded-lg flex items-center space-x-2"
          >
            <span>📥</span>
            <span>Import CSV</span>
          </button>
          <button
            onClick={() => { setEditingProduct(null); setShowForm(true); }}
            className="px-4 py-2 bg-green-600 hover:bg-green-700 rounded-lg flex items-center space-x-2"
          >
            <span>➕</span>
            <span>Add Product</span>
          </button>
        </div>
      </div>

      {/* Markup Settings */}
      <div className="bg-gray-800 rounded-lg p-4 flex items-center justify-between">
        <div className="flex items-center space-x-4">
          <span className="text-gray-400">Default Markup:</span>
          <input
            type="number"
            value={markupPercentage}
            onChange={(e) => setMarkupPercentage(Number(e.target.value))}
            className="w-20 px-3 py-1 bg-gray-700 rounded-lg text-center"
            min="0"
            max="500"
          />
          <span className="text-gray-400">%</span>
        </div>
        <button
          onClick={handleApplyMarkup}
          className="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg"
        >
          Apply to All Products with Cost
        </button>
      </div>

      {/* Filters */}
      <div className="bg-gray-800 rounded-lg p-4 flex items-center space-x-4">
        <input
          type="text"
          placeholder="Search products..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          className="flex-1 px-4 py-2 bg-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <select
          value={statusFilter}
          onChange={(e) => setStatusFilter(e.target.value)}
          className="px-4 py-2 bg-gray-700 rounded-lg focus:outline-none"
        >
          <option value="all">All Status</option>
          <option value="active">Active</option>
          <option value="draft">Draft</option>
          <option value="archived">Archived</option>
        </select>
      </div>

      {/* Bulk Actions */}
      {selectedProducts.length > 0 && (
        <div className="bg-blue-900/50 rounded-lg p-4 flex items-center justify-between">
          <span>{selectedProducts.length} products selected</span>
          <div className="flex items-center space-x-3">
            <select
              value={bulkAction}
              onChange={(e) => setBulkAction(e.target.value)}
              className="px-4 py-2 bg-gray-700 rounded-lg"
            >
              <option value="">Select Action</option>
              <option value="active">Set Active</option>
              <option value="draft">Set Draft</option>
              <option value="archived">Set Archived</option>
              <option value="delete">Delete</option>
            </select>
            <button
              onClick={handleBulkAction}
              disabled={!bulkAction}
              className="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg disabled:opacity-50"
            >
              Apply
            </button>
            <button
              onClick={() => setSelectedProducts([])}
              className="px-4 py-2 bg-gray-600 hover:bg-gray-700 rounded-lg"
            >
              Cancel
            </button>
          </div>
        </div>
      )}

      {/* Products Table */}
      <div className="bg-gray-800 rounded-lg overflow-hidden">
        <table className="w-full">
          <thead className="bg-gray-700">
            <tr>
              <th className="px-4 py-3 text-left">
                <input
                  type="checkbox"
                  checked={selectedProducts.length === filteredProducts.length && filteredProducts.length > 0}
                  onChange={(e) => handleSelectAll(e.target.checked)}
                  className="rounded"
                />
              </th>
              <th className="px-4 py-3 text-left">Product</th>
              <th className="px-4 py-3 text-left">SKU</th>
              <th className="px-4 py-3 text-left">Category</th>
              <th className="px-4 py-3 text-right">Cost</th>
              <th className="px-4 py-3 text-right">Price</th>
              <th className="px-4 py-3 text-center">Status</th>
              <th className="px-4 py-3 text-center">Platforms</th>
              <th className="px-4 py-3 text-center">Actions</th>
            </tr>
          </thead>
          <tbody>
            {filteredProducts.length === 0 ? (
              <tr>
                <td colSpan={9} className="px-4 py-8 text-center text-gray-400">
                  No products found. Add some products or import from CSV.
                </td>
              </tr>
            ) : (
              filteredProducts.map((product) => (
                <tr key={product.id} className="border-t border-gray-700 hover:bg-gray-700/50">
                  <td className="px-4 py-3">
                    <input
                      type="checkbox"
                      checked={selectedProducts.includes(product.id)}
                      onChange={(e) => handleSelectProduct(product.id, e.target.checked)}
                      className="rounded"
                    />
                  </td>
                  <td className="px-4 py-3">
                    <div className="flex items-center space-x-3">
                      {product.images && (
                        <img
                          src={JSON.parse(product.images)[0]}
                          alt=""
                          className="w-10 h-10 rounded object-cover"
                          onError={(e) => { (e.target as HTMLImageElement).style.display = 'none'; }}
                        />
                      )}
                      <div>
                        <p className="font-medium">{product.title}</p>
                        {product.description && (
                          <p className="text-sm text-gray-400 truncate max-w-xs">
                            {product.description.substring(0, 50)}...
                          </p>
                        )}
                      </div>
                    </div>
                  </td>
                  <td className="px-4 py-3 text-gray-400">{product.sku || '-'}</td>
                  <td className="px-4 py-3 text-gray-400">{product.category || '-'}</td>
                  <td className="px-4 py-3 text-right text-red-400">
                    {product.cost ? `$${product.cost.toFixed(2)}` : '-'}
                  </td>
                  <td className="px-4 py-3 text-right text-green-400 font-medium">
                    ${product.price.toFixed(2)}
                  </td>
                  <td className="px-4 py-3 text-center">
                    <span className={`px-2 py-1 rounded text-xs ${
                      product.status === 'active' ? 'bg-green-900 text-green-400' :
                      product.status === 'draft' ? 'bg-yellow-900 text-yellow-400' :
                      'bg-gray-700 text-gray-400'
                    }`}>
                      {product.status}
                    </span>
                  </td>
                  <td className="px-4 py-3 text-center">
                    <div className="flex justify-center space-x-1">
                      {getPlatformLinks(product.platform_links).map(platform => (
                        <span key={platform} className="text-sm" title={platform}>
                          {platform === 'etsy' ? '🛍️' : platform === 'shopify' ? '🛒' : '📦'}
                        </span>
                      ))}
                    </div>
                  </td>
                  <td className="px-4 py-3 text-center">
                    <div className="flex justify-center space-x-2">
                      <button
                        onClick={() => handleEditProduct(product)}
                        className="p-1 hover:bg-gray-600 rounded"
                        title="Edit"
                      >
                        ✏️
                      </button>
                      <button
                        onClick={() => handleDeleteProduct(product.id)}
                        className="p-1 hover:bg-gray-600 rounded text-red-400"
                        title="Delete"
                      >
                        🗑️
                      </button>
                    </div>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {/* Import Modal */}
      {showImport && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-gray-800 rounded-lg p-6 w-full max-w-2xl max-h-[80vh] overflow-auto">
            <ProductImport
              onClose={() => setShowImport(false)}
              onImportComplete={() => { setShowImport(false); loadProducts(); }}
            />
          </div>
        </div>
      )}

      {/* Product Form Modal */}
      {showForm && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-gray-800 rounded-lg p-6 w-full max-w-2xl max-h-[80vh] overflow-auto">
            <ProductForm
              product={editingProduct}
              onClose={() => setShowForm(false)}
              onSave={() => { setShowForm(false); loadProducts(); }}
            />
          </div>
        </div>
      )}
    </div>
  );
}
