<template>
  <div class="bg-background-light dark:bg-background-dark min-h-screen text-text-main dark:text-gray-100 p-4 md:p-8">
    <div class="mx-auto max-w-[1024px] flex flex-col gap-6">
      <header class="bg-surface-light dark:bg-surface-dark rounded-xl shadow-paper p-6 border border-border-color/50 dark:border-gray-700">
        <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
          <div class="flex flex-col gap-1">
            <h1 class="text-2xl md:text-3xl font-black text-text-main dark:text-white tracking-tight">2023年10月财务月结总结报告</h1>
            <p class="text-sm text-text-secondary">周期: 2023/10/01 - 2023/10/31</p>
          </div>
          <button class="bg-primary text-white px-5 py-2.5 rounded-lg text-sm font-bold">导出PDF</button>
        </div>
      </header>
      <section class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <div class="bg-surface-light dark:bg-surface-dark p-6 rounded-xl border border-border-color/60 dark:border-gray-700 shadow-card flex flex-col justify-between h-36">
          <p class="text-text-secondary dark:text-gray-400 font-medium text-sm">本月营收</p>
          <div>
            <p class="text-2xl font-bold text-text-main dark:text-white font-mono-nums mb-1">¥1,240,000</p>
            <div class="flex items-center gap-1 text-success text-sm font-medium bg-success/10 w-fit px-2 py-0.5 rounded">
              <span class="material-symbols-outlined text-[16px]">trending_up</span>
              <span>+5.2%</span>
            </div>
          </div>
        </div>
        <!-- More cards... -->
      </section>
    </div>
  </div>
</template>
