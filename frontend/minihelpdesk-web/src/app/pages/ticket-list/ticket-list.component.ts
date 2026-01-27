import { CommonModule } from '@angular/common';
import {
  ChangeDetectorRef,
  Component,
  inject,
  OnInit,
} from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RouterModule } from '@angular/router';

import { timeout } from 'rxjs';

import {
  Ticket,
  TicketService,
} from '../../services/ticket.service';

@Component({
  selector: 'app-ticket-list',
  standalone: true,
  imports: [CommonModule, RouterModule, FormsModule],
  templateUrl: './ticket-list.component.html',
  styleUrls: ['./ticket-list.component.scss'],
})
export class TicketListComponent implements OnInit {
  private ticketService = inject(TicketService);
  private cdr = inject(ChangeDetectorRef);

  loading = true;
  errorMsg: string | null = null;
  tickets: Ticket[] = [];
  searchText = '';
  statusFilter: 'all' | 'open' | 'in_progress' | 'closed' = 'all';
  filteredTickets: Ticket[] = [];
  stats = {
  open: 0,
  in_progress: 0,
  closed: 0,

};

  ngOnInit(): void {
    this.load();
  }

  load(): void {
    this.loading = true;
    this.errorMsg = null;

    this.ticketService
      .listTickets()
      .pipe(timeout(5000))
      .subscribe({
        next: (res) => {
          this.tickets = res?.tickets ?? [];
          this.applyFilters();
          this.calculateStats(this.tickets);
          this.loading = false;
          this.cdr.detectChanges(); // ✅ Force change detection
        },
        error: (err) => {
          this.loading = false;
          this.errorMsg =
            err?.name === 'TimeoutError'
              ? 'API timeout. Is backend running on :8080?'
              : (err?.error?.error ?? 'Failed to load tickets.');
          this.cdr.detectChanges(); // ✅ Force change detection
        },
      });
  }

  private calculateStats(tickets: Ticket[]) {
  this.stats = {
    open: tickets.filter(t => t.status === 'open').length,
    in_progress: tickets.filter(t => t.status === 'in_progress').length,
    closed: tickets.filter(t => t.status === 'closed').length,
  };
}

applyFilters() {
  let filtered = [...this.tickets];

  // Search filter
  if (this.searchText.trim().length > 0) {
    const search = this.searchText.toLowerCase();
    filtered = filtered.filter(t =>
      t.title.toLowerCase().includes(search) ||
      (t.description ?? '').toLowerCase().includes(search)
    );
  }

  // Status filter
  if (this.statusFilter !== 'all') {
    filtered = filtered.filter(t => t.status === this.statusFilter);
  }

  this.filteredTickets = filtered;
}

}
