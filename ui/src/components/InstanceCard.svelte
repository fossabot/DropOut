<script lang="ts">
  import type { Instance } from "../types";
  import { instancesState } from "../stores/instances.svelte";
  import {
    Play,
    Settings,
    Copy,
    Trash2,
    FolderUp,
    Clock,
    Box,
    MoreVertical,
  } from "lucide-svelte";

  interface Props {
    instance: Instance;
    isActive: boolean;
    onSelect: () => void;
    onEdit: () => void;
    onDuplicate: () => void | Promise<unknown>;
    onExport: () => void | Promise<void>;
    onDelete: () => void | Promise<void>;
  }

  let {
    instance,
    isActive,
    onSelect,
    onEdit,
    onDuplicate,
    onExport,
    onDelete,
  }: Props = $props();

  let showMenu = $state(false);

  function getVersionBadgeClass(versionId: string): string {
    if (versionId.includes("fabric")) {
      return "bg-indigo-500/20 text-indigo-300 border-indigo-500/30";
    }
    if (versionId.includes("forge")) {
      return "bg-orange-500/20 text-orange-300 border-orange-500/30";
    }
    if (versionId.includes("snapshot") || versionId.includes("w")) {
      return "bg-amber-500/20 text-amber-300 border-amber-500/30";
    }
    return "bg-emerald-500/20 text-emerald-300 border-emerald-500/30";
  }

  function handleMenuClick(e: MouseEvent) {
    e.stopPropagation();
    showMenu = !showMenu;
  }

  function handleMenuAction(action: () => void) {
    showMenu = false;
    action();
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="group relative rounded-2xl border transition-all duration-200 cursor-pointer overflow-hidden
    {isActive
    ? 'bg-indigo-50 dark:bg-indigo-600/20 border-indigo-200 dark:border-indigo-500/50 shadow-lg shadow-indigo-500/10'
    : 'bg-white/40 dark:bg-white/5 border-black/5 dark:border-white/5 hover:bg-white/60 dark:hover:bg-white/10 hover:border-black/10 dark:hover:border-white/10'}"
  onclick={onSelect}
>
  <!-- Active Indicator Glow -->
  {#if isActive}
    <div
      class="absolute inset-0 bg-gradient-to-br from-indigo-500/10 to-transparent pointer-events-none"
    ></div>
  {/if}

  <!-- Card Content -->
  <div class="relative z-10 p-5">
    <!-- Header -->
    <div class="flex items-start justify-between mb-4">
      <div class="flex items-center gap-3">
        <!-- Icon -->
        <div
          class="w-12 h-12 rounded-xl flex items-center justify-center text-2xl
            {isActive
            ? 'bg-indigo-500/20 border border-indigo-500/30'
            : 'bg-white/60 dark:bg-white/10 border border-black/5 dark:border-white/10'}"
        >
          {#if instance.icon}
            <img
              src={instance.icon}
              alt=""
              class="w-8 h-8 object-cover rounded"
            />
          {:else}
            <Box
              size={24}
              class={isActive
                ? "text-indigo-500"
                : "text-black/30 dark:text-white/30"}
            />
          {/if}
        </div>

        <div>
          <h3
            class="font-bold text-lg truncate max-w-[180px]
              {isActive
              ? 'text-indigo-700 dark:text-white'
              : 'text-gray-800 dark:text-white'}"
          >
            {instance.name}
          </h3>
          <span
            class="inline-block px-2 py-0.5 rounded-full text-[10px] font-bold uppercase tracking-wide border {getVersionBadgeClass(
              instance.version_id
            )}"
          >
            {instance.version_id}
          </span>
        </div>
      </div>

      <!-- Menu Button -->
      <div class="relative">
        <button
          class="p-1.5 rounded-lg opacity-0 group-hover:opacity-100 transition-opacity
            hover:bg-black/5 dark:hover:bg-white/10"
          onclick={handleMenuClick}
        >
          <MoreVertical size={16} class="dark:text-white/50 text-black/50" />
        </button>

        <!-- Dropdown Menu -->
        {#if showMenu}
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="absolute right-0 top-8 w-40 py-1 bg-white dark:bg-zinc-900 rounded-xl border border-black/10 dark:border-white/10 shadow-xl z-50"
            onclick={(e) => e.stopPropagation()}
          >
            <button
              class="w-full px-3 py-2 text-left text-sm flex items-center gap-2 hover:bg-black/5 dark:hover:bg-white/5 dark:text-white"
              onclick={() => handleMenuAction(onEdit)}
            >
              <Settings size={14} />
              Edit
            </button>
            <button
              class="w-full px-3 py-2 text-left text-sm flex items-center gap-2 hover:bg-black/5 dark:hover:bg-white/5 dark:text-white"
              onclick={() => handleMenuAction(onDuplicate)}
            >
              <Copy size={14} />
              Duplicate
            </button>
            <button
              class="w-full px-3 py-2 text-left text-sm flex items-center gap-2 hover:bg-black/5 dark:hover:bg-white/5 dark:text-white"
              onclick={() => handleMenuAction(onExport)}
            >
              <FolderUp size={14} />
              Export
            </button>
            <hr class="my-1 border-black/5 dark:border-white/10" />
            <button
              class="w-full px-3 py-2 text-left text-sm flex items-center gap-2 hover:bg-red-500/10 text-red-500"
              onclick={() => handleMenuAction(onDelete)}
            >
              <Trash2 size={14} />
              Delete
            </button>
          </div>
        {/if}
      </div>
    </div>

    <!-- Stats -->
    <div class="flex items-center gap-4 text-xs dark:text-white/40 text-black/40">
      <div class="flex items-center gap-1.5">
        <Clock size={12} />
        <span>{instancesState.formatRelativeTime(instance.last_played)}</span>
      </div>
    </div>

    <!-- Quick Actions (visible on hover) -->
    <div
      class="mt-4 pt-4 border-t border-black/5 dark:border-white/5 flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity"
    >
      <button
        class="flex-1 flex items-center justify-center gap-2 py-2 rounded-lg text-sm font-medium
          {isActive
          ? 'bg-indigo-500 text-white hover:bg-indigo-600'
          : 'bg-black/5 dark:bg-white/10 dark:text-white hover:bg-black/10 dark:hover:bg-white/20'}"
        onclick={(e) => {
          e.stopPropagation();
          onSelect();
        }}
      >
        <Play size={14} />
        {isActive ? "Selected" : "Select"}
      </button>
      <button
        class="p-2 rounded-lg bg-black/5 dark:bg-white/10 hover:bg-black/10 dark:hover:bg-white/20 transition-colors"
        onclick={(e) => {
          e.stopPropagation();
          onEdit();
        }}
        title="Edit instance"
      >
        <Settings size={14} class="dark:text-white" />
      </button>
    </div>
  </div>

  <!-- Active Badge -->
  {#if isActive}
    <div
      class="absolute top-3 right-3 px-2 py-0.5 rounded-full text-[10px] font-bold uppercase tracking-wide bg-indigo-500 text-white"
    >
      Active
    </div>
  {/if}
</div>

<!-- Click outside to close menu -->
{#if showMenu}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 z-40" onclick={() => (showMenu = false)}></div>
{/if}
