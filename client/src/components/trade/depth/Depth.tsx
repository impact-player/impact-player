// components/Depth.tsx
'use client';
import { useEffect, useState } from 'react';
import { getDepth } from '@/src/utils/httpClient';
import { AskTable } from './AskTable';
import { BidTable } from './BidTable';
import { SignalingManager } from '@/src/utils/SignalingManager';

export default function Depth({ market }: { market: string }) {
  const [bids, setBids] = useState<[string, string][]>();
  const [asks, setAsks] = useState<[string, string][]>();
  const room = `depth@${market}`;

  useEffect(() => {
    const onDepth = (data: {
      bids: [string, string][];
      asks: [string, string][];
    }) => {
      console.log('real-time depth update:', data);
      if (data.bids && data.bids.length > 0) {
        setBids(data.bids);
      }
      if (data.asks && data.asks.length > 0) {
        setAsks(data.asks);
      }
    };

    const mgr = SignalingManager.getInstance();
    mgr.registerCallback(room, onDepth);
    mgr.sendMessage({ type: 'SUBSCRIBE', payload: { room } });

    getDepth(market).then((d) => {
      setBids(d.payload.bids);
      setAsks(d.payload.asks);
    });

    return () => {
      mgr.sendMessage({ type: 'UNSUBSCRIBE', payload: { room } });
      mgr.deRegisterCallback(room, onDepth);
    };
  }, [market]);

  return (
    <div className="flex flex-col h-full border-t border-border/20 pt-2">
      <div className="sticky top-0 bg-background z-20 py-1 px-2">
        <TableHeader />
      </div>
      <div className="flex flex-grow overflow-hidden">
        <div className="w-1/2 pr-1 flex flex-col">
          {asks && (
            <div className="flex-grow overflow-y-auto">
              <AskTable asks={asks.slice(0, 50)} />
            </div>
          )}
        </div>
        <div className="w-1/2 pl-1 flex flex-col">
          {bids && (
            <div className="flex-grow overflow-y-auto">
              <BidTable bids={bids.slice(0, 50)} />
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

function TableHeader() {
  return (
    <>
      <div className="flex justify-between items-center mb-2">
        <div className="flex items-center gap-4 text-xs">
          <div className="flex items-center">
            <div className="w-3 h-3 bg-red-800 mr-1" /> Asks
          </div>
          <div className="flex items-center">
            <div className="w-3 h-3 bg-green-800 mr-1" /> Bids
          </div>
        </div>
      </div>
      <div className="flex text-xs text-muted-foreground mb-2">
        <div className="flex-grow text-left">Size</div>
        <div className="flex-grow text-center">Ask</div>
        <div className="flex-grow text-center">Bid</div>
        <div className="flex-grow text-right">Size</div>
      </div>
    </>
  );
}
