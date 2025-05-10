import BottomTable from '@/src/components/trade/BottomTable';
import ChartArea from '@/src/components/trade/chart/ChartArea';
import OrderBook from '@/src/components/trade/Orderbook';
import SwapUI from '@/src/components/trade/SwapUI';
import TradeHeader from '@/src/components/trade/TradeHeader';

export async function generateStaticParams() {
  return [{ pair: 'SOLUSDC' }, { pair: 'BTCUSDC' }, { pair: 'ETHUSDC' }];
}

export default async function TradingPage({
  params,
}: {
  params: { pair?: string };
}) {
  const pair = params.pair || 'SOLUSDC';
  const baseCurrency = pair.length > 4 ? pair.slice(0, pair.length - 4) : 'SOL';
  const quoteCurrency = pair.length >= 4 ? pair.slice(-4) : 'USDC';

  return (
    <div className="min-h-screen bg-background text-foreground flex flex-col">
      {/* Trading Header */}
      <TradeHeader baseCurrency={baseCurrency} quoteCurrency={quoteCurrency} />

      <div className="flex-1 flex flex-col md:flex-row">
        {/* Left Section - Chart and Bottom Trading Table */}
        <div className="w-full md:w-3/4 border-r border-border/20 flex flex-col">
          <ChartArea />
          <BottomTable />
        </div>

        {/* Right Section - Trading Form and OrderBook */}
        <div className="w-full md:w-1/4 border-t md:border-t-0 border-border/20">
          <div className="p-4">
            <SwapUI baseCurrency={baseCurrency} quoteCurrency={quoteCurrency} />
            <OrderBook />
          </div>
        </div>
      </div>
    </div>
  );
}
