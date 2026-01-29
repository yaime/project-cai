<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import { useAppStore } from '../stores/app'
import { storeToRefs } from 'pinia'

const appStore = useAppStore()
const { isDark } = storeToRefs(appStore)
const { toggleTheme } = appStore
</script>

<template>
  <div class="flex h-screen w-full overflow-hidden bg-background-light dark:bg-background-dark text-text-main font-display">
    <!-- Sidebar -->
    <aside class="w-64 bg-white dark:bg-gray-900 border-r border-border-light dark:border-gray-800 flex flex-col justify-between h-full shadow-naive z-10 hidden md:flex shrink-0">
      <div class="flex flex-col h-full">
        <div class="h-16 flex items-center px-6 border-b border-border-light dark:border-gray-800">
          <div class="flex items-center gap-2 text-primary font-bold text-xl tracking-tight">
            <!-- Icon -->
            <svg class="w-8 h-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
            </svg>
            <span>FinLedger</span>
          </div>
        </div>
        <nav class="flex-1 px-3 py-6 space-y-1 overflow-y-auto">
          <router-link to="/dashboard" class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-text-sub hover:bg-gray-50 dark:hover:bg-gray-800 hover:text-text-main group transition-colors" active-class="bg-primary/10 text-primary hover:text-primary">
            <span class="material-symbols-outlined">dashboard</span>
            <span class="text-sm font-medium">Dashboard</span>
          </router-link>

          <router-link to="/ledger/summary" class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-text-sub hover:bg-gray-50 dark:hover:bg-gray-800 hover:text-text-main group transition-colors" active-class="bg-primary/10 text-primary hover:text-primary">
            <span class="material-symbols-outlined">book</span>
            <span class="text-sm font-medium">General Ledger</span>
          </router-link>

          <router-link to="/voucher/entry" class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-text-sub hover:bg-gray-50 dark:hover:bg-gray-800 hover:text-text-main group transition-colors" active-class="bg-primary/10 text-primary hover:text-primary">
            <span class="material-symbols-outlined">receipt_long</span>
            <span class="text-sm font-medium">Vouchers</span>
          </router-link>

           <router-link to="/assets/list" class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-text-sub hover:bg-gray-50 dark:hover:bg-gray-800 hover:text-text-main group transition-colors" active-class="bg-primary/10 text-primary hover:text-primary">
            <span class="material-symbols-outlined">domain</span>
            <span class="text-sm font-medium">Assets</span>
          </router-link>

          <div class="pt-4 mt-4 border-t border-border-light dark:border-gray-800">
            <p class="px-3 text-xs font-semibold text-text-sub uppercase tracking-wider mb-2">System</p>
            <a href="#" class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-text-sub hover:bg-gray-50 dark:hover:bg-gray-800 hover:text-text-main group transition-colors">
              <span class="material-symbols-outlined">settings</span>
              <span class="text-sm font-medium">Settings</span>
            </a>
          </div>
        </nav>
        <div class="p-4 border-t border-border-light dark:border-gray-800">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-full bg-gray-200 dark:bg-gray-700"></div>
            <div class="flex flex-col overflow-hidden">
              <span class="text-sm font-medium text-text-main truncate">James Anderson</span>
              <span class="text-xs text-text-sub truncate">Senior Accountant</span>
            </div>
          </div>
        </div>
      </div>
    </aside>

    <!-- Main Content -->
    <main class="flex-1 flex flex-col h-full overflow-hidden relative">
      <header class="h-16 bg-white dark:bg-gray-900 border-b border-border-light dark:border-gray-800 flex items-center justify-between px-6 shadow-sm shrink-0">
        <div class="flex items-center gap-4">
          <h1 class="text-lg font-semibold text-text-main hidden sm:block">
            {{ $route.meta.title || 'Workspace' }}
          </h1>
        </div>
        <div class="flex items-center gap-4">
           <button @click="toggleTheme" class="p-2 text-text-sub hover:bg-gray-100 dark:hover:bg-gray-800 rounded-full">
            <span class="material-symbols-outlined">{{ isDark ? 'light_mode' : 'dark_mode' }}</span>
          </button>
          <div class="relative hidden sm:block">
            <span class="material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 text-[20px]">search</span>
            <input class="pl-10 pr-4 py-2 text-sm border border-border-light dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-800 focus:bg-white dark:focus:bg-gray-900 focus:ring-1 focus:ring-primary focus:border-primary outline-none transition-all w-64" placeholder="Search..." type="text"/>
          </div>
          <button class="p-2 text-text-sub hover:bg-gray-100 dark:hover:bg-gray-800 rounded-full relative">
            <span class="material-symbols-outlined">notifications</span>
            <span class="absolute top-2 right-2 w-2 h-2 bg-red-500 rounded-full border-2 border-white dark:border-gray-900"></span>
          </button>
        </div>
      </header>

      <div class="flex-1 overflow-hidden relative">
        <router-view></router-view>
      </div>
    </main>
  </div>
</template>
