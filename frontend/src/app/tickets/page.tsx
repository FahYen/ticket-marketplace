'use client';

import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { Navbar } from '@/components/layout/Navbar';
import { Footer } from '@/components/layout/Footer';
import { TicketList } from '@/components/tickets/TicketList';
import { api } from '@/lib/api';
import { Ticket } from '@/types';
import { useAuth } from '@/lib/auth';

export default function TicketsPage() {
  const [tickets, setTickets] = useState<Ticket[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const { isAuthenticated } = useAuth();
  const router = useRouter();

  useEffect(() => {
    async function fetchTickets() {
      try {
        const { tickets } = await api.getTickets();
        setTickets(tickets);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load tickets');
      } finally {
        setIsLoading(false);
      }
    }
    fetchTickets();
  }, []);

  const handleBuy = async (ticketId: string) => {
    if (!isAuthenticated) {
      router.push('/login');
      return;
    }

    try {
      const reservation = await api.reserveTicket(ticketId);
      // For now, just show an alert since Stripe is not integrated
      alert(
        `Ticket reserved! Price: $${(reservation.price_at_reservation / 100).toFixed(2)}\n\nNote: Payment integration coming soon.`
      );
      // Refresh tickets
      const { tickets } = await api.getTickets();
      setTickets(tickets);
    } catch (err) {
      alert(err instanceof Error ? err.message : 'Failed to reserve ticket');
    }
  };

  return (
    <div className="min-h-screen flex flex-col">
      <Navbar />
      <main className="flex-1 px-6 py-12">
        <div className="max-w-6xl mx-auto">
          <h1 className="text-3xl font-bold text-spartan-black mb-8">
            Available Tickets
          </h1>

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
              showBuyButton
              onBuy={handleBuy}
              emptyMessage="No tickets available at the moment. Check back later!"
            />
          )}
        </div>
      </main>
      <Footer />
    </div>
  );
}
