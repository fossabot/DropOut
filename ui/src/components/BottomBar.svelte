<script lang="ts">
  import { authState } from "../stores/auth.svelte";
  import { gameState } from "../stores/game.svelte";
  import { uiState } from "../stores/ui.svelte";
  import { Terminal, ChevronDown, Play, User } from 'lucide-svelte';
</script>

<div
  class="h-20 bg-white/80 dark:bg-[#09090b]/90 border-t dark:border-white/10 border-black/5 flex items-center px-8 justify-between z-20 backdrop-blur-md"
>
  <!-- Account Area -->
  <div class="flex items-center gap-6">
    <div
      class="group flex items-center gap-4 cursor-pointer"
      onclick={() => authState.openLoginModal()}
      role="button"
      tabindex="0"
      onkeydown={(e) => e.key === "Enter" && authState.openLoginModal()}
    >
      <div
        class="w-10 h-10 rounded-sm bg-zinc-100 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 flex items-center justify-center overflow-hidden transition-all group-hover:border-zinc-400 dark:group-hover:border-zinc-500"
      >
        {#if authState.currentAccount}
          <img
            src={`https://minotar.net/avatar/${authState.currentAccount.username}/48`}
            alt={authState.currentAccount.username}
            class="w-full h-full"
          />
        {:else}
          <User size={20} class="text-zinc-400" />
        {/if}
      </div>
      <div>
        <div class="font-bold dark:text-white text-gray-900 text-sm group-hover:text-black dark:group-hover:text-zinc-200 transition-colors">
          {authState.currentAccount ? authState.currentAccount.username : "Login Account"}
        </div>
        <div class="text-[10px] uppercase tracking-wider dark:text-zinc-500 text-gray-500 flex items-center gap-2">
          <span
            class="w-1.5 h-1.5 rounded-full {authState.currentAccount
              ? 'bg-emerald-500'
              : 'bg-zinc-400'}"
          ></span>
          {authState.currentAccount ? "Online" : "Guest"}
        </div>
      </div>
    </div>
    
    <div class="h-8 w-px dark:bg-white/10 bg-black/10"></div>
    
    <!-- Console Toggle -->
    <button
      class="text-xs font-mono dark:text-zinc-500 text-gray-500 dark:hover:text-white hover:text-black transition-colors flex items-center gap-2 px-2 py-1 rounded-sm hover:bg-black/5 dark:hover:bg-white/5"
      onclick={() => uiState.toggleConsole()}
    >
      <Terminal size={14} />
      {uiState.showConsole ? "HIDE LOGS" : "SHOW LOGS"}
    </button>
  </div>

  <!-- Action Area -->
  <div class="flex items-center gap-4">
    <div class="flex flex-col items-end mr-2">
      <div class="relative group">
        <select
          id="version-select"
          bind:value={gameState.selectedVersion}
          class="appearance-none dark:bg-zinc-900 bg-zinc-50 dark:text-white text-gray-900 border dark:border-white/10 border-black/10 rounded-sm pl-4 pr-10 py-2 dark:hover:border-white/30 hover:border-black/30 transition-all cursor-pointer outline-none focus:ring-1 focus:ring-zinc-500 w-56 text-sm font-mono"
        >
          {#if gameState.versions.length === 0}
            <option>Loading...</option>
          {:else}
            {#each gameState.versions as version}
              <option value={version.id}>{version.id} {version.type !== 'release' ? `(${version.type})` : ''}</option>
            {/each}
          {/if}
        </select>
        <div class="absolute right-3 top-1/2 -translate-y-1/2 dark:text-white/20 text-black/20 pointer-events-none">
            <ChevronDown size={14} />
        </div>
      </div>
    </div>

    <button
      onclick={() => gameState.startGame()}
      class="bg-emerald-600 hover:bg-emerald-500 text-white h-14 px-10 rounded-sm transition-all duration-200 hover:scale-[1.02] active:scale-[0.98] shadow-lg shadow-emerald-500/20 flex items-center gap-3 font-bold text-lg tracking-widest uppercase"
    >
      <Play size={24} fill="currentColor" />
      <span>Launch</span>
    </button>
  </div>
</div>
