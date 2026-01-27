import { CommonModule } from '@angular/common';
import {
  Component,
  inject,
} from '@angular/core';

import { ToastService } from '../../services/toast/toast.service';

@Component({
  selector: 'app-toast',
  standalone: true,
  imports: [CommonModule],
  template: `
    <div class="toast-container">
      <div *ngFor="let t of toastService.toasts()" 
           class="toast"
           [ngClass]="t.type">
        {{ t.text }}
      </div>
    </div>
  `,
  styleUrls: ['./toast.scss'],
})
export class ToastComponent {
  toastService = inject(ToastService);
}
