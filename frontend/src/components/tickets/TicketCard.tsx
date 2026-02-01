'use client';

import { Ticket } from '@/types';
import { Card, CardContent } from '@/components/ui/Card';
import { Badge } from '@/components/ui/Badge';
import { Button } from '@/components/ui/Button';
import { formatPrice, formatDate } from '@/lib/utils';

interface TicketCardProps {
  ticket: Ticket;
  showStatus?: boolean;
  showBuyButton?: boolean;
  onBuy?: (ticketId: string) => void;
}

export function TicketCard({
  ticket,
  showStatus = false,
  showBuyButton = false,
  onBuy,
}: TicketCardProps) {
  return (
    <Card className="hover:shadow-lg transition-shadow">
      <CardContent className="p-4">
        <div className="flex justify-between items-start">
          <div className="flex-1">
            <h3 className="font-semibold text-lg text-spartan-black">
              {ticket.event_name}
            </h3>
            <p className="text-gray-600 text-sm mt-1">
              {formatDate(ticket.event_date)}
            </p>
            <div className="mt-3 text-sm text-gray-700">
              <p>
                <span className="font-medium">Section:</span> {ticket.seat_section}
              </p>
              <p>
                <span className="font-medium">Row:</span> {ticket.seat_row}
              </p>
              <p>
                <span className="font-medium">Seat:</span> {ticket.seat_number}
              </p>
              <p>
                <span className="font-medium">Level:</span> {ticket.level}
              </p>
            </div>
          </div>

          <div className="text-right flex flex-col items-end gap-2">
            {showStatus && <Badge status={ticket.status} />}
            <p className="text-2xl font-bold text-spartan-green">
              {formatPrice(ticket.price)}
            </p>
            {showBuyButton && ticket.status === 'Verified' && (
              <Button
                size="sm"
                onClick={() => onBuy?.(ticket.id)}
              >
                Buy Ticket
              </Button>
            )}
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
