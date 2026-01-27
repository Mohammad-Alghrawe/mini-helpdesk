import { CommonModule } from '@angular/common';
import { HttpClient } from '@angular/common/http';
import {
  Component,
  inject,
  signal,
} from '@angular/core';
import { FormsModule } from '@angular/forms';
import {
  ActivatedRoute,
  RouterModule,
} from '@angular/router';

interface Ticket {
  id: string;
  title: string;
  description?: string;
  priority: 'low' | 'medium' | 'high';
  status: 'open' | 'in_progress' | 'closed';
  created_at: string;
  updated_at: string;
}

@Component({
  selector: 'app-ticket-details',
  standalone: true,
  imports: [CommonModule, RouterModule, FormsModule],
  templateUrl: './ticket-details.html',
  styleUrl: './ticket-details.scss',
})
export class TicketDetailsComponent {
  private route = inject(ActivatedRoute);
  private http = inject(HttpClient);

  loading = signal(true);
  error = signal<string | null>(null);
  saving = signal(false);

  ticket = signal<Ticket | null>(null);

  ngOnInit() {
    const id = this.route.snapshot.paramMap.get('id')!;
    this.loadTicket(id);
  }

  loadTicket(id: string) {
    this.http.get<{ ticket: Ticket }>(`http://localhost:8080/api/tickets/${id}`)
      .subscribe({
        next: (res) => {
          this.ticket.set(res.ticket);
          this.loading.set(false);
        },
        error: () => {
          this.error.set('Ticket not found.');
          this.loading.set(false);
        }
      });
  }

updateField(field: 'title' | 'description', value: string) {
    const t = this.ticket();
    if (!t) return;
    this.ticket.set({ ...t, [field]: value });
}

 saveChanges() {
  const t = this.ticket();
  if (!t) return;

  this.saving.set(true);

  const body = {
    title: t.title,
    description: t.description,
    status: t.status,
    priority: t.priority,
  };

  this.http.patch<{ ticket: Ticket }>(
    `http://localhost:8080/api/tickets/${t.id}`,
    body
  ).subscribe({
    next: (res) => {
      this.ticket.set(res.ticket);
      this.saving.set(false);
    },
    error: () => {
      this.error.set('Failed to save changes.');
      this.saving.set(false);
    }
  });
}
}
