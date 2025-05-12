'use client';

import { useState, useEffect } from 'react';
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '@/src/components/ui/tabs';
import { Trade } from '@/src/utils/types';
import { getTrades } from '@/src/utils/httpClient';

export default function BottomTable() {
  const [trades, setTrades] = useState<Trade[]>([]);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  const fetchTrades = async () => {
    try {
      setLoading(true);
      setError(null);
      const response = await getTrades('SOL/USD');
      if (response.success) {
        setTrades(response.data);
      } else {
        setError('Failed to fetch trades');
      }
    } catch (err) {
      setError('Error fetching trades');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleTimeString();
  };

  useEffect(() => {
    fetchTrades();
  }, []);

  return (
    <div className="h-48 border-t border-border/20">
      <Tabs
        defaultValue="trade"
        onValueChange={(value) => {
          if (value === 'trade') fetchTrades();
        }}
      >
        <div className="border-b border-border/20 sticky top-0 bg-background z-10">
          <TabsList className="bg-background border-b border-border/20 rounded-none">
            <TabsTrigger
              value="trade"
              className="data-[state=active]:bg-background data-[state=active]:border-b-2 data-[state=active]:border-foreground rounded-none"
            >
              Trade
            </TabsTrigger>
            <TabsTrigger
              value="advanced"
              className="data-[state=active]:bg-background data-[state=active]:border-b-2 data-[state=active]:border-foreground rounded-none"
            >
              Advanced
            </TabsTrigger>
            <TabsTrigger
              value="recent"
              className="data-[state=active]:bg-background data-[state=active]:border-b-2 data-[state=active]:border-foreground rounded-none"
            >
              Recent
            </TabsTrigger>
            <TabsTrigger
              value="algos"
              className="data-[state=active]:bg-background data-[state=active]:border-b-2 data-[state=active]:border-foreground rounded-none"
            >
              Algos
            </TabsTrigger>
          </TabsList>
        </div>

        <TabsContent value="trade" className="p-2">
          {loading ? (
            <div className="flex justify-center items-center h-12">
              <div className="text-sm text-muted-foreground">
                Loading trades...
              </div>
            </div>
          ) : error ? (
            <div className="flex justify-center items-center h-12">
              <div className="text-sm text-red-500">{error}</div>
            </div>
          ) : trades.length === 0 ? (
            <div className="flex justify-center items-center h-12">
              <div className="text-sm text-muted-foreground">
                No trades available
              </div>
            </div>
          ) : (
            <div className="flex flex-col h-36">
              <div className="sticky top-0 bg-background z-10">
                <table className="w-full text-sm">
                  <thead>
                    <tr className="text-xs text-muted-foreground border-b border-border/10">
                      <th className="text-left py-2">Price</th>
                      <th className="text-left py-2">Quantity</th>
                      <th className="text-left py-2">Currency</th>
                    </tr>
                  </thead>
                </table>
              </div>
              <div className="overflow-y-auto flex-grow">
                <table className="w-full text-sm">
                  <tbody>
                    {trades.map((trade, index) => (
                      <tr key={index} className="border-t border-border/10">
                        <td className="py-2 pr-8">${trade.price.toFixed(2)}</td>
                        <td className="py-2 pr-18">
                          {trade.quantity.toFixed(2)}
                        </td>
                        <td className="py-2 pr-18">{trade.currency_code}</td>
                        {/* <td className="py-2 pr-8">{formatDate(trade.time)}</td> */}
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          )}
        </TabsContent>

        <TabsContent value="advanced" className="p-4">
          <div className="flex flex-col">
            <div className="text-sm text-muted-foreground">
              Advanced trading options will appear here
            </div>
          </div>
        </TabsContent>

        <TabsContent value="recent" className="p-4">
          <div className="flex flex-col">
            <div className="text-sm text-muted-foreground">
              Recent trades will appear here
            </div>
          </div>
        </TabsContent>

        <TabsContent value="algos" className="p-4">
          <div className="flex flex-col">
            <div className="text-sm text-muted-foreground">
              Trading algorithms will appear here
            </div>
          </div>
        </TabsContent>
      </Tabs>
    </div>
  );
}
