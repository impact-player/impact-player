// TradeHeader.tsx
import Link from 'next/link';
import { StarIcon } from '@/src/components/icons';
import Image from 'next/image';

interface TradeHeaderProps {
  baseCurrency: string;
  quoteCurrency: string;
}

export default function TradeHeader({
  baseCurrency,
  quoteCurrency,
}: TradeHeaderProps) {
  return (
    <div className="flex items-center justify-between px-4 py-3 border-b border-border/20">
      <div className="flex items-center">
        <Link href="/" passHref className="flex items-center mr-6">
          <Image src={'/logo.png'} height={54} width={54} alt={'logo'} />
        </Link>

        <div className="flex items-center gap-2">
          <h2 className="text-lg font-semibold">
            {baseCurrency}/{quoteCurrency}
          </h2>
          <button className="text-amber-500 hover:text-amber-600">
            <StarIcon className="h-5 w-5" />
          </button>
        </div>

        <div className="flex items-center ml-8">
          <div>
            <div className="text-xl font-bold">94.25</div>
            <div className="text-xs text-green-500">+2.15 (2.34%)</div>
          </div>
        </div>
      </div>

      <div className="flex gap-6 text-sm">
        <div>
          <div className="text-muted-foreground">24h High</div>
          <div className="font-medium">96.48</div>
        </div>
        <div>
          <div className="text-muted-foreground">24h Low</div>
          <div className="font-medium">92.07</div>
        </div>
        <div>
          <div className="text-muted-foreground">24h Volume</div>
          <div className="font-medium">{`4,256.42 ${baseCurrency}`}</div>
        </div>
        <div>
          <div className="text-muted-foreground">
            24h Volume ({quoteCurrency})
          </div>
          <div className="font-medium">401,172.58</div>
        </div>
      </div>
    </div>
  );
}
