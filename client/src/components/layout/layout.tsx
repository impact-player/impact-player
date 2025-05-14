import type React from 'react';
import { Navbar } from './navbar';
import { Footer } from './footer';

interface MainLayoutProps {
  children: React.ReactNode;
  activePage?: 'home' | 'trade' | 'rewards' | 'learn' | 'news' | 'about';
}

export function MainLayout({ children, activePage = 'home' }: MainLayoutProps) {
  return (
    <div className="flex min-h-screen flex-col mx-auto max-w-7xl text-white">
      <Navbar activePage={activePage} />
      <main className="flex-1 w-full">{children}</main>
      <Footer />
    </div>
  );
}
