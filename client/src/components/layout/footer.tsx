import Image from 'next/image';
import Link from 'next/link';

export function Footer() {
  return (
    <footer className="border-t border-border/10 bg-background pt-8 pb-6 w-full">
      <div className="container">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-8 gap-8 mb-8">
          <div className="lg:col-span-2">
            <Link
              href="/"
              className="inline-block mb-6"
              title="IP | The World's Fastest Cricket Prediction Market"
            >
              <Image src={'/logo.png'} height={54} width={54} alt={'logo'} />
            </Link>
          </div>

          <div className="lg:col-span-1">
            <h4 className="text-sm font-medium text-foreground mb-4">
              Company
            </h4>
            <FooterLink href="/company">Company</FooterLink>
          </div>

          <div className="lg:col-span-1">
            <h4 className="text-sm font-medium text-foreground mb-4">Legal</h4>
            <FooterLink href="/legal">Legal</FooterLink>
          </div>

          <div className="lg:col-span-1">
            <h4 className="text-sm font-medium text-foreground mb-4">FAQ</h4>
            <FooterLink href="/faqs">FAQ</FooterLink>
          </div>

          <div className="lg:col-span-1">
            <h4 className="text-sm font-medium text-foreground mb-4">Fees</h4>
            <FooterLink href="/fees">Fees</FooterLink>
          </div>

          <div className="lg:col-span-1">
            <h4 className="text-sm font-medium text-foreground mb-4">News</h4>
            <FooterLink href="/news">News</FooterLink>
            <FooterLink href="/press">Press</FooterLink>
          </div>

          <div className="lg:col-span-1">
            <h4 className="text-sm font-medium text-foreground mb-4">
              Careers
            </h4>
            <FooterLink href="https://www.linkedin.com/company/cubexch/jobs">
              Careers
            </FooterLink>
            <FooterLink href="/legal/privacy-policy">Privacy</FooterLink>
          </div>
        </div>

        <div className="flex flex-col md:flex-row justify-between items-center gap-4 pt-6 border-t border-border/10">
          <div className="flex gap-4">
            <Link
              href="https://www.twitter.com/cubexch"
              className="text-muted-foreground hover:text-foreground"
              aria-label="Twitter"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
                className="h-5 w-5"
              >
                <path d="M22 4s-.7 2.1-2 3.4c1.6 10-9.4 17.3-18 11.6 2.2.1 4.4-.6 6-2C3 15.5.5 9.6 3 5c2.2 2.6 5.6 4.1 9 4-.9-4.2 4-6.6 7-3.8 1.1 0 3-1.2 3-1.2z"></path>
              </svg>
            </Link>
          </div>
          <div className="text-sm text-muted-foreground">
            © 2025 IP Market Pvt Ltd, All rights reserved.
          </div>
        </div>
      </div>
    </footer>
  );
}

interface FooterLinkProps {
  href: string;
  children: React.ReactNode;
}

function FooterLink({ href, children }: FooterLinkProps) {
  return (
    <Link
      href={href}
      className="block text-sm text-muted-foreground hover:text-foreground transition-colors mb-2"
    >
      {children}
    </Link>
  );
}
