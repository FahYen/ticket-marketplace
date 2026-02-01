'use client';

import Link from 'next/link';
import { useAuth } from '@/lib/auth';
import { Button } from '@/components/ui/Button';
import { useRouter } from 'next/navigation';

interface NavbarProps {
  transparent?: boolean;
}

export function Navbar({ transparent = false }: NavbarProps) {
  const { user, logout, isAuthenticated } = useAuth();
  const router = useRouter();

  const handleLogout = () => {
    logout();
    router.push('/');
  };

  return (
    <nav
      className={`w-full py-4 px-6 ${
        transparent
          ? 'absolute top-0 left-0 z-50 bg-transparent'
          : 'bg-spartan-white border-b border-gray-200'
      }`}
    >
      <div className="max-w-7xl mx-auto flex items-center justify-between">
        <Link
          href="/"
          className={`text-2xl font-bold ${
            transparent ? 'text-white' : 'text-spartan-green'
          }`}
        >
          MSU Tickets
        </Link>

        <div className="flex items-center gap-6">
          <Link
            href="/tickets"
            className={`font-medium hover:underline ${
              transparent ? 'text-white' : 'text-spartan-black'
            }`}
          >
            Browse
          </Link>
          <Link
            href="/games"
            className={`font-medium hover:underline ${
              transparent ? 'text-white' : 'text-spartan-black'
            }`}
          >
            Games
          </Link>

          {isAuthenticated ? (
            <>
              <Link
                href="/sell"
                className={`font-medium hover:underline ${
                  transparent ? 'text-white' : 'text-spartan-black'
                }`}
              >
                Sell
              </Link>
              <Link
                href="/my-listings"
                className={`font-medium hover:underline ${
                  transparent ? 'text-white' : 'text-spartan-black'
                }`}
              >
                My Listings
              </Link>
              <span
                className={`text-sm ${
                  transparent ? 'text-white/80' : 'text-gray-600'
                }`}
              >
                {user?.email}
              </span>
              <Button variant="outline" size="sm" onClick={handleLogout}>
                Logout
              </Button>
            </>
          ) : (
            <>
              <Link href="/login">
                <Button
                  variant={transparent ? 'outline' : 'secondary'}
                  size="sm"
                  className={transparent ? 'border-white text-white hover:bg-white hover:text-spartan-black' : ''}
                >
                  Login
                </Button>
              </Link>
              <Link href="/register">
                <Button variant="primary" size="sm">
                  Register
                </Button>
              </Link>
            </>
          )}
        </div>
      </div>
    </nav>
  );
}
