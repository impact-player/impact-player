'use client';

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

type PromoItem = {
  id: string;
  title: string;
  subtitle?: string;
  timeRemaining?: string;
  dateRange?: { from: string; to: string };
  linkUrl: string;
  bgColor: string;
  textColor: string;
  buttonText?: string;
};

const promos: PromoItem[] = [
  {
    id: '1',
    title: '$45,000 Rewards',
    subtitle: 'Trade SOL, BTC, SUI',
    timeRemaining: '24 HOURS ONLY - MAY 13',
    linkUrl: 'https://www.cube.exchange/trade/SOLUSDC?promo_banner',
    bgColor: 'from-[#121212] to-[#291500]',
    textColor: 'text-white',
    buttonText: 'Trade now',
  },
  {
    id: '2',
    title: 'WIN A MACBOOK PRO',
    timeRemaining: '24 HOURS ONLY - MAY 13',
    linkUrl:
      'https://www.cube.exchange/news/physical-giveaways-monsoon?promo_banner',
    bgColor: 'from-[#121212] to-[#1A0F00]',
    textColor: 'text-white',
    buttonText: 'See More',
  },
  {
    id: '3',
    title: '$20K SOL & BTC Rewards',
    dateRange: { from: 'May 8', to: 'May 15' },
    linkUrl:
      'https://www.cube.exchange/news/20k-sol-btc-trading-competition-may-15?promo_banner',
    bgColor: 'from-[#121212] to-[#001A2C]',
    textColor: 'text-white',
    buttonText: 'Trade SOL & BTC',
  },
];

const HeroCarousel = () => {
  const [activeIndex, setActiveIndex] = useState(0);

  // Auto-advance carousel
  useEffect(() => {
    const interval = setInterval(() => {
      setActiveIndex((current) => (current + 1) % promos.length);
    }, 5000);
    return () => clearInterval(interval);
  }, []);

  const handleDotClick = (index: number) => {
    setActiveIndex(index);
  };

  return (
    <div className="w-full max-w-5xl mx-auto px-4 pt-8 pb-4">
      <div className="relative rounded-xl overflow-hidden">
        {/* Carousel items */}
        <div className="relative h-[180px]">
          {promos.map((promo, index) => (
            <div
              key={promo.id}
              className={`absolute inset-0 transition-opacity duration-500 ${
                index === activeIndex ? 'opacity-100 z-10' : 'opacity-0 z-0'
              }`}
            >
              <Link href={promo.linkUrl} className="block h-full">
                <div
                  className={`h-full w-full bg-gradient-to-r ${promo.bgColor} p-8 flex flex-col justify-center`}
                >
                  {promo.timeRemaining && (
                    <div className="mb-2 text-xs bg-black/30 backdrop-blur-sm text-white px-3 py-1 rounded-full w-fit">
                      {promo.timeRemaining}
                    </div>
                  )}

                  {promo.dateRange && (
                    <div className="mb-2 text-xs bg-black/30 backdrop-blur-sm text-white px-3 py-1 rounded-full w-fit">
                      {promo.dateRange.from} - {promo.dateRange.to}
                    </div>
                  )}

                  <h2
                    className={`text-2xl md:text-3xl font-light mb-1 ${promo.textColor}`}
                  >
                    {promo.title}
                  </h2>
                  {promo.subtitle && (
                    <p
                      className={`text-lg md:text-xl ${promo.textColor} opacity-80`}
                    >
                      {promo.subtitle}
                    </p>
                  )}

                  {promo.buttonText && (
                    <div className="mt-4">
                      <button className="px-4 py-1.5 bg-[#FF6B00] text-white text-xs rounded-full hover:bg-[#FF6B00]/90 transition">
                        {promo.buttonText}
                      </button>
                    </div>
                  )}
                </div>
              </Link>
            </div>
          ))}
        </div>

        {/* Carousel indicators */}
        <div className="absolute bottom-4 left-0 right-0 flex justify-center space-x-2 z-20">
          {promos.map((_, index) => (
            <button
              key={index}
              onClick={() => handleDotClick(index)}
              className={`w-2 h-2 rounded-full transition-colors ${
                index === activeIndex ? 'bg-white' : 'bg-white/30'
              }`}
              aria-label={`Go to slide ${index + 1}`}
            />
          ))}
        </div>
      </div>
    </div>
  );
};

export default HeroCarousel;
