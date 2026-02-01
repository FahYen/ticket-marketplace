'use client';

import { Ticket } from '@/types';
import { TicketCard } from './TicketCard';

interface TicketListProps {
  tickets: Ticket[];
  showStatus?: boolean;
  showBuyButton?: boolean;
  onBuy?: (ticketId: string) => void;
  emptyMessage?: string;
}

export function TicketList({
  tickets,
  showStatus = false,
  showBuyButton = false,
  onBuy,
  emptyMessage = 'No tickets available',
}: TicketListProps) {
  if (tickets.length === 0) {
    return (
      <div className="text-center py-12 text-gray-500">
        <p>{emptyMessage}</p>
      </div>
    );
  }

  return (
    <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      {tickets.map((ticket) => (
        <TicketCard
          key={ticket.id}
          ticket={ticket}
          showStatus={showStatus}
          showBuyButton={showBuyButton}
          onBuy={onBuy}
        />
      ))}
    </div>
  );
}
