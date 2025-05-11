'use client';

import { useEffect, useRef } from 'react';
import ChartControl from './ChartControl';
import { ChartManager } from '@/src/utils/chartManager';
import { KLine } from '@/src/utils/types';
import { getKlines } from '@/src/utils/httpClient';

export default function ChartArea({ market }: { market: string }) {
  const chartRef = useRef<HTMLDivElement>(null);
  const chartManagerRef = useRef<ChartManager>(null);

  const init = async () => {
    let klineData: KLine[] = [];
    try {
      klineData = await getKlines(
        market,
        '1h',
        Math.floor(new Date().getTime() - 1000 * 60 * 60 * 24 * 7).toString(),
        Math.floor(new Date().getTime()).toString()
      );
    } catch (error) {
      console.error('Error fetching kline data:', error);
    }

    console.log('klines data: ', klineData);
    if (chartRef) {
      if (chartManagerRef.current) {
        chartManagerRef.current.destroy();
      }
      const chartManager = new ChartManager(
        chartRef.current,
        [
          ...klineData?.map((x) => ({
            close: parseFloat(x.close),
            high: parseFloat(x.high),
            low: parseFloat(x.low),
            open: parseFloat(x.open),
            timestamp: new Date(parseInt(x.end)).getTime(),
          })),
        ].sort((x, y) => (x.timestamp < y.timestamp ? -1 : 1)) || [],
        {
          background: '#0e0f14',
          color: 'white',
        }
      );
      //@ts-ignore
      chartManagerRef.current = chartManager;
    }
  };

  useEffect(() => {
    init();
  }, [market, chartRef]);
  return (
    <>
      <ChartControl />

      <div className="flex-1 bg-card p-4 flex items-center justify-center">
        <div
          ref={chartRef}
          className="text-muted-foreground"
          style={{ height: '520px', width: '100%', marginTop: 4 }}
        ></div>
      </div>
    </>
  );
}
