import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '@/src/components/ui/tabs';

export default function BottomTable() {
  return (
    <div className="h-48 border-t border-border/20">
      <Tabs defaultValue="trade">
        <div className="border-b border-border/20">
          <TabsList className="bg-background border-b border-border/20 rounded-none">
            <TabsTrigger
              value="trade"
              className="data-[state=active]:bg-background data-[state=active]:border-b-2 data-[state=active]:border-foreground rounded-none"
            >
              Trade
            </TabsTrigger>
            <TabsTrigger
              value="advanced"
              className="data-[state=active]:bg-background data-[state=active]:border-b-2 data-[state=active]:border-foreground rounded-none"
            >
              Advanced
            </TabsTrigger>
            <TabsTrigger
              value="recent"
              className="data-[state=active]:bg-background data-[state=active]:border-b-2 data-[state=active]:border-foreground rounded-none"
            >
              Recent
            </TabsTrigger>
            <TabsTrigger
              value="algos"
              className="data-[state=active]:bg-background data-[state=active]:border-b-2 data-[state=active]:border-foreground rounded-none"
            >
              Algos
            </TabsTrigger>
          </TabsList>
        </div>

        <TabsContent value="trade" className="p-4">
          <div className="flex flex-col">
            <div className="text-sm text-muted-foreground">
              Trade history will appear here
            </div>
          </div>
        </TabsContent>

        <TabsContent value="advanced" className="p-4">
          <div className="flex flex-col">
            <div className="text-sm text-muted-foreground">
              Advanced trading options will appear here
            </div>
          </div>
        </TabsContent>

        <TabsContent value="recent" className="p-4">
          <div className="flex flex-col">
            <div className="text-sm text-muted-foreground">
              Recent trades will appear here
            </div>
          </div>
        </TabsContent>

        <TabsContent value="algos" className="p-4">
          <div className="flex flex-col">
            <div className="text-sm text-muted-foreground">
              Trading algorithms will appear here
            </div>
          </div>
        </TabsContent>
      </Tabs>
    </div>
  );
}
