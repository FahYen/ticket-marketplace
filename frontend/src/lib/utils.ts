import { TicketStatus } from '@/types';

export function formatPrice(cents: number): string {
  return `$${(cents / 100).toFixed(2)}`;
}

export function formatDate(dateString: string): string {
  const date = new Date(dateString);
  return date.toLocaleDateString('en-US', {
    weekday: 'short',
    month: 'short',
    day: 'numeric',
    year: 'numeric',
  });
}

export function formatDateTime(dateString: string): string {
  const date = new Date(dateString);
  return date.toLocaleString('en-US', {
    weekday: 'short',
    month: 'short',
    day: 'numeric',
    year: 'numeric',
    hour: 'numeric',
    minute: '2-digit',
  });
}

export const statusColors: Record<TicketStatus, string> = {
  Unverified: 'bg-yellow-100 text-yellow-800',
  Verifying: 'bg-blue-100 text-blue-800',
  Verified: 'bg-green-100 text-green-800',
  Reserved: 'bg-purple-100 text-purple-800',
  Paid: 'bg-emerald-100 text-emerald-800',
  Sold: 'bg-gray-100 text-gray-800',
  Cancelled: 'bg-red-100 text-red-800',
};

export function cn(...classes: (string | boolean | undefined)[]): string {
  return classes.filter(Boolean).join(' ');
}
