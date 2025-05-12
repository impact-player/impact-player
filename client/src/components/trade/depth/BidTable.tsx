export const BidTable = ({ bids }: { bids: [string, string][] }) => {
  // Assuming bids are already sorted (highest price first for bids)
  const relevantBids = bids.slice(0, 30);
  let currentTotal = 0;

  let bidsWithTotal: [string, string, number][] = [];

  // For bids, we process in the existing order if already sorted
  for (let i = 0; i < relevantBids.length; i++) {
    const [price, quantity] = relevantBids[i];
    currentTotal += Number(quantity);
    bidsWithTotal.push([price, quantity, currentTotal]);
  }

  const maxTotal = currentTotal; // Use the final accumulated total

  return (
    <div>
      {bidsWithTotal.map(([price, quantity, total]) => (
        <Bid
          maxTotal={maxTotal}
          total={total}
          key={price}
          price={price}
          quantity={quantity}
        />
      ))}
    </div>
  );
};

function Bid({
  price,
  quantity,
  total,
  maxTotal,
}: {
  price: string;
  quantity: string;
  total: number;
  maxTotal: number;
}) {
  return (
    <div
      style={{
        display: 'flex',
        position: 'relative',
        width: '100%',
        backgroundColor: 'transparent',
        overflow: 'hidden',
        marginTop: '2px',
        paddingTop: '2px',
        paddingBottom: '2px',
      }}
    >
      <div
        style={{
          position: 'absolute',
          top: 0,
          left: 0,
          width: `${(100 * total) / maxTotal}%`,
          height: '100%',
          background: 'rgba(1, 167, 129, 0.325)',
          transition: 'width 0.3s ease-in-out',
        }}
      ></div>
      <div className={`flex justify-between text-sm w-full px-2`}>
        <div>{price}</div>
        <div>{quantity}</div>
      </div>
    </div>
  );
}
