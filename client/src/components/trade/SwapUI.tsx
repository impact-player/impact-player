'use client';

import { useState } from 'react';
import { Button } from '@/src/components/ui/button';
import { Input } from '@/src/components/ui/input';
import { ArrowDownIcon } from '@/src/components/icons';

interface SwapUIProps {
  baseCurrency: string;
  quoteCurrency: string;
}

type OrderType = 'BUY' | 'SELL';
type OrderMode = 'MKT' | 'LIMIT';

export default function SwapUI({ baseCurrency, quoteCurrency }: SwapUIProps) {
  // State for tracking active tabs
  const [orderType, setOrderType] = useState<OrderType>('BUY');
  const [orderMode, setOrderMode] = useState<OrderMode>('MKT');

  // Handler functions
  const handleOrderTypeChange = (type: OrderType) => {
    setOrderType(type);
  };

  const handleOrderModeChange = (mode: OrderMode) => {
    setOrderMode(mode);
  };

  return (
    <>
      <div className="flex mb-4">
        <Button
          className={`flex-1 ${
            orderType === 'BUY'
              ? 'bg-white text-green-500'
              : 'bg-card hover:bg-card/90 text-green-500'
          } rounded-l-xl rounded-r-none border border-border`}
          onClick={() => handleOrderTypeChange('BUY')}
        >
          BUY
        </Button>
        <Button
          className={`flex-1 ${
            orderType === 'SELL'
              ? 'bg-white text-red-500'
              : 'bg-card hover:bg-card/90 text-red-500'
          } rounded-r-xl rounded-l-none border border-l-0 border-border`}
          onClick={() => handleOrderTypeChange('SELL')}
        >
          SELL
        </Button>
      </div>

      <div className="flex mb-4">
        <Button
          className={`flex-1 ${
            orderMode === 'MKT'
              ? 'bg-white text-foreground'
              : 'bg-card hover:bg-card/90 text-foreground'
          } rounded-l-xl rounded-r-none border border-border`}
          onClick={() => handleOrderModeChange('MKT')}
        >
          MKT
        </Button>
        <Button
          className={`flex-1 ${
            orderMode === 'LIMIT'
              ? 'bg-white text-background'
              : 'bg-card hover:bg-card/90 text-foreground'
          } rounded-r-xl rounded-l-none border border-l-0 border-border`}
          onClick={() => handleOrderModeChange('LIMIT')}
        >
          LIMIT
        </Button>
      </div>

      {/* Only show Limit Price input when LIMIT mode is selected */}
      {orderMode === 'LIMIT' && (
        <div className="mb-4">
          <div className="flex justify-between items-center text-sm mb-2">
            <span>Limit Price:</span>
            <div className="flex gap-2">
              <Button
                size="sm"
                variant="ghost"
                className="text-amber-500 hover:text-amber-600 px-2 h-6"
              >
                BID
              </Button>
              <Button
                size="sm"
                variant="ghost"
                className="text-muted-foreground hover:text-foreground px-2 h-6"
              >
                MID
              </Button>
              <Button
                size="sm"
                variant="ghost"
                className="text-muted-foreground hover:text-foreground px-2 h-6"
              >
                ASK
              </Button>
            </div>
            <div className="bg-secondary text-xs rounded-md px-2 py-1">
              {quoteCurrency}
            </div>
          </div>
          <Input className="bg-card border-border" placeholder="enter price" />
        </div>
      )}

      <div className="mb-4">
        <div className="flex justify-between items-center text-sm mb-2">
          <span>Amount:</span>
          <div className="flex gap-2">
            <Button
              size="sm"
              variant="ghost"
              className="text-muted-foreground hover:text-foreground px-2 h-6"
            >
              .5x
            </Button>
            <Button
              size="sm"
              variant="ghost"
              className="text-muted-foreground hover:text-foreground px-2 h-6"
            >
              2x
            </Button>
          </div>
        </div>
        <div className="flex">
          <Input
            className="bg-card border-border rounded-r-none"
            placeholder="0"
          />
          <Button className="bg-secondary hover:bg-secondary/90 border border-border rounded-l-none">
            {baseCurrency}
            <ArrowDownIcon className="h-4 w-4 ml-2" />
          </Button>
        </div>
      </div>

      <div className="flex justify-between items-center text-sm mb-2">
        <span>Execution</span>
        <span className="text-amber-500">0.00 {quoteCurrency}</span>
      </div>

      <Button
        disabled
        className="w-full py-6 bg-secondary/50 hover:bg-secondary/50 text-muted-foreground cursor-not-allowed"
      >
        Sign In to Trade
      </Button>
    </>
  );
}
