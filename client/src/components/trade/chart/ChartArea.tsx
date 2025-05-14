'use client';

import { useEffect, useRef } from 'react';
import ChartControl from './ChartControl';
import { ChartManager } from '@/src/utils/chartManager';
import { KLine } from '@/src/utils/types';
import { getKlines } from '@/src/utils/httpClient';

export default function ChartArea({ market }: { market: string }) {
  const containerRef = useRef<HTMLDivElement>(null);
  const managerRef = useRef<ChartManager>(null);
  const lastBarTsRef = useRef<number>(0); // in ms

  // convert API KLine → internal bar
  const toBar = (x: KLine) => {
    const ts = parseInt(x.end, 10) * 1000; // ms
    return {
      timestamp: ts,
      close: parseFloat(x.close),
      volume: parseFloat(x.volume),
    };
  };

  useEffect(() => {
    let intervalId: NodeJS.Timeout;

    // 1) INITIALIZE
    const init = async () => {
      if (!containerRef.current) return;
      const now = Date.now();
      const start = (now - 1000 * 60 * 60).toString(); // 60 min ago
      const end = now.toString();

      let klines: KLine[] = [];
      try {
        klines = await getKlines(market, '1m', start, end);
      } catch (e) {
        console.error('failed to fetch initial history', e);
      }

      const bars = klines.map(toBar).sort((a, b) => a.timestamp - b.timestamp);

      // destroy previous if any
      managerRef.current?.destroy();

      // create new chart
      managerRef.current = new ChartManager(containerRef.current, bars, {
        background: '#0e0f14',
        color: 'white',
      });

      // remember last timestamp
      lastBarTsRef.current = bars[bars.length - 1]?.timestamp ?? now;
    };

    // 2) REAL-TIME UPDATES
    const startUpdates = () => {
      intervalId = setInterval(async () => {
        const mgr = managerRef.current;
        if (!mgr) return;

        const since = lastBarTsRef.current + 1;
        const until = Date.now();

        let klines: KLine[] = [];
        try {
          klines = await getKlines(
            market,
            '1m',
            Math.floor(since).toString(),
            Math.floor(until).toString()
          );
        } catch (e) {
          console.error('failed to fetch new bar', e);
          return;
        }

        if (klines.length === 0) return;

        // take the newest returned bar
        const newest = toBar(klines[klines.length - 1]);

        // prepare payload for your ChartManager.update()
        const payload = {
          close: newest.close,
          volume: newest.volume,
          // ChartManager.update() expects `time` in **seconds**
          time: Math.floor(newest.timestamp / 1000),
          newCandleInitiated: newest.timestamp > lastBarTsRef.current,
        };

        mgr.update(payload);
        lastBarTsRef.current = newest.timestamp;
      }, 1_000);
    };

    init().then(startUpdates);

    // cleanup on unmount or market change
    return () => {
      clearInterval(intervalId);
      managerRef.current?.destroy();
    };
  }, [market]);

  return (
    <>
      <ChartControl />
      <div className="flex-1 bg-card flex items-center justify-center">
        <div
          ref={containerRef}
          className="text-muted-foreground w-full h-full"
          style={{ minHeight: '300px' }}
        />
      </div>
    </>
  );
}
