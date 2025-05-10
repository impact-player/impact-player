'use client';

import { useEffect, useState } from 'react';
import axios from 'axios';
import { getDepth } from '@/src/utils/httpClient';

interface Order {
  price: number;
  quantity: number;
}

interface OrderBookData {
  bids: Order[];
  asks: Order[];
}

export default function OrderBook({ market }: { market: string }) {
  const [orderBook, setOrderBook] = useState<OrderBookData>({
    bids: [],
    asks: [],
  });
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchOrderBook = async () => {
      try {
        setLoading(true);
        const data = await getDepth(market);

        if (data.type === 'DEPTH' && data.payload?.orders) {
          const bids: Order[] = [];
          const asks: Order[] = [];

          Object.entries(data.payload.orders).forEach(([price, details]) => {
            const entry: Order = {
              price: parseFloat(price),
              quantity: parseFloat(details.quantity),
            };
            details.type_ === 'Bid' ? bids.push(entry) : asks.push(entry);
          });

          bids.sort((a, b) => b.price - a.price);
          asks.sort((a, b) => a.price - b.price);

          setOrderBook({ bids, asks });
        }
      } catch (err) {
        console.error(err);
        setError('Failed to load order book data');
      } finally {
        setLoading(false);
      }
    };

    fetchOrderBook();
    const interval = setInterval(fetchOrderBook, 5000);
    return () => clearInterval(interval);
  }, []);

  if (loading && !orderBook.bids.length && !orderBook.asks.length) {
    return <div className="flex justify-center p-8">Loading order book...</div>;
  }
  if (error) {
    return <div className="text-red-500 p-4">{error}</div>;
  }

  return (
    <div className="flex flex-col h-full border-t border-border/20 pt-2">
      <div className="flex justify-between items-center mb-2 px-2">
        <div className="flex items-center gap-4">
          <div className="flex items-center text-xs">
            <div className="w-3 h-3 bg-green-800 mr-1" />
            Bids
          </div>
          <div className="flex items-center text-xs">
            <div className="w-3 h-3 bg-red-800 mr-1" />
            Asks
          </div>
        </div>
      </div>
      <div className="grid grid-cols-4 text-xs text-muted-foreground px-2 mb-2">
        <div>Size</div>
        <div className="text-center">Bid</div>
        <div className="text-center">Ask</div>
        <div className="text-right">Size</div>
      </div>

      <div className="flex-grow overflow-y-auto px-2 text-center">
        {Array.from({
          length: Math.max(50, orderBook.bids.length, orderBook.asks.length),
        }).map((_, idx) => {
          const bid = orderBook.bids[idx] || { price: null, quantity: null };
          const ask = orderBook.asks[idx] || { price: null, quantity: null };
          return (
            <div
              key={idx}
              className={`grid grid-cols-4 text-xs py-1 ${
                idx % 2 === 0 ? 'bg-secondary/30' : ''
              }`}
            >
              <div>{bid.quantity != null ? bid.quantity.toFixed(5) : '-'}</div>
              <div className="text-green-500 text-center h-6">
                {bid.price != null ? bid.price.toFixed(3) : '-'}
              </div>
              <div className="text-red-500 text-center h-6">
                {ask.price != null ? ask.price.toFixed(3) : '-'}
              </div>
              <div className="text-right">
                {ask.quantity != null ? ask.quantity.toFixed(5) : '-'}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
