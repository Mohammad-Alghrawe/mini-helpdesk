import {
  Injectable,
  signal,
} from '@angular/core';

export interface ToastMessage {
  id: number;
  type: 'success' | 'error';
  text: string;
}

@Injectable({ providedIn: 'root' })
export class ToastService {
  toasts = signal<ToastMessage[]>([]);

  show(type: 'success' | 'error', text: string) {
    const id = Date.now();

    this.toasts.update(t => [...t, { id, type, text }]);

    setTimeout(() => {
      this.toasts.update(t => t.filter(msg => msg.id !== id));
    }, 3000);
  }
}
