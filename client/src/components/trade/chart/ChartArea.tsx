'use client';

import { useEffect, useRef } from 'react';
import ChartControl from './ChartControl';
import { ChartManager } from '@/src/utils/chartManager';
import { KLine } from '@/src/utils/types';
import { getKlines } from '@/src/utils/httpClient';

export default function ChartArea({ market }: { market: string }) {
  const chartContainerRef = useRef<HTMLDivElement>(null);
  const chartManagerRef = useRef<ChartManager | null>(null);

  const init = async () => {
    if (!chartContainerRef.current) return;

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

    if (chartManagerRef.current) {
      chartManagerRef.current.destroy();
    }

    const chartManager = new ChartManager(
      chartContainerRef.current,
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

    chartManagerRef.current = chartManager;
  };

  useEffect(() => {
    init();

    const handleResize = () => {
      if (chartManagerRef.current && chartContainerRef.current) {
        chartManagerRef.current.destroy();
        init();
      }
    };

    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('resize', handleResize);
      if (chartManagerRef.current) {
        chartManagerRef.current.destroy();
      }
    };
  }, [market]);

  return (
    <>
      <ChartControl />
      <div className="flex-1 bg-card flex items-center justify-center">
        <div
          ref={chartContainerRef}
          className="text-muted-foreground w-full h-full"
          style={{ minHeight: '300px' }}
        ></div>
      </div>
    </>
  );
}
