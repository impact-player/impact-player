import ClientTradingPage from './ClientTradingPage';

export default async function TradingPage({
  params,
}: {
  params: { pair?: string };
}) {
  const pair = await params.pair!;

  const baseCurrency = pair.length > 4 ? pair.slice(0, pair.length - 4) : 'SOL';
  const quoteCurrency = pair.length >= 4 ? pair.slice(-4) : 'USDC';

  return (
    <ClientTradingPage
      pair={pair}
      baseCurrency={baseCurrency}
      quoteCurrency={quoteCurrency}
    />
  );
}
