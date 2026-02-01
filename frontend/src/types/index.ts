export interface User {
  id: string;
  email: string;
  email_verified: boolean;
}

export interface Game {
  id: string;
  sport_type: 'Football' | 'Basketball' | 'Hockey';
  name: string;
  game_time: string;
  cutoff_time: string;
}

export type TicketStatus =
  | 'Unverified'
  | 'Verifying'
  | 'Verified'
  | 'Reserved'
  | 'Paid'
  | 'Sold'
  | 'Cancelled';

export interface Ticket {
  id: string;
  seller_id: string;
  game_id: string;
  event_name: string;
  event_date: string;
  level: string;
  seat_section: string;
  seat_row: string;
  seat_number: string;
  price: number;
  status: TicketStatus;
  transfer_deadline: string;
  price_at_reservation?: number;
  created_at: string;
}

export interface ReservationResponse {
  ticket_id: string;
  status: 'Reserved';
  price_at_reservation: number;
  reserved_at: string;
}

export interface LoginResponse {
  token: string;
  user: User;
}

export interface RegisterResponse {
  message: string;
  verification_code?: string;
}

export interface CreateTicketRequest {
  game_id: string;
  level: string;
  seat_section: string;
  seat_row: string;
  seat_number: string;
  price: number;
}
