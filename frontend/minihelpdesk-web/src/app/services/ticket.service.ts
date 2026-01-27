import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';

import {
  map,
  Observable,
} from 'rxjs';

import { environment } from '../../environments/environment';

export type TicketPriority = 'low' | 'medium' | 'high';
export type TicketStatus = 'open' | 'in_progress' | 'closed';

export interface Ticket {
  id: string;
  title: string;
  description?: string | null;
  priority: TicketPriority;
  status: TicketStatus;
  created_at: string;
  updated_at: string;
}

export interface CreateTicketRequest {
  title: string;
  description?: string | null;
  priority?: TicketPriority;
}

@Injectable({
  providedIn: 'root',
})
export class TicketService {
  private readonly baseUrl = `${environment.apiUrl}/api/tickets`;

  constructor(private http: HttpClient) {}

listTickets(): Observable<{ tickets: Ticket[] }> {
  return this.http.get<{ tickets: Ticket[] }>(this.baseUrl).pipe(
    map((res) => {
      console.log('Raw response:', res);
      console.log('Response type:', typeof res);
      console.log('Is array?:', Array.isArray(res));

      if (Array.isArray(res)) {
        console.log('Returning wrapped array');
        return { tickets: res as Ticket[] };
      }
      console.log('Returning tickets property:', res?.tickets);
      return { tickets: (res?.tickets ?? []) as Ticket[] };
    })
  );
}

  createTicket(payload: CreateTicketRequest): Observable<{ ticket: Ticket }> {
    return this.http.post<{ ticket: Ticket }>(this.baseUrl, payload);
  }
}
