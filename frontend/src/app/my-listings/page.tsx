'use client';

import { useState, useEffect } from 'react';
import Link from 'next/link';
import { Navbar } from '@/components/layout/Navbar';
import { Footer } from '@/components/layout/Footer';
import { TicketList } from '@/components/tickets/TicketList';
import { Button } from '@/components/ui/Button';
import { AuthGuard } from '@/components/layout/AuthGuard';
import { api } from '@/lib/api';
import { Ticket, TicketStatus } from '@/types';

const STATUS_FILTERS: { label: string; value: TicketStatus | 'all' }[] = [
  { label: 'All', value: 'all' },
  { label: 'Unverified', value: 'Unverified' },
  { label: 'Verifying', value: 'Verifying' },
  { label: 'Verified', value: 'Verified' },
  { label: 'Reserved', value: 'Reserved' },
  { label: 'Paid', value: 'Paid' },
  { label: 'Sold', value: 'Sold' },
];

export default function MyListingsPage() {
  const [tickets, setTickets] = useState<Ticket[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [activeFilter, setActiveFilter] = useState<TicketStatus | 'all'>('all');

  useEffect(() => {
    async function fetchListings() {
      setIsLoading(true);
      try {
        const status = activeFilter === 'all' ? undefined : activeFilter.toLowerCase();
        const { tickets } = await api.getMyListings(status);
        setTickets(tickets);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load listings');
      } finally {
        setIsLoading(false);
      }
    }
    fetchListings();
  }, [activeFilter]);

  return (
    <AuthGuard>
      <div className="min-h-screen flex flex-col">
        <Navbar />
        <main className="flex-1 px-6 py-12">
          <div className="max-w-6xl mx-auto">
            <div className="flex justify-between items-center mb-8">
              <h1 className="text-3xl font-bold text-spartan-black">
                My Listings
              </h1>
              <Link href="/sell">
                <Button>List New Ticket</Button>
              </Link>
            </div>

            {/* Status Filters */}
            <div className="flex flex-wrap gap-2 mb-6">
              {STATUS_FILTERS.map((filter) => (
                <button
                  key={filter.value}
                  onClick={() => setActiveFilter(filter.value)}
                  className={`px-4 py-2 rounded-full text-sm font-medium transition-colors ${
                    activeFilter === filter.value
                      ? 'bg-spartan-green text-white'
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  {filter.label}
                </button>
              ))}
            </div>

            {isLoading ? (
              <div className="flex justify-center py-12">
                <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-spartan-green"></div>
              </div>
            ) : error ? (
              <div className="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg">
                {error}
              </div>
            ) : (
              <TicketList
                tickets={tickets}
                showStatus
                emptyMessage={
                  activeFilter === 'all'
                    ? "You haven't listed any tickets yet."
                    : `No ${activeFilter.toLowerCase()} tickets.`
                }
              />
            )}
          </div>
        </main>
        <Footer />
      </div>
    </AuthGuard>
  );
}
