import ChartControl from './ChartControl';

export default function ChartArea() {
  return (
    <>
      <ChartControl />

      <div className="flex-1 bg-card p-4 flex items-center justify-center">
        <div className="text-muted-foreground">
          Trading chart will appear here
        </div>
      </div>
    </>
  );
}
