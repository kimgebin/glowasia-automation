import { useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface CsvImportResult {
  imported: number;
  errors: string[];
}

interface ParsedRow {
  title: string;
  description: string;
  price: string;
  cost: string;
  image_url: string;
  sku: string;
  category: string;
}

export default function ProductImport({ onClose, onImportComplete }: {
  onClose: () => void;
  onImportComplete: () => void;
}) {
  const [dragActive, setDragActive] = useState(false);
  const [file, setFile] = useState<File | null>(null);
  const [parsedData, setParsedData] = useState<ParsedRow[]>([]);
  const [errors, setErrors] = useState<string[]>([]);
  const [importing, setImporting] = useState(false);
  const [step, setStep] = useState<'upload' | 'preview' | 'result'>('upload');

  const handleDrag = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (e.type === 'dragenter' || e.type === 'dragover') {
      setDragActive(true);
    } else if (e.type === 'dragleave') {
      setDragActive(false);
    }
  }, []);

  const parseCSV = (content: string): ParsedRow[] => {
    const lines = content.split('\n').filter(l => l.trim());
    if (lines.length < 2) return [];
    
    const headers = lines[0].split(',').map(h => h.trim().toLowerCase());
    const rows: ParsedRow[] = [];
    
    const getCol = (cols: string[], name: string): string => {
      const idx = headers.findIndex(h => h.includes(name));
      return idx >= 0 && idx < cols.length ? cols[idx].trim().replace(/^"|"$/g, '') : '';
    };
    
    for (let i = 1; i < lines.length; i++) {
      const cols = lines[i].split(',').map(c => c.trim().replace(/^"|"$/g, ''));
      rows.push({
        title: getCol(cols, 'title'),
        description: getCol(cols, 'description'),
        price: getCol(cols, 'price'),
        cost: getCol(cols, 'cost'),
        image_url: getCol(cols, 'image'),
        sku: getCol(cols, 'sku'),
        category: getCol(cols, 'category'),
      });
    }
    
    return rows;
  };

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setDragActive(false);
    
    const f = e.dataTransfer.files[0];
    if (f && f.name.endsWith('.csv')) {
      setFile(f);
      const reader = new FileReader();
      reader.onload = (event) => {
        const content = event.target?.result as string;
        const data = parseCSV(content);
        if (data.length === 0) {
          setErrors(['CSV must have title and price columns, plus at least one data row']);
        } else {
          const parseErrors: string[] = [];
          data.forEach((row, idx) => {
            if (!row.title) parseErrors.push(`Row ${idx + 2}: Missing title`);
            if (!row.price || isNaN(parseFloat(row.price))) {
              parseErrors.push(`Row ${idx + 2}: Invalid price`);
            }
          });
          setErrors(parseErrors);
          setParsedData(data);
          setStep('preview');
        }
      };
      reader.readAsText(f);
    } else {
      setErrors(['Please upload a CSV file']);
    }
  }, []);

  const handleFileInput = (e: React.ChangeEvent<HTMLInputElement>) => {
    const f = e.target.files?.[0];
    if (f) {
      setFile(f);
      const reader = new FileReader();
      reader.onload = (event) => {
        const content = event.target?.result as string;
        const data = parseCSV(content);
        if (data.length === 0) {
          setErrors(['CSV must have title and price columns, plus at least one data row']);
        } else {
          const parseErrors: string[] = [];
          data.forEach((row, idx) => {
            if (!row.title) parseErrors.push(`Row ${idx + 2}: Missing title`);
            if (!row.price || isNaN(parseFloat(row.price))) {
              parseErrors.push(`Row ${idx + 2}: Invalid price`);
            }
          });
          setErrors(parseErrors);
          setParsedData(data);
          setStep('preview');
        }
      };
      reader.readAsText(f);
    }
  };

  const handleImport = async () => {
    if (!file) return;
    
    setImporting(true);
    try {
      const content = await file.text();
      const result = await invoke<CsvImportResult>('import_products_csv', { csvContent: content });
      setStep('result');
      setErrors(result.errors);
    } catch (e) {
      setErrors([`Import failed: ${e}`]);
    } finally {
      setImporting(false);
    }
  };

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-xl font-bold">📥 Import Products from CSV</h3>
        <button onClick={onClose} className="text-gray-400 hover:text-white text-2xl">&times;</button>
      </div>

      {/* Step 1: Upload */}
      {step === 'upload' && (
        <div>
          <div className="mb-4 p-4 bg-gray-700/50 rounded-lg">
            <h4 className="font-medium mb-2">CSV Format Requirements:</h4>
            <p className="text-sm text-gray-400">
              Required columns: <code className="bg-gray-600 px-1 rounded">title</code>, <code className="bg-gray-600 px-1 rounded">price</code><br/>
              Optional columns: description, cost, image_url, sku, category
            </p>
          </div>

          <div
            onDragEnter={handleDrag}
            onDragLeave={handleDrag}
            onDragOver={handleDrag}
            onDrop={handleDrop}
            className={`border-2 border-dashed rounded-lg p-8 text-center transition-colors ${
              dragActive ? 'border-blue-500 bg-blue-500/10' : 'border-gray-600 hover:border-gray-500'
            }`}
          >
            <input
              type="file"
              accept=".csv"
              onChange={handleFileInput}
              className="hidden"
              id="csv-upload"
            />
            <label htmlFor="csv-upload" className="cursor-pointer">
              <div className="text-4xl mb-2">📄</div>
              <p className="text-lg">Drag & drop your CSV file here</p>
              <p className="text-gray-400 text-sm mt-1">or click to browse</p>
            </label>
          </div>

          {errors.length > 0 && (
            <div className="mt-4 p-4 bg-red-900/30 border border-red-700 rounded-lg">
              <h4 className="font-medium text-red-400 mb-2">Errors:</h4>
              <ul className="text-sm text-red-300 space-y-1">
                {errors.map((err, i) => <li key={i}>• {err}</li>)}
              </ul>
            </div>
          )}

          <div className="mt-6 flex justify-end space-x-3">
            <button onClick={onClose} className="px-4 py-2 bg-gray-600 hover:bg-gray-700 rounded-lg">
              Cancel
            </button>
          </div>
        </div>
      )}

      {/* Step 2: Preview */}
      {step === 'preview' && (
        <div>
          <div className="mb-4 flex items-center justify-between">
            <p className="text-gray-400">
              Found <span className="text-white font-medium">{parsedData.length}</span> products to import
              {file && ` from ${file.name}`}
            </p>
            <button
              onClick={() => { setStep('upload'); setParsedData([]); setFile(null); setErrors([]); }}
              className="text-sm text-blue-400 hover:text-blue-300"
            >
              Choose different file
            </button>
          </div>

          <div className="max-h-64 overflow-auto bg-gray-900 rounded-lg">
            <table className="w-full text-sm">
              <thead className="bg-gray-700 sticky top-0">
                <tr>
                  <th className="px-3 py-2 text-left">#</th>
                  <th className="px-3 py-2 text-left">Title</th>
                  <th className="px-3 py-2 text-right">Price</th>
                  <th className="px-3 py-2 text-right">Cost</th>
                  <th className="px-3 py-2 text-left">SKU</th>
                  <th className="px-3 py-2 text-left">Category</th>
                </tr>
              </thead>
              <tbody>
                {parsedData.slice(0, 50).map((row, idx) => (
                  <tr key={idx} className="border-t border-gray-700">
                    <td className="px-3 py-2 text-gray-400">{idx + 1}</td>
                    <td className="px-3 py-2">{row.title || <span className="text-red-400">MISSING</span>}</td>
                    <td className="px-3 py-2 text-right text-green-400">
                      {row.price || <span className="text-red-400">INVALID</span>}
                    </td>
                    <td className="px-3 py-2 text-right text-gray-400">{row.cost || '-'}</td>
                    <td className="px-3 py-2 text-gray-400">{row.sku || '-'}</td>
                    <td className="px-3 py-2 text-gray-400">{row.category || '-'}</td>
                  </tr>
                ))}
              </tbody>
            </table>
            {parsedData.length > 50 && (
              <p className="p-2 text-center text-gray-400 text-sm">
                ...and {parsedData.length - 50} more rows
              </p>
            )}
          </div>

          {errors.length > 0 && (
            <div className="mt-4 p-4 bg-yellow-900/30 border border-yellow-700 rounded-lg">
              <h4 className="font-medium text-yellow-400 mb-2">Warnings ({errors.length}):</h4>
              <ul className="text-sm text-yellow-300 space-y-1 max-h-32 overflow-auto">
                {errors.slice(0, 20).map((err, i) => <li key={i}>• {err}</li>)}
                {errors.length > 20 && <li>...and {errors.length - 20} more</li>}
              </ul>
            </div>
          )}

          <div className="mt-6 flex justify-end space-x-3">
            <button onClick={onClose} className="px-4 py-2 bg-gray-600 hover:bg-gray-700 rounded-lg">
              Cancel
            </button>
            <button
              onClick={handleImport}
              disabled={importing}
              className="px-6 py-2 bg-green-600 hover:bg-green-700 rounded-lg disabled:opacity-50"
            >
              {importing ? 'Importing...' : `Import ${parsedData.length} Products`}
            </button>
          </div>
        </div>
      )}

      {/* Step 3: Result */}
      {step === 'result' && (
        <div className="text-center py-8">
          <div className="text-6xl mb-4">✅</div>
          <h3 className="text-xl font-bold mb-2">Import Complete!</h3>
          
          {errors.length > 0 ? (
            <div className="mt-4 p-4 bg-yellow-900/30 border border-yellow-700 rounded-lg text-left max-w-md mx-auto">
              <h4 className="font-medium text-yellow-400 mb-2">
                {errors.length} row(s) had issues and were skipped:
              </h4>
              <ul className="text-sm text-yellow-300 space-y-1 max-h-40 overflow-auto">
                {errors.slice(0, 30).map((err, i) => <li key={i}>• {err}</li>)}
                {errors.length > 30 && <li>...and {errors.length - 30} more</li>}
              </ul>
            </div>
          ) : (
            <p className="text-gray-400">All products imported successfully!</p>
          )}

          <div className="mt-6 flex justify-center space-x-3">
            <button
              onClick={() => { setStep('upload'); setParsedData([]); setFile(null); setErrors([]); }}
              className="px-4 py-2 bg-gray-600 hover:bg-gray-700 rounded-lg"
            >
              Import More
            </button>
            <button
              onClick={onImportComplete}
              className="px-6 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg"
            >
              Done
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
