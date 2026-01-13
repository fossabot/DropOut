<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount, onDestroy } from "svelte";

  export let visible = false;

  let logs: { type: 'stdout' | 'stderr', line: string }[] = [];
  let consoleElement: HTMLDivElement;
  let unlistenStdout: () => void;
  let unlistenStderr: () => void;

  onMount(async () => {
    unlistenStdout = await listen<string>("game-stdout", (event) => {
      addLog('stdout', event.payload);
    });

    unlistenStderr = await listen<string>("game-stderr", (event) => {
      addLog('stderr', event.payload);
    });
  });

  onDestroy(() => {
    if (unlistenStdout) unlistenStdout();
    if (unlistenStderr) unlistenStderr();
  });

  function addLog(type: 'stdout' | 'stderr', line: string) {
    logs = [...logs, { type, line }];
    if (logs.length > 1000) {
      logs = logs.slice(logs.length - 1000);
    }
    // Auto-scroll
    setTimeout(() => {
        if (consoleElement) {
            consoleElement.scrollTop = consoleElement.scrollHeight;
        }
    }, 0);
  }

  function clearLogs() {
      logs = [];
  }
</script>

{#if visible}
<div class="fixed bottom-0 left-0 right-0 h-64 bg-zinc-950/95 border-t border-zinc-700 backdrop-blur flex flex-col z-50 transition-transform duration-300 transform translate-y-0">
  <div class="flex items-center justify-between px-4 py-2 border-b border-zinc-800 bg-zinc-900/50">
    <span class="text-xs font-bold text-zinc-400 uppercase tracking-widest">Game Console</span>
    <div class="flex gap-2">
        <button on:click={clearLogs} class="text-xs text-zinc-500 hover:text-white px-2 py-1 rounded transition">Clear</button>
        <button on:click={() => visible = false} class="text-xs text-zinc-500 hover:text-white px-2 py-1 rounded transition">Close</button>
    </div>
  </div>
  <div bind:this={consoleElement} class="flex-1 overflow-y-auto p-4 font-mono text-xs space-y-1">
    {#each logs as log}
      <div class="{log.type === 'stderr' ? 'text-red-400' : 'text-zinc-300'} whitespace-pre-wrap break-all border-l-2 pl-2 {log.type === 'stderr' ? 'border-red-900/50' : 'border-transparent'}">
        {log.line}
      </div>
    {/each}
    {#if logs.length === 0}
        <div class="text-zinc-600 italic">Waiting for game output...</div>
    {/if}
  </div>
</div>
{/if}
