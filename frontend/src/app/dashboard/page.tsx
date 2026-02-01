'use client';

import { useState, useEffect } from 'react';
import Link from 'next/link';
import { Navbar } from '@/components/layout/Navbar';
import { Footer } from '@/components/layout/Footer';
import { Card, CardContent, CardHeader } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { AuthGuard } from '@/components/layout/AuthGuard';
import { useAuth } from '@/lib/auth';
import { api } from '@/lib/api';
import { Ticket } from '@/types';

export default function DashboardPage() {
  const { user } = useAuth();
  const [stats, setStats] = useState({
    total: 0,
    unverified: 0,
    verified: 0,
    sold: 0,
  });
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    async function fetchStats() {
      try {
        const { tickets } = await api.getMyListings();
        setStats({
          total: tickets.length,
          unverified: tickets.filter((t: Ticket) => t.status === 'Unverified').length,
          verified: tickets.filter((t: Ticket) => t.status === 'Verified').length,
          sold: tickets.filter((t: Ticket) => t.status === 'Sold').length,
        });
      } catch {
        // Ignore errors for stats
      } finally {
        setIsLoading(false);
      }
    }
    fetchStats();
  }, []);

  return (
    <AuthGuard>
      <div className="min-h-screen flex flex-col">
        <Navbar />
        <main className="flex-1 px-6 py-12">
          <div className="max-w-4xl mx-auto">
            <h1 className="text-3xl font-bold text-spartan-black mb-2">
              Welcome back!
            </h1>
            <p className="text-gray-600 mb-8">{user?.email}</p>

            {/* Quick Actions */}
            <div className="flex gap-4 mb-8">
              <Link href="/sell">
                <Button size="lg">List a Ticket</Button>
              </Link>
              <Link href="/tickets">
                <Button size="lg" variant="outline">
                  Browse Tickets
                </Button>
              </Link>
            </div>

            {/* Stats */}
            <div className="grid md:grid-cols-4 gap-4 mb-8">
              <Card>
                <CardContent className="p-6 text-center">
                  <p className="text-3xl font-bold text-spartan-black">
                    {isLoading ? '-' : stats.total}
                  </p>
                  <p className="text-gray-600">Total Listings</p>
                </CardContent>
              </Card>
              <Card>
                <CardContent className="p-6 text-center">
                  <p className="text-3xl font-bold text-yellow-600">
                    {isLoading ? '-' : stats.unverified}
                  </p>
                  <p className="text-gray-600">Unverified</p>
                </CardContent>
              </Card>
              <Card>
                <CardContent className="p-6 text-center">
                  <p className="text-3xl font-bold text-green-600">
                    {isLoading ? '-' : stats.verified}
                  </p>
                  <p className="text-gray-600">Verified</p>
                </CardContent>
              </Card>
              <Card>
                <CardContent className="p-6 text-center">
                  <p className="text-3xl font-bold text-spartan-green">
                    {isLoading ? '-' : stats.sold}
                  </p>
                  <p className="text-gray-600">Sold</p>
                </CardContent>
              </Card>
            </div>

            {/* Quick Links */}
            <Card>
              <CardHeader>
                <h2 className="text-xl font-semibold">Quick Links</h2>
              </CardHeader>
              <CardContent>
                <div className="grid md:grid-cols-2 gap-4">
                  <Link
                    href="/my-listings"
                    className="flex items-center gap-3 p-4 rounded-lg hover:bg-gray-50 transition-colors"
                  >
                    <div className="w-10 h-10 bg-spartan-green/10 rounded-full flex items-center justify-center">
                      📋
                    </div>
                    <div>
                      <p className="font-medium text-spartan-black">My Listings</p>
                      <p className="text-sm text-gray-600">
                        View and manage your tickets
                      </p>
                    </div>
                  </Link>
                  <Link
                    href="/games"
                    className="flex items-center gap-3 p-4 rounded-lg hover:bg-gray-50 transition-colors"
                  >
                    <div className="w-10 h-10 bg-spartan-green/10 rounded-full flex items-center justify-center">
                      🏟️
                    </div>
                    <div>
                      <p className="font-medium text-spartan-black">Upcoming Games</p>
                      <p className="text-sm text-gray-600">
                        See all upcoming events
                      </p>
                    </div>
                  </Link>
                </div>
              </CardContent>
            </Card>
          </div>
        </main>
        <Footer />
      </div>
    </AuthGuard>
  );
}
