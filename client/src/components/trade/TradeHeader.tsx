'use client';

import Link from 'next/link';
import Image from 'next/image';
import { useEffect, useState } from 'react';
import { getTicker } from '@/src/utils/httpClient';
import { MarketIcon } from '../market/MarketIcon';
import { Button } from '../ui/button';
import { signOut, useSession } from 'next-auth/react';
import { SignalingManager } from '@/src/utils/SignalingManager';
import { TickerPayload } from '@/src/utils/types';

interface TradeHeaderProps {
  baseCurrency: string;
  quoteCurrency: string;
}

export default function TradeHeader({
  baseCurrency,
  quoteCurrency,
}: TradeHeaderProps) {
  const [name, setName] = useState<string>();
  const [price, setPrice] = useState<string>('');
  const [quantity, setQuantity] = useState<string>('');
  const [priceChange, setPriceChange] = useState<number>(0);
  const [prevPrice, setPrevPrice] = useState<number | null>(null);
  const session = useSession();

  useEffect(() => {
    const getTickerData = async () => {
      const data = await getTicker(`${baseCurrency}`);
      console.log('trade header data: ', data);
      setName(data.name);
    };

    getTickerData();
  }, [baseCurrency, quoteCurrency]);
  const market = `${baseCurrency}${quoteCurrency}`;
  const room = `ticker@${baseCurrency}${quoteCurrency}` as const;

  useEffect(() => {
    const onTicker = (data: TickerPayload) => {
      const { p, q, t } = data.data;

      if (p) {
        const currentPrice = parseFloat(p);
        setPrice(p);
        setQuantity(q || '');

        if (prevPrice !== null) {
          const change = ((currentPrice - prevPrice) / prevPrice) * 100;
          setPriceChange(parseFloat(change.toFixed(2)));
        }

        setPrevPrice(currentPrice);
      }
    };

    const mgr = SignalingManager.getInstance();

    mgr.registerTickerCallback(room, onTicker);
    mgr.subscribe(room);

    return () => {
      mgr.deRegisterTickerCallback(room, onTicker);
      mgr.unsubscribe(room);
    };
  }, [market, room, prevPrice]);

  return (
    <div className="flex items-center justify-between px-4 py-3 border-b border-border/20">
      <div className="flex items-center">
        <Link href="/" passHref className="flex items-center mr-6">
          <Image src={'/logo.png'} height={54} width={54} alt={'logo'} />
        </Link>

        <div className="flex items-center gap-2">
          <MarketIcon base_asset={baseCurrency} />
          <h2 className="text-lg font-semibold">
            {baseCurrency}
            {quoteCurrency}
          </h2>
        </div>

        <div className="flex items-center ml-8">
          <div>
            <div className="text-xl font-bold">
              {(parseFloat(price) * 100).toFixed(2)}%
            </div>

            <div className="text-xs text-green-500">${priceChange}</div>
          </div>
        </div>
      </div>
      <div>
        <div className="font-semibold text-lg">{name}</div>
      </div>
      {session.data?.user?.email && (
        <div className="flex items-center space-x-6">
          <div className="flex gap-6 text-sm">
            <div>
              <div className="text-muted-foreground">Balance(USDC)</div>
              <div className="font-medium">$10_000</div>
            </div>
            <Button
              variant="outline"
              className="rounded-xl border-white text-white bg-transparent hover:bg-white/10"
              onClick={async () => signOut()}
            >
              Log out
            </Button>
          </div>
        </div>
      )}
    </div>
  );
}
