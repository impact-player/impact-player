'use client';

import React, { useState, useEffect } from 'react';

export type SparklineDataPoint = {
  price: number;
  timestamp: number; // Expecting milliseconds
};

export type SparklineData = {
  data: SparklineDataPoint[];
  type: 'up' | 'down' | 'volatile';
};

// K-line data structure from your API
type ApiKLine = {
  close: string;
  end: string; // Timestamp in milliseconds as a string
  high: string;
  low: string;
  open: string;
  quoteVolume: string;
  start: string; // Timestamp in milliseconds as a string
  trades: string;
  volume: string;
};

interface SparklineChartProps {
  marketId: string; // e.g., "KOHLI_USDC" or whatever format your API expects for the market
  height?: number;
  showDashedLine?: boolean;
  daysHistory?: number; // Optional: how many days of history to fetch
  interval?: string; // Optional: k-line interval e.g., "1h", "4h", "1m"
}

const SparklineChart: React.FC<SparklineChartProps> = ({
  marketId,
  height = 60,
  showDashedLine = true,
  daysHistory = 7, // Default to 7 days
  interval = '1h', // Default to 1-hour interval
}) => {
  const [chartData, setChartData] = useState<SparklineData | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!marketId) {
      setIsLoading(false);
      setError('Market ID is required.');
      setChartData(null); // Clear previous data if any
      return;
    }

    const fetchKlinesData = async () => {
      setIsLoading(true);
      setError(null);
      setChartData(null); // Clear previous data

      const endTime = Date.now();
      const startTime = endTime - daysHistory * 24 * 60 * 60 * 1000;

      try {
        // Construct the URL for fetching k-lines
        // Example: http://localhost:8080/api/v1/klines?market=SOL_USDC&interval=1h&startTime=xxxx&endTime=yyyy
        const apiUrl = `http://localhost:8080/api/v1/klines?market=${marketId}&interval=${interval}&startTime=${startTime}&endTime=${endTime}`;

        const response = await fetch(apiUrl);
        if (!response.ok) {
          const errorData = await response.text();
          throw new Error(
            `Failed to fetch k-lines: ${response.status} ${errorData}`
          );
        }

        const klines: ApiKLine[] = await response.json();

        if (!klines || klines.length === 0) {
          // No data returned, could be a new market or no trades in the period
          setChartData({ data: [], type: 'volatile' });
          setIsLoading(false);
          return;
        }

        // Sort by timestamp just in case they are not sorted, using 'start' or 'end'
        // API usually returns them sorted by 'start' time. Sparkline uses 'end' time usually.
        const sortedKlines = klines.sort(
          (a, b) => parseInt(a.start, 10) - parseInt(b.start, 10)
        );

        const processedData: SparklineDataPoint[] = sortedKlines.map(
          (kline) => ({
            price: parseFloat(kline.close),
            timestamp: parseInt(kline.end, 10), // Use end time for the point
          })
        );

        let type: 'up' | 'down' | 'volatile' = 'volatile';
        if (processedData.length > 1) {
          const firstPrice = processedData[0].price;
          const lastPrice = processedData[processedData.length - 1].price;
          if (lastPrice > firstPrice) {
            type = 'up';
          } else if (lastPrice < firstPrice) {
            type = 'down';
          }
        }

        setChartData({ data: processedData, type });
      } catch (e: any) {
        console.error(`Error fetching k-lines for ${marketId}:`, e);
        setError(e.message || 'Could not fetch chart data.');
        setChartData(null); // Ensure chart data is cleared on error
      } finally {
        setIsLoading(false);
      }
    };

    fetchKlinesData();
  }, [marketId, daysHistory, interval]); // Re-fetch if marketId, days, or interval changes

  if (isLoading) {
    // Placeholder for loading state, ensuring height consistency
    return (
      <div
        className="w-full h-full bg-transparent animate-pulse"
        style={{ height: `${height}px` }}
      />
    );
  }

  if (error) {
    // Placeholder for error state
    return (
      <div
        className="w-full h-full flex items-center justify-center text-red-500 text-xs"
        style={{ height: `${height}px` }}
        title={error}
      >
        Chart N/A
      </div>
    );
  }

  if (!chartData || chartData.data.length === 0) {
    // Placeholder for no data, ensuring height consistency
    return (
      <div
        className="w-full h-full flex items-center justify-center text-zinc-500 text-xs"
        style={{ height: `${height}px` }}
      >
        No chart data
      </div>
    );
  }

  const dataPoints = chartData.data;

  // Calculate min/max for scaling
  const prices = dataPoints.map((d) => d.price);
  const minPrice = Math.min(...prices);
  const maxPrice = Math.max(...prices);

  // Handle case where all prices are the same or only one point
  let priceRange = maxPrice - minPrice;
  if (priceRange === 0) {
    priceRange = minPrice > 0 ? minPrice * 0.2 : 1; // Avoid division by zero, create a small artificial range
  }

  // Add a small buffer to the top and bottom
  const buffer = priceRange * 0.1; // Adjust buffer if needed
  const adjustedMin = minPrice - buffer;
  const adjustedMax = maxPrice + buffer;
  const finalRange =
    adjustedMax - adjustedMin > 0 ? adjustedMax - adjustedMin : 1; // Ensure finalRange is not zero

  // Convert data points to SVG path coordinates
  const svgWidth = 240; // SVG viewBox width (keep it consistent or make it a prop)
  const generatePath = () => {
    if (dataPoints.length <= 1) {
      // Handle single point or no points for path
      if (dataPoints.length === 1) {
        const y =
          height - ((dataPoints[0].price - adjustedMin) / finalRange) * height;
        return `M 0 ${y} L ${svgWidth} ${y}`; // Draw a horizontal line for a single point
      }
      return 'M 0 0'; // No path
    }
    return dataPoints
      .map((point, i) => {
        const x = (i / (dataPoints.length - 1)) * svgWidth;
        // Ensure y is a finite number, especially if finalRange was 0 or adjustedMin/Max are problematic
        let yValue = ((point.price - adjustedMin) / finalRange) * height;
        if (!isFinite(yValue)) yValue = height / 2; // Default to middle if calculation fails
        const y = height - yValue;
        return `${i === 0 ? 'M' : 'L'} ${x.toFixed(2)} ${y.toFixed(2)}`;
      })
      .join(' ');
  };

  const generateFillPath = () => {
    const linePath = generatePath();
    if (dataPoints.length === 0) return 'M 0 0 Z';

    const firstPoint = dataPoints[0];
    const lastPoint = dataPoints[dataPoints.length - 1];

    let firstYValue = ((firstPoint.price - adjustedMin) / finalRange) * height;
    if (!isFinite(firstYValue)) firstYValue = height / 2;
    const firstY = height - firstYValue;

    return `${linePath} L ${svgWidth.toFixed(
      2
    )} ${height} L 0 ${height} L 0 ${firstY.toFixed(2)} Z`;
  };

  const strokeColor = chartData.type === 'down' ? '#FF3D00' : '#00C853'; // Red for down, Green for up/volatile

  // Generate dashed line at the starting price
  let startPriceY = height / 2; // Default if no data
  if (dataPoints.length > 0) {
    let startPriceYValue =
      ((dataPoints[0].price - adjustedMin) / finalRange) * height;
    if (!isFinite(startPriceYValue)) startPriceYValue = height / 2;
    startPriceY = height - startPriceYValue;
  }

  return (
    <svg
      viewBox={`0 0 ${svgWidth} ${height}`}
      className="w-full h-full"
      preserveAspectRatio="none"
    >
      <defs>
        <linearGradient
          id={`gradient-${chartData.type}-${marketId.replace(
            /[^a-zA-Z0-9]/g,
            ''
          )}`} // Sanitize marketId for ID
          x1="0%"
          y1="0%"
          x2="0%"
          y2="100%"
        >
          <stop offset="0%" stopColor={strokeColor} stopOpacity="0.15" />
          <stop offset="100%" stopColor={strokeColor} stopOpacity="0" />
        </linearGradient>
      </defs>

      <path
        d={generateFillPath()}
        fill={`url(#gradient-${chartData.type}-${marketId.replace(
          /[^a-zA-Z0-9]/g,
          ''
        )})`}
      />

      {showDashedLine && dataPoints.length > 0 && (
        <line
          x1="0"
          y1={startPriceY.toFixed(2)}
          x2={svgWidth}
          y2={startPriceY.toFixed(2)}
          stroke="#666666"
          strokeWidth="0.5"
          strokeDasharray="2,2"
          opacity="0.5"
        />
      )}

      <path
        d={generatePath()}
        fill="none"
        stroke={strokeColor}
        strokeWidth="1.5"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
};

export default SparklineChart;
