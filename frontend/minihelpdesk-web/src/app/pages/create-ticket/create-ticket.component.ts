import { CommonModule } from '@angular/common';
import {
  Component,
  inject,
} from '@angular/core';
import {
  FormBuilder,
  ReactiveFormsModule,
  Validators,
} from '@angular/forms';
import { Router } from '@angular/router';

import {
  TicketPriority,
  TicketService,
} from '../../services/ticket.service';

@Component({
  selector: 'app-create-ticket',
  standalone: true,
  imports: [CommonModule, ReactiveFormsModule],
  templateUrl: './create-ticket.component.html',
  styleUrl: './create-ticket.component.scss',
})
export class CreateTicketComponent {
  private fb = inject(FormBuilder);
  loading = false;
  errorMsg: string | null = null;

  priorities: TicketPriority[] = ['low', 'medium', 'high'];

  form = this.fb.nonNullable.group({
    title: ['', [Validators.required, Validators.minLength(3)]],
    description: [''],
    priority: ['medium' as TicketPriority],
  });

  constructor(
    private ticketService: TicketService,
    private router: Router
  ) {}

  submit() {
    this.errorMsg = null;

    if (this.form.invalid) {
      this.form.markAllAsTouched();
      return;
    }

    this.loading = true;

    const payload = {
      title: this.form.value.title!.trim(),
      description: (this.form.value.description ?? '').trim() || null,
      priority: this.form.value.priority ?? 'medium',
    };

    this.ticketService.createTicket(payload).subscribe({
      next: () => {
        this.loading = false;
        this.router.navigate(['/tickets']);
      },
      error: (err) => {
        this.loading = false;
        this.errorMsg =
          err?.error?.error ?? 'Something went wrong while creating the ticket.';
      },
    });
  }
}
