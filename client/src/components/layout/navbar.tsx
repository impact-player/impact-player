import Link from 'next/link';
import { GlobeIcon } from '@/src/components/icons';
import { Button } from '@/src/components/ui/button';
import Image from 'next/image';

interface NavbarProps {
  activePage?: 'home' | 'trade' | 'rewards' | 'learn' | 'news' | 'about';
}

export function Navbar({ activePage = 'home' }: NavbarProps) {
  return (
    <header className="h-[4.5rem] w-full flex items-center border-b border-border/10 bg-background">
      <nav
        className="container flex items-center justify-between"
        aria-label="Global"
      >
        <div className="flex items-center gap-8">
          <Link href="/" passHref className="flex items-center">
            <Image src={'/logo.png'} height={54} width={54} alt={'logo'} />
          </Link>

          <div className="hidden md:flex items-center gap-5">
            <NavLink href="/trade/SOLUSDC" active={activePage === 'trade'}>
              Trade
            </NavLink>
            <NavLink href="/rewards" active={activePage === 'rewards'}>
              Rewards
            </NavLink>
            <NavLink href="/learn" active={activePage === 'learn'}>
              Learn
            </NavLink>
            <NavLink href="/news" active={activePage === 'news'}>
              News
            </NavLink>
            <NavLink href="/about" active={activePage === 'about'}>
              About
            </NavLink>
          </div>
        </div>

        <div className="flex items-center gap-4">
          <div className="flex items-center gap-2">
            <span className="text-sm uppercase text-foreground dark:text-foreground">
              EN
            </span>
            <Button
              variant="outline"
              size="icon"
              className="rounded-full bg-secondary border-none"
              aria-label="Language"
            >
              <GlobeIcon className="h-5 w-5" />
            </Button>
          </div>

          <div className="hidden sm:flex items-center gap-2">
            <Link href="/signin" passHref>
              <Button
                variant="outline"
                className="rounded-xl border-white text-white bg-transparent hover:bg-white/10"
              >
                Sign In
              </Button>
            </Link>
            <Link href="/signup" passHref>
              <Button className="rounded-xl bg-cube-orange hover:bg-cube-orange/90 text-white">
                Sign Up
              </Button>
            </Link>
          </div>
        </div>
      </nav>
    </header>
  );
}

interface NavLinkProps {
  href: string;
  active?: boolean;
  children: React.ReactNode;
}

function NavLink({ href, active, children }: NavLinkProps) {
  return (
    <Link
      href={href}
      className={`text-sm font-medium transition-colors hover:text-white ${
        active ? 'text-white' : 'text-muted-foreground'
      }`}
    >
      {children}
    </Link>
  );
}
