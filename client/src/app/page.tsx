import FeatureSections from '../components/home/FeatureSections';
import HeroCarousel from '../components/home/HeroCarousel';
import PriceTable from '../components/home/PriceTable';
import { MainLayout } from '../components/layout/layout';

export default function Home() {
  return (
    <MainLayout>
      <div>
        <HeroCarousel />
        <PriceTable />
        <FeatureSections />
      </div>
    </MainLayout>
  );
}
