'use client';

import Link from 'next/link';
import { StarIcon } from '@/src/components/icons';
import Image from 'next/image';
import { useEffect, useState } from 'react';
import { getTicker } from '@/src/utils/httpClient';
import { MarketIcon } from '../market/MarketIcon';
import { Button } from '../ui/button';

interface TradeHeaderProps {
  baseCurrency: string;
  quoteCurrency: string;
}

export default function TradeHeader({
  baseCurrency,
  quoteCurrency,
}: TradeHeaderProps) {
  const [name, setName] = useState<string>();

  useEffect(() => {
    const getTickerData = async () => {
      const data = await getTicker(`${baseCurrency}`);
      console.log('trade header data: ', data);
      setName(data.name);

      // Helper function to convert day of year and time values to date-month-year HH:MM AM/PM format
      const formatDateTime = (timeArray: number[]) => {
        if (!timeArray || timeArray.length < 6) return 'Invalid date';

        const [year, dayOfYear, hour, minute] = timeArray;

        // Create date from day of year
        const date = new Date(year, 0);
        date.setDate(dayOfYear);

        const day = date.getDate();
        const month = date.getMonth() + 1; // getMonth() is zero-indexed

        // Convert to 12-hour format with AM/PM
        let hours12 = hour % 12;
        hours12 = hours12 === 0 ? 12 : hours12; // Convert 0 to 12 for 12 AM
        const ampm = hour >= 12 ? 'PM' : 'AM';

        // Format time with leading zeros where needed
        const formattedHour = hours12 < 10 ? `0${hours12}` : hours12;
        const formattedMinute = minute < 10 ? `0${minute}` : minute;

        return `${day < 10 ? '0' + day : day}-${
          month < 10 ? '0' + month : month
        }-${year} ${formattedHour}:${formattedMinute} ${ampm}`;
      };
    };

    getTickerData();
  }, [baseCurrency, quoteCurrency]);

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
            <div className="text-xl font-bold">94.25</div>
            <div className="text-xs text-green-500">+2.15 (2.34%)</div>
          </div>
        </div>
      </div>
      <div>
        <div className="font-medium">{name}</div>
      </div>

      <div className="flex gap-6 text-sm">
        <div>
          <div className="text-muted-foreground">Balance(USDC)</div>
          <div className="font-medium">$10_000</div>
        </div>
        <Button
          variant="outline"
          className="rounded-xl border-white text-white bg-transparent hover:bg-white/10"
        >
          Deposit
        </Button>
      </div>
    </div>
  );
}
