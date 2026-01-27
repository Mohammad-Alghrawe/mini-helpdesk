import { CommonModule } from '@angular/common';
import {
  ChangeDetectorRef,
  Component,
  inject,
  OnInit,
} from '@angular/core';
import { RouterModule } from '@angular/router';

import { timeout } from 'rxjs';

import {
  Ticket,
  TicketService,
} from '../../services/ticket.service';

@Component({
  selector: 'app-ticket-list',
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: './ticket-list.component.html',
  styleUrls: ['./ticket-list.component.scss'],
})
export class TicketListComponent implements OnInit {
  private ticketService = inject(TicketService);
  private cdr = inject(ChangeDetectorRef);

  loading = true;
  errorMsg: string | null = null;
  tickets: Ticket[] = [];

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
}
