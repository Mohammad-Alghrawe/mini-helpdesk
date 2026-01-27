import { Routes } from '@angular/router';

import {
  CreateTicketComponent,
} from './pages/create-ticket/create-ticket.component';
import { TicketListComponent } from './pages/ticket-list/ticket-list.component';

export const routes: Routes = [
  { path: '', pathMatch: 'full', redirectTo: 'tickets' },
  { path: 'tickets', component: TicketListComponent },
  { path: 'tickets/new', component: CreateTicketComponent },
  {
    path: 'tickets/:id',
    loadComponent: () =>
      import('./pages/ticket-details/ticket-details').then(
        (m) => m.TicketDetailsComponent
      ),
  },
];

