// src/store/chartStore.ts
import { create } from 'zustand';

type Interval = '1m' | '1h' | '1d' | '1w';
type ChartType = 'chart' | 'book' | 'depth' | 'equalizer';

interface ChartState {
  market: string;
  interval: Interval;
  chartType: ChartType;
  setInterval: (interval: Interval) => void;
  setMarket: (market: string) => void;
  setChartType: (type: ChartType) => void;
}

export const useChartStore = create<ChartState>((set) => ({
  market: 'SOL_USDC',
  interval: '1h',
  chartType: 'chart',
  setInterval: (interval) => set({ interval }),
  setMarket: (market) => set({ market }),
  setChartType: (chartType) => set({ chartType }),
}));
