import { HTMLAttributes, forwardRef } from 'react';
import { cn, statusColors } from '@/lib/utils';
import { TicketStatus } from '@/types';

interface BadgeProps extends HTMLAttributes<HTMLSpanElement> {
  status: TicketStatus;
}

const Badge = forwardRef<HTMLSpanElement, BadgeProps>(
  ({ className, status, ...props }, ref) => {
    return (
      <span
        ref={ref}
        className={cn(
          'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium',
          statusColors[status],
          className
        )}
        {...props}
      >
        {status}
      </span>
    );
  }
);

Badge.displayName = 'Badge';

export { Badge };
