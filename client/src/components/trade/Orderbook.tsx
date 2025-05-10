// OrderBook.tsx
export default function OrderBook() {
  return (
    <div className="mt-8 border-t border-border/20 pt-4">
      <div className="flex justify-between items-center mb-4">
        <h3 className="font-bold">OrderBook</h3>
        <div className="flex items-center gap-2">
          <div className="flex items-center text-xs">
            <div className="w-3 h-3 bg-green-500/20 mr-1"></div>
            <span>Bids</span>
          </div>
          <div className="flex items-center text-xs">
            <div className="w-3 h-3 bg-red-500/20 mr-1"></div>
            <span>Asks</span>
          </div>
        </div>
      </div>

      <div className="grid grid-cols-6 text-xs text-muted-foreground mb-2">
        <div>Size</div>
        <div className="col-span-2 text-center">Bid</div>
        <div className="col-span-2 text-center">Ask</div>
        <div className="text-right">Size</div>
      </div>

      <div className="max-h-[400px] overflow-y-auto">
        {/* Ask Rows */}
        <div className="grid grid-cols-6 text-xs mb-1 bg-secondary/30">
          <div>0.00004</div>
          <div className="col-span-2 text-green-500 text-center">94.254</div>
          <div className="col-span-2 text-red-500 text-center">94.255</div>
          <div className="text-right">0.05000</div>
        </div>
        <div className="grid grid-cols-6 text-xs mb-1">
          <div>0.11970</div>
          <div className="col-span-2 text-green-500 text-center">94.252</div>
          <div className="col-span-2 text-red-500 text-center">94.261</div>
          <div className="text-right">0.10608</div>
        </div>
        <div className="grid grid-cols-6 text-xs mb-1 bg-secondary/30">
          <div>0.05000</div>
          <div className="col-span-2 text-green-500 text-center">94.241</div>
          <div className="col-span-2 text-red-500 text-center">94.262</div>
          <div className="text-right">0.12900</div>
        </div>
        <div className="grid grid-cols-6 text-xs mb-1">
          <div>0.10611</div>
          <div className="col-span-2 text-green-500 text-center">94.222</div>
          <div className="col-span-2 text-red-500 text-center">94.274</div>
          <div className="text-right">0.52632</div>
        </div>
        <div className="grid grid-cols-6 text-xs mb-1 bg-secondary/30">
          <div>0.52632</div>
          <div className="col-span-2 text-green-500 text-center">94.217</div>
          <div className="col-span-2 text-red-500 text-center">94.332</div>
          <div className="text-right">0.22710</div>
        </div>
        <div className="grid grid-cols-6 text-xs mb-1">
          <div>0.23880</div>
          <div className="col-span-2 text-green-500 text-center">94.199</div>
          <div className="col-span-2 text-red-500 text-center">94.353</div>
          <div className="text-right">0.00290</div>
        </div>
        <div className="grid grid-cols-6 text-xs mb-1 bg-secondary/30">
          <div>0.45510</div>
          <div className="col-span-2 text-green-500 text-center">94.084</div>
          <div className="col-span-2 text-red-500 text-center">94.426</div>
          <div className="text-right">0.45440</div>
        </div>
        <div className="grid grid-cols-6 text-xs mb-1">
          <div>0.00094</div>
          <div className="col-span-2 text-green-500 text-center">94.005</div>
          <div className="col-span-2 text-red-500 text-center">94.460</div>
          <div className="text-right">0.00179</div>
        </div>
        <div className="grid grid-cols-6 text-xs mb-1 bg-secondary/30">
          <div>0.00020</div>
          <div className="col-span-2 text-green-500 text-center">93.854</div>
          <div className="col-span-2 text-red-500 text-center">94.495</div>
          <div className="text-right">0.00380</div>
        </div>
        <div className="grid grid-cols-6 text-xs mb-1">
          <div>0.00140</div>
          <div className="col-span-2 text-green-500 text-center">93.816</div>
          <div className="col-span-2 text-red-500 text-center">94.636</div>
          <div className="text-right">0.00471</div>
        </div>
      </div>

      {/* Percentage Bar */}
      <div className="flex w-full mt-4 rounded-sm overflow-hidden">
        <div
          className="bg-green-900/50 text-xs text-center py-1"
          style={{ width: '61%' }}
        >
          61.0%
        </div>
        <div
          className="bg-red-900/50 text-xs text-center py-1"
          style={{ width: '39%' }}
        >
          39.0%
        </div>
      </div>
    </div>
  );
}
