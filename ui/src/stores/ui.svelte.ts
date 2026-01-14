export class UIState {
  currentView = $state("home");
  status = $state("Ready");
  showConsole = $state(false);
  appVersion = $state("...");
  
  private statusTimeout: any;

  constructor() {
    // Watch for status changes to auto-dismiss
    $effect(() => {
        if (this.status !== "Ready") {
            if (this.statusTimeout) clearTimeout(this.statusTimeout);
            this.statusTimeout = setTimeout(() => {
                this.status = "Ready";
            }, 5000);
        }
    });
  }

  setStatus(msg: string) {
    this.status = msg;
  }

  toggleConsole() {
    this.showConsole = !this.showConsole;
  }

  setView(view: string) {
    this.currentView = view;
  }
}

export const uiState = new UIState();
