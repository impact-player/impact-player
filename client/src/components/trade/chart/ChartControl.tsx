'use client';

import { Button } from '@/src/components/ui/button';
import { Tabs, TabsList, TabsTrigger } from '@/src/components/ui/tabs';
import { ChartIcon, BookIcon, SettingsIcon } from '@/src/components/icons';
import { useChartStore } from '@/src/utils/store/chartStore';

export default function ChartControl() {
  const { interval, chartType, setInterval, setChartType } = useChartStore();

  return (
    <div className="border-b border-border/20 flex items-center p-2">
      <div className="flex items-center gap-4">
        <Tabs
          defaultValue={chartType}
          value={chartType}
          onValueChange={(value) =>
            setChartType(value as 'chart' | 'book' | 'depth' | 'equalizer')
          }
        >
          <TabsList className="bg-background border border-border/20 rounded-md">
            <TabsTrigger
              value="chart"
              className="data-[state=active]:bg-secondary rounded-md"
            >
              <ChartIcon className="h-4 w-4 mr-1" />
              Chart
            </TabsTrigger>
            <TabsTrigger
              value="book"
              className="data-[state=active]:bg-secondary rounded-md"
            >
              <BookIcon className="h-4 w-4 mr-1" />
              Book
            </TabsTrigger>
            <TabsTrigger
              value="depth"
              className="data-[state=active]:bg-secondary rounded-md"
            >
              Depth
            </TabsTrigger>
            <TabsTrigger
              value="equalizer"
              className="data-[state=active]:bg-secondary rounded-md"
            >
              Equalizer
            </TabsTrigger>
          </TabsList>
        </Tabs>

        <div className="flex items-center gap-2 text-sm">
          <Button
            variant={interval === '1m' ? 'secondary' : 'ghost'}
            size="sm"
            className="px-2 py-1 h-8 rounded-md"
            onClick={() => setInterval('1m')}
          >
            1m
          </Button>
          <Button
            variant={interval === '1h' ? 'secondary' : 'ghost'}
            size="sm"
            className="px-2 py-1 h-8 rounded-md"
            onClick={() => setInterval('1h')}
          >
            1h
          </Button>
          <Button
            variant={interval === '1d' ? 'secondary' : 'ghost'}
            size="sm"
            className="px-2 py-1 h-8 rounded-md"
            onClick={() => setInterval('1d')}
          >
            1d
          </Button>
          <Button
            variant={interval === '1w' ? 'secondary' : 'ghost'}
            size="sm"
            className="px-2 py-1 h-8 rounded-md"
            onClick={() => setInterval('1w')}
          >
            1w
          </Button>
        </div>
      </div>

      <div className="ml-auto flex items-center gap-2">
        <Button variant="ghost" size="icon" className="h-8 w-8 rounded-md">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            className="h-4 w-4"
          >
            <path d="M3 3v18h18" />
            <path d="m19 9-5-5-4 8-3-4-4 1" />
          </svg>
        </Button>
        <Button variant="ghost" size="icon" className="h-8 w-8 rounded-md">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            className="h-4 w-4"
          >
            <path d="M2 12h20M12 2v20" />
          </svg>
        </Button>
        <Button variant="ghost" size="icon" className="h-8 w-8 rounded-md">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            className="h-4 w-4"
          >
            <circle cx="12" cy="12" r="10" />
            <line x1="12" y1="8" x2="12" y2="16" />
            <line x1="8" y1="12" x2="16" y2="12" />
          </svg>
        </Button>
        <Button variant="ghost" size="icon" className="h-8 w-8 rounded-md">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            className="h-4 w-4"
          >
            <circle cx="12" cy="12" r="10" />
            <line x1="8" y1="12" x2="16" y2="12" />
          </svg>
        </Button>
        <Button variant="ghost" size="icon" className="h-8 w-8 rounded-md">
          <SettingsIcon className="h-4 w-4" />
        </Button>
      </div>
    </div>
  );
}
