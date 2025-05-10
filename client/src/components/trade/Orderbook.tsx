'use client';

import { useEffect, useState } from 'react';
import axios from 'axios';

interface OrderDetails {
  quantity: string;
  type_: 'Bid' | 'Ask';
}

interface DepthResponse {
  type: string;
  payload: {
    orders: {
      [price: string]: OrderDetails;
    };
  };
}

interface Order {
  price: number;
  quantity: number;
}

interface OrderBookData {
  bids: Order[];
  asks: Order[];
}

export default function OrderBook() {
  const [orderBook, setOrderBook] = useState<OrderBookData>({
    bids: [],
    asks: [],
  });
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const calculateVolume = (orders: Order[]): number => {
    return orders.reduce((sum, order) => sum + order.quantity, 0);
  };

  useEffect(() => {
    const fetchOrderBook = async (): Promise<void> => {
      try {
        setLoading(true);
        const response = await axios.get<DepthResponse>(
          'http://localhost:8080/api/v1/depth?market=SOL_USDC'
        );
        const data = response.data;

        if (data.type === 'DEPTH' && data.payload && data.payload.orders) {
          const orders = data.payload.orders;
          const processedOrders: OrderBookData = {
            bids: [],
            asks: [],
          };

          Object.entries(orders).forEach(([price, details]) => {
            const entry: Order = {
              price: parseFloat(price),
              quantity: parseFloat(details.quantity),
            };

            if (details.type_ === 'Bid') {
              processedOrders.bids.push(entry);
            } else if (details.type_ === 'Ask') {
              processedOrders.asks.push(entry);
            }
          });

          processedOrders.bids.sort((a, b) => b.price - a.price);
          processedOrders.asks.sort((a, b) => a.price - b.price);

          setOrderBook(processedOrders);
        }
      } catch (err) {
        console.error('Failed to fetch order book:', err);
        setError('Failed to load order book data');
      } finally {
        setLoading(false);
      }
    };

    fetchOrderBook();

    const interval = setInterval(fetchOrderBook, 5000);

    return () => clearInterval(interval);
  }, []);

  const bidVolume = calculateVolume(orderBook.bids);
  const askVolume = calculateVolume(orderBook.asks);
  const totalVolume = bidVolume + askVolume;
  const bidPercentage = totalVolume > 0 ? (bidVolume / totalVolume) * 100 : 50;
  const askPercentage = totalVolume > 0 ? (askVolume / totalVolume) * 100 : 50;

  if (loading && orderBook.bids.length === 0 && orderBook.asks.length === 0) {
    return <div className="flex justify-center p-8">Loading order book...</div>;
  }

  if (error) {
    return <div className="text-red-500 p-4">{error}</div>;
  }

  return (
    <div className="flex flex-col flex-grow h-full border-t border-border/20 pt-2">
      <div className="flex justify-between items-center mb-4">
        <div className="flex items-center gap-4">
          <div className="flex items-center text-xs">
            <div className="w-4 h-4 bg-green-800 mr-2"></div>
            <span>Bids</span>
          </div>
          <div className="flex items-center text-xs">
            <div className="w-4 h-4 bg-red-800 mr-2"></div>
            <span>Asks</span>
          </div>
        </div>
      </div>

      <div className="grid grid-cols-4 text-xs text-muted-foreground mb-2">
        <div>Size</div>
        <div className="text-center">Bid</div>
        <div className="text-center">Ask</div>
        <div className="text-right">Size</div>
      </div>

      <div className="flex-grow overflow-y-auto">
        {Array.from({
          length: Math.max(
            10,
            Math.max(orderBook.bids.length, orderBook.asks.length)
          ),
        }).map((_, index) => {
          const bid = orderBook.bids[index] || { price: null, quantity: null };
          const ask = orderBook.asks[index] || { price: null, quantity: null };

          return (
            <div
              key={index}
              className={`grid grid-cols-4 text-xs mb-1 ${
                index % 2 === 0 ? 'bg-secondary/30' : ''
              }`}
            >
              <div>{bid.quantity !== null ? bid.quantity.toFixed(5) : '-'}</div>
              <div className="text-green-500 text-center">
                {bid.price !== null ? bid.price.toFixed(3) : '-'}
              </div>
              <div className="text-red-500 text-center">
                {ask.price !== null ? ask.price.toFixed(3) : '-'}
              </div>
              <div className="text-right">
                {ask.quantity !== null ? ask.quantity.toFixed(5) : '-'}
              </div>
            </div>
          );
        })}
      </div>

      {/* Percentage Bar - Always at the bottom */}
      <div className="w-full rounded-sm overflow-hidden">
        <div className="flex">
          <div
            className="bg-green-800 text-xs text-center py-1 text-white"
            style={{ width: `${bidPercentage}%` }}
          >
            {bidPercentage.toFixed(1)}%
          </div>
          <div
            className="bg-red-800 text-xs text-center py-1 text-white"
            style={{ width: `${askPercentage}%` }}
          >
            {askPercentage.toFixed(1)}%
          </div>
        </div>
      </div>
    </div>
  );
}
