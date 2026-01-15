<script lang="ts">
  import { onMount } from "svelte";
  import { instancesState } from "../stores/instances.svelte";
  import InstanceCard from "./InstanceCard.svelte";
  import InstanceEditModal from "./InstanceEditModal.svelte";
  import { Plus, FolderDown, FolderUp, RefreshCw } from "lucide-svelte";
  import { open, save } from "@tauri-apps/plugin-dialog";

  let searchQuery = $state("");

  // Filter instances by search
  let filteredInstances = $derived(() => {
    if (!searchQuery.trim()) return instancesState.instances;
    const query = searchQuery.toLowerCase();
    return instancesState.instances.filter(
      (i) =>
        i.name.toLowerCase().includes(query) ||
        i.version_id.toLowerCase().includes(query)
    );
  });

  onMount(() => {
    instancesState.loadInstances();
  });

  async function handleImport() {
    const selected = await open({
      multiple: false,
      filters: [{ name: "ZIP Archive", extensions: ["zip"] }],
    });

    if (selected) {
      await instancesState.importInstance(selected as string);
    }
  }

  async function handleExport(instanceId: string) {
    const instance = instancesState.instances.find((i) => i.id === instanceId);
    if (!instance) return;

    const path = await save({
      defaultPath: `${instance.name.replace(/[^a-zA-Z0-9]/g, "_")}.zip`,
      filters: [{ name: "ZIP Archive", extensions: ["zip"] }],
    });

    if (path) {
      await instancesState.exportInstance(instanceId, path);
    }
  }
</script>

<div class="h-full flex flex-col p-6 overflow-hidden">
  <!-- Header -->
  <div class="flex items-center justify-between mb-6">
    <div>
      <h2
        class="text-3xl font-black bg-clip-text text-transparent bg-gradient-to-r from-gray-900 to-gray-600 dark:from-white dark:to-white/60"
      >
        Instances
      </h2>
      <p class="text-sm dark:text-white/40 text-black/50 mt-1">
        Manage your game profiles with isolated mods, saves, and settings
      </p>
    </div>

    <div class="flex items-center gap-2">
      <!-- Refresh -->
      <button
        class="p-2.5 rounded-xl bg-white/60 dark:bg-white/5 border border-black/5 dark:border-white/10 hover:bg-white dark:hover:bg-white/10 transition-all"
        onclick={() => instancesState.loadInstances()}
        title="Refresh instances"
      >
        <RefreshCw
          size={18}
          class={instancesState.isLoading ? "animate-spin" : ""}
        />
      </button>

      <!-- Import -->
      <button
        class="p-2.5 rounded-xl bg-white/60 dark:bg-white/5 border border-black/5 dark:border-white/10 hover:bg-white dark:hover:bg-white/10 transition-all"
        onclick={handleImport}
        disabled={instancesState.isImporting}
        title="Import instance"
      >
        <FolderDown size={18} />
      </button>

      <!-- Create New -->
      <button
        class="flex items-center gap-2 px-4 py-2.5 rounded-xl bg-indigo-500 hover:bg-indigo-600 text-white font-medium transition-all shadow-lg shadow-indigo-500/20"
        onclick={() => instancesState.openCreateModal()}
      >
        <Plus size={18} />
        <span>New Instance</span>
      </button>
    </div>
  </div>

  <!-- Search -->
  <div class="mb-6">
    <div class="relative">
      <span
        class="absolute left-3 top-1/2 -translate-y-1/2 dark:text-white/30 text-black/30"
        >🔍</span
      >
      <input
        type="text"
        placeholder="Search instances..."
        class="w-full pl-9 pr-4 py-3 bg-white/60 dark:bg-black/20 border border-black/10 dark:border-white/10 rounded-xl dark:text-white text-gray-900 placeholder-black/30 dark:placeholder-white/30 focus:outline-none focus:border-indigo-500/50 dark:focus:bg-black/40 focus:bg-white/80 transition-all backdrop-blur-sm"
        bind:value={searchQuery}
      />
    </div>
  </div>

  <!-- Instances Grid -->
  <div class="flex-1 overflow-y-auto custom-scrollbar">
    {#if instancesState.isLoading && instancesState.instances.length === 0}
      <div
        class="flex items-center justify-center h-40 dark:text-white/30 text-black/30 italic animate-pulse"
      >
        Loading instances...
      </div>
    {:else if instancesState.error}
      <div
        class="p-6 border border-red-500/20 bg-red-500/10 text-red-400 rounded-xl"
      >
        {instancesState.error}
      </div>
    {:else if filteredInstances().length === 0}
      <div
        class="flex flex-col items-center justify-center h-64 dark:text-white/30 text-black/30 gap-4"
      >
        {#if instancesState.instances.length === 0}
          <div class="text-6xl">📦</div>
          <div class="text-center">
            <p class="text-lg font-medium dark:text-white/50 text-black/50">
              No instances yet
            </p>
            <p class="text-sm mt-1">
              Create your first instance to get started
            </p>
          </div>
          <button
            class="mt-4 flex items-center gap-2 px-6 py-3 rounded-xl bg-indigo-500 hover:bg-indigo-600 text-white font-medium transition-all"
            onclick={() => instancesState.openCreateModal()}
          >
            <Plus size={18} />
            <span>Create Instance</span>
          </button>
        {:else}
          <div class="text-4xl">🔍</div>
          <p>No instances match your search</p>
        {/if}
      </div>
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 pb-4">
        {#each filteredInstances() as instance (instance.id)}
          <InstanceCard
            {instance}
            isActive={instancesState.activeInstanceId === instance.id}
            onSelect={() => instancesState.setActiveInstance(instance.id)}
            onEdit={() => instancesState.openEditModal(instance)}
            onDuplicate={() =>
              instancesState.duplicateInstance(
                instance.id,
                `${instance.name} (Copy)`
              )}
            onExport={() => handleExport(instance.id)}
            onDelete={() => instancesState.deleteInstance(instance.id)}
          />
        {/each}
      </div>
    {/if}
  </div>
</div>

<!-- Edit/Create Modal -->
{#if instancesState.showEditModal}
  <InstanceEditModal />
{/if}
