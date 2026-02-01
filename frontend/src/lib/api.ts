import {
  Game,
  Ticket,
  LoginResponse,
  RegisterResponse,
  ReservationResponse,
  CreateTicketRequest
} from '@/types';

const API_BASE = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000';

class ApiClient {
  private token: string | null = null;

  constructor() {
    if (typeof window !== 'undefined') {
      this.token = localStorage.getItem('token');
    }
  }

  setToken(token: string) {
    this.token = token;
    if (typeof window !== 'undefined') {
      localStorage.setItem('token', token);
    }
  }

  clearToken() {
    this.token = null;
    if (typeof window !== 'undefined') {
      localStorage.removeItem('token');
    }
  }

  getToken() {
    return this.token;
  }

  private async request<T>(endpoint: string, options?: RequestInit): Promise<T> {
    const headers: HeadersInit = {
      'Content-Type': 'application/json',
      ...(this.token && { Authorization: this.token }),
    };

    const res = await fetch(`${API_BASE}${endpoint}`, {
      ...options,
      headers: { ...headers, ...options?.headers },
    });

    if (!res.ok) {
      const error = await res.json().catch(() => ({ error: 'Request failed' }));
      throw new Error(error.error || 'Request failed');
    }

    // Handle 204 No Content
    if (res.status === 204) {
      return {} as T;
    }

    return res.json();
  }

  // Auth
  async register(email: string, password: string): Promise<RegisterResponse> {
    return this.request('/api/auth/register', {
      method: 'POST',
      body: JSON.stringify({ email, password }),
    });
  }

  async login(email: string, password: string): Promise<LoginResponse> {
    const response = await this.request<LoginResponse>('/api/auth/login', {
      method: 'POST',
      body: JSON.stringify({ email, password }),
    });
    this.setToken(response.token);
    return response;
  }

  logout() {
    this.clearToken();
  }

  // Games
  async getGames(): Promise<{ games: Game[] }> {
    return this.request('/api/games');
  }

  // Tickets
  async getTickets(): Promise<{ tickets: Ticket[] }> {
    return this.request('/api/tickets');
  }

  async getMyListings(status?: string): Promise<{ tickets: Ticket[] }> {
    const query = status ? `?status=${status}` : '';
    return this.request(`/api/tickets/my-listings${query}`);
  }

  async createTicket(data: CreateTicketRequest): Promise<Ticket> {
    return this.request('/api/tickets', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }

  async reserveTicket(ticketId: string): Promise<ReservationResponse> {
    return this.request(`/api/tickets/${ticketId}/reserve`, {
      method: 'POST',
    });
  }
}

export const api = new ApiClient();
