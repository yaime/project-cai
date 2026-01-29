<template>
  <div class="flex flex-col h-full bg-background-light dark:bg-background-dark overflow-y-auto custom-scrollbar p-6">
    <div class="max-w-[1600px] mx-auto w-full flex flex-col gap-6">
      <div class="flex flex-col md:flex-row md:items-center justify-between gap-4 shrink-0">
        <div>
          <h2 class="text-2xl font-bold tracking-tight text-text-primary-light dark:text-white">固定资产卡片列表</h2>
          <p class="text-sm text-text-secondary-light mt-1">管理企业所有固定资产及其全生命周期状态</p>
        </div>
      </div>
      <!-- Toolbar -->
      <div class="bg-surface-light dark:bg-surface-dark p-4 rounded-t-xl border border-border-light dark:border-border-dark border-b-0 flex flex-col xl:flex-row xl:items-center justify-between gap-4 shrink-0 shadow-sm">
        <div class="flex flex-wrap items-center gap-3">
          <button class="flex items-center gap-2 bg-primary hover:bg-primary-hover text-white px-4 py-2.5 rounded-lg text-sm font-semibold shadow-sm transition-all active:scale-95">
            <span class="material-symbols-outlined text-[20px]">add</span>
            新增资产
          </button>
          <button class="flex items-center gap-2 bg-white dark:bg-slate-800 border border-border-light dark:border-border-dark text-text-primary-light dark:text-white hover:bg-slate-50 dark:hover:bg-slate-700 px-4 py-2.5 rounded-lg text-sm font-medium transition-colors">
            <span class="material-symbols-outlined text-[20px]">upload_file</span>
            批量导入
          </button>
        </div>
      </div>
      <!-- Table -->
      <div class="relative flex-1 bg-surface-light dark:bg-surface-dark rounded-b-xl border border-border-light dark:border-border-dark shadow-sm overflow-hidden flex flex-col">
        <div class="overflow-auto custom-scrollbar flex-1">
          <table class="w-full text-sm text-left rtl:text-right text-text-secondary-light dark:text-text-secondary-dark whitespace-nowrap">
            <thead class="text-xs text-text-secondary-light uppercase bg-slate-50 dark:bg-slate-800 dark:text-text-secondary-dark sticky top-0 z-10">
              <tr>
                <th class="px-6 py-3 font-semibold" scope="col">资产编号</th>
                <th class="px-6 py-3 font-semibold" scope="col">资产名称</th>
                <th class="px-6 py-3 font-semibold" scope="col">资产类别</th>
                <th class="px-6 py-3 font-semibold" scope="col">存放地点</th>
                <th class="px-6 py-3 font-semibold text-right" scope="col">原值 (CNY)</th>
                <th class="px-6 py-3 font-semibold text-center" scope="col">状态</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-border-light dark:divide-border-dark">
              <tr class="bg-white dark:bg-surface-dark hover:bg-slate-50 dark:hover:bg-slate-800/50 group/row transition-colors">
                <td class="px-6 py-3 font-medium">ZC202310001</td>
                <td class="px-6 py-3 text-primary">MacBook Pro M2 Max</td>
                <td class="px-6 py-3">电子设备</td>
                <td class="px-6 py-3">上海总部-研发中心</td>
                <td class="px-6 py-3 text-right font-mono">24,999.00</td>
                <td class="px-6 py-3 text-center"><span class="bg-green-100 text-green-800 px-2 py-1 rounded text-xs">在用</span></td>
              </tr>
              <!-- More rows... -->
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>
