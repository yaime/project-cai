<template>
  <div class="flex flex-col h-full bg-background-light dark:bg-background-dark p-6 pb-24 overflow-y-auto">
    <div class="max-w-[1200px] mx-auto flex flex-col gap-6">
      <div class="flex flex-col md:flex-row md:items-end justify-between gap-4">
        <div>
          <h1 class="text-2xl font-bold text-slate-900 dark:text-white tracking-tight">固定资产计提折旧</h1>
          <p class="text-slate-500 dark:text-slate-400 mt-1 text-sm">
            请确认本期折旧数据准确无误后，执行计提并生成凭证。
          </p>
        </div>
        <div class="flex items-center gap-2 bg-white dark:bg-[#1a202c] border border-slate-200 dark:border-slate-700 rounded-lg px-4 py-2 shadow-sm">
          <span class="material-symbols-outlined text-primary">calendar_month</span>
          <div class="flex flex-col">
            <span class="text-[10px] uppercase text-slate-400 font-bold tracking-wider">当前计提期间</span>
            <span class="text-sm font-semibold text-slate-900 dark:text-white tabular-nums">2023年10月</span>
          </div>
        </div>
      </div>
      <!-- Stats -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div class="bg-white dark:bg-[#1a202c] p-5 rounded-xl border border-slate-200 dark:border-slate-700 shadow-sm flex flex-col justify-between h-32">
          <p class="text-slate-500 dark:text-slate-400 text-sm font-medium">计提折旧总额</p>
          <div class="flex items-baseline gap-2">
            <span class="text-lg text-slate-400 font-medium">¥</span>
            <p class="text-3xl font-bold text-primary tabular-nums">452,100.00</p>
          </div>
        </div>
      </div>
      <!-- Table -->
      <div class="bg-white dark:bg-[#1a202c] rounded-xl border border-slate-200 dark:border-slate-700 shadow-sm flex flex-col min-h-[400px]">
        <div class="overflow-x-auto grow">
          <table class="w-full text-sm text-left">
            <thead class="bg-slate-50 dark:bg-slate-800/50 text-slate-500 dark:text-slate-400 font-medium border-b border-slate-200 dark:border-slate-700">
              <tr>
                <th class="px-6 py-3 whitespace-nowrap">资产编号</th>
                <th class="px-6 py-3 whitespace-nowrap">资产名称</th>
                <th class="px-6 py-3 whitespace-nowrap">资产类别</th>
                <th class="px-6 py-3 whitespace-nowrap">使用部门</th>
                <th class="px-6 py-3 text-right whitespace-nowrap">本期折旧额</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-slate-100 dark:divide-slate-800">
              <tr class="hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors">
                <td class="px-6 py-3 font-medium text-slate-900 dark:text-white tabular-nums">FA-2023-001</td>
                <td class="px-6 py-3 text-slate-700 dark:text-slate-300">Dell PowerEdge 服务器</td>
                <td class="px-6 py-3 text-slate-500">电子设备</td>
                <td class="px-6 py-3">研发部</td>
                <td class="px-6 py-3 text-right font-medium text-slate-900 dark:text-white tabular-nums">1,200.00</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>
