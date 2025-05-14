'use client';

import { useEffect, useRef } from 'react';
import ChartControl from './ChartControl';
import { ChartManager } from '@/src/utils/chartManager';
import { KLine } from '@/src/utils/types';
import { getKlines } from '@/src/utils/httpClient';
import { UTCTimestamp } from 'lightweight-charts';
import { useChartStore } from '@/src/utils/store/chartStore';

export default function ChartArea({ market }: { market: string }) {
  const containerRef = useRef<HTMLDivElement>(null);
  const managerRef = useRef<ChartManager | null>(null);
  const lastBarTsRef = useRef<number>(0);
  const intervalIdRef = useRef<NodeJS.Timeout | null>(null);

  const { interval } = useChartStore();

  const toBar = (x: KLine) => {
    const tsInMillis =
      typeof x.end === 'string' ? parseInt(x.end, 10) : Number(x.end);
    return {
      timestamp: tsInMillis,
      close: parseFloat(x.close),
      volume: parseFloat(x.volume),
      open: x.open ? parseFloat(x.open) : undefined,
      high: x.high ? parseFloat(x.high) : undefined,
      low: x.low ? parseFloat(x.low) : undefined,
    };
  };

  const initializeChart = async () => {
    if (!containerRef.current) return;

    const now = Date.now();
    let startTime;
    switch (interval) {
      case '1m':
        startTime = now - 1000 * 60 * 60;
        break;
      case '1h':
        startTime = now - 1000 * 60 * 60 * 24 * 7;
        break;
      case '1d':
        startTime = now - 1000 * 60 * 60 * 24 * 30 * 6;
        break;
      case '1w':
        startTime = now - 1000 * 60 * 60 * 24 * 365 * 2;
        break;
      default:
        startTime = now - 1000 * 60 * 60;
    }

    let klines: KLine[] = [];
    try {
      klines = await getKlines(
        market,
        interval,
        startTime.toString(),
        now.toString()
      );
    } catch (e) {
      console.error('Failed to fetch initial history:', e);
      return;
    }

    const bars = klines.map(toBar).sort((a, b) => a.timestamp - b.timestamp);

    managerRef.current?.destroy();

    managerRef.current = new ChartManager(containerRef.current, bars, {
      background: '#0e0f14',
      color: 'white',
    });

    lastBarTsRef.current =
      bars.length > 0 ? bars[bars.length - 1].timestamp : now;
  };

  const fetchLatestDataAndUpdate = async () => {
    console.log(
      `[${new Date().toLocaleTimeString()}] fetchLatestDataAndUpdate called`
    ); // 1. Is it running?

    if (!managerRef.current) {
      console.log('Chart manager not initialized, skipping fetch.');
      return;
    }

    const start = (lastBarTsRef.current + 1).toString();
    const now = Date.now().toString();

    let klines: KLine[] = [];
    try {
      klines = await getKlines(market, interval, start, now);
      console.log(
        `[${new Date().toLocaleTimeString()}] Klines fetched:`,
        klines
      ); // 2. What data did we get?
    } catch (e) {
      console.error(
        `[${new Date().toLocaleTimeString()}] Failed to fetch latest data:`,
        e
      );
      return;
    }

    if (klines.length > 0) {
      const newBars = klines
        .map(toBar)
        .sort((a, b) => a.timestamp - b.timestamp);

      const latestBar = newBars[newBars.length - 1];

      const barTimeForChart = Math.floor(
        latestBar.timestamp / 1000
      ) as UTCTimestamp;

      const updateData: any = {
        time: barTimeForChart,
        open: latestBar.open,
        high: latestBar.high,
        low: latestBar.low,
        close: latestBar.close,
        volume: latestBar.volume,
      };

      console.log(
        `[${new Date().toLocaleTimeString()}] Chart updateData:`,
        updateData
      ); // 3. What are we sending to ChartManager?

      const isNewCandlePeriod = latestBar.timestamp > lastBarTsRef.current;

      // This newCandleInitiated flag is optional, depends on ChartManager's needs
      // if (isNewCandlePeriod) {
      //   updateData.newCandleInitiated = true;
      // }

      managerRef.current?.update(updateData);
      console.log(
        `[${new Date().toLocaleTimeString()}] Called managerRef.current.update()`
      ); // 4. Did we call update?

      if (isNewCandlePeriod) {
        lastBarTsRef.current = latestBar.timestamp;
      }
    } else {
      console.log(
        `[${new Date().toLocaleTimeString()}] No new klines received.`
      ); // 5. Log if no data
    }
  };

  useEffect(() => {
    console.log(
      `[EFFECT] Initializing chart and setting up for ${market} @ ${interval}`
    );
    initializeChart();
    return () => {
      console.log(
        `[EFFECT CLEANUP] Destroying chart for ${market} @ ${interval}`
      );
      managerRef.current?.destroy();
      managerRef.current = null;
      if (intervalIdRef.current) {
        clearInterval(intervalIdRef.current);
        intervalIdRef.current = null;
      }
    };
  }, [market, interval]);

  useEffect(() => {
    if (intervalIdRef.current) {
      clearInterval(intervalIdRef.current);
    }
    console.log(
      `[EFFECT INTERVAL] Setting up polling interval for ${market} @ ${interval}`
    );
    intervalIdRef.current = setInterval(fetchLatestDataAndUpdate, 1000);
    return () => {
      if (intervalIdRef.current) {
        console.log(
          `[EFFECT INTERVAL CLEANUP] Clearing polling interval for ${market} @ ${interval}`
        );
        clearInterval(intervalIdRef.current);
        intervalIdRef.current = null;
      }
    };
  }, [market, interval]);

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
