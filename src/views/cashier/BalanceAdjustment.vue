<template>
  <div class="flex flex-col h-full bg-background-light dark:bg-background-dark p-4 md:p-8 overflow-y-auto">
    <div class="max-w-[1100px] mx-auto w-full flex flex-col gap-6">
      <!-- Breadcrumbs -->
      <nav class="flex items-center text-sm font-medium text-[#4c669a] dark:text-gray-400">
        <a class="hover:text-primary transition-colors" href="#">出纳管理</a>
        <span class="mx-2 text-gray-300">/</span>
        <a class="hover:text-primary transition-colors" href="#">报表中心</a>
        <span class="mx-2 text-gray-300">/</span>
        <span class="text-[#0d121b] dark:text-white">银行存款余额调节表</span>
      </nav>
      <!-- Page Header & Actions -->
      <div class="flex flex-col md:flex-row md:items-end justify-between gap-4">
        <div>
          <h1 class="text-2xl md:text-3xl font-bold text-[#0d121b] dark:text-white tracking-tight">银行存款余额调节表</h1>
          <p class="mt-1 text-sm text-[#4c669a] dark:text-gray-400">生成并打印银行存款与企业账面的余额调节报告</p>
        </div>
        <div class="flex gap-3">
          <button class="flex items-center gap-2 px-4 py-2 text-sm font-medium text-[#0d121b] dark:text-white bg-white dark:bg-gray-800 border border-[#cfd7e7] dark:border-gray-700 rounded-lg shadow-sm hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">
            <span class="material-symbols-outlined text-[18px]">download</span>
            导出 Excel
          </button>
          <button class="flex items-center gap-2 px-4 py-2 text-sm font-medium text-[#0d121b] dark:text-white bg-white dark:bg-gray-800 border border-[#cfd7e7] dark:border-gray-700 rounded-lg shadow-sm hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors" onclick="window.print()">
            <span class="material-symbols-outlined text-[18px]">print</span>
            打印
          </button>
        </div>
      </div>
      <!-- Filter Bar -->
      <div class="bg-white dark:bg-[#1a202c] p-4 rounded-xl border border-[#e7ebf3] dark:border-gray-800 shadow-sm flex flex-wrap items-end gap-4">
        <!-- Account, Date, Currency inputs... -->
        <button class="flex items-center justify-center gap-2 px-6 py-2.5 text-sm font-medium text-white bg-primary hover:bg-blue-600 rounded-lg shadow-sm transition-colors mt-auto min-w-[100px]">
          <span class="material-symbols-outlined text-[20px]">search</span>
          查询
        </button>
      </div>
      <!-- Report Paper -->
      <div class="bg-white dark:bg-[#1a202c] rounded-xl shadow-md border border-[#e7ebf3] dark:border-gray-800 p-8 md:p-12 overflow-x-auto">
        <div class="flex flex-col items-center mb-8 gap-2">
          <h2 class="text-2xl md:text-3xl font-bold text-[#0d121b] dark:text-white tracking-wide">银行存款余额调节表</h2>
          <div class="flex flex-wrap justify-center gap-x-8 gap-y-2 mt-4 text-sm text-[#4c669a] dark:text-gray-400 font-medium w-full max-w-4xl border-b pb-4 border-gray-100 dark:border-gray-700">
            <div class="flex gap-2">
              <span class="text-gray-500">单位名称:</span>
              <span class="text-[#0d121b] dark:text-gray-200">未来科技集团有限公司</span>
            </div>
            <div class="flex gap-2">
              <span class="text-gray-500">银行账号:</span>
              <span class="text-[#0d121b] dark:text-gray-200 font-mono">6222 0210 0109 8829</span>
            </div>
            <div class="flex gap-2">
              <span class="text-gray-500">日期:</span>
              <span class="text-[#0d121b] dark:text-gray-200 font-mono">2023-10-31</span>
            </div>
            <div class="flex gap-2">
              <span class="text-gray-500">单位:</span>
              <span class="text-[#0d121b] dark:text-gray-200">元</span>
            </div>
          </div>
        </div>
        <!-- Report Table -->
        <div class="grid grid-cols-1 md:grid-cols-2 border-2 border-[#0d121b] dark:border-gray-500 text-sm">
          <!-- Left Column: Enterprise -->
          <div class="flex flex-col border-b md:border-b-0 md:border-r border-[#0d121b] dark:border-gray-500">
            <div class="bg-gray-50 dark:bg-gray-800 px-4 py-3 border-b border-[#0d121b] dark:border-gray-500 font-bold text-[#0d121b] dark:text-white flex justify-between items-center">
              <span>项目 (企业账面)</span>
              <span>金额</span>
            </div>
            <div class="flex flex-col flex-1">
              <div class="flex justify-between px-4 py-3 border-b border-gray-200 dark:border-gray-700">
                <span class="font-bold text-[#0d121b] dark:text-gray-200">企业银行存款日记账余额</span>
                <span class="font-bold font-mono text-[#0d121b] dark:text-gray-200">1,250,000.00</span>
              </div>
              <div class="flex flex-col">
                <div class="px-4 py-2 text-[#4c669a] dark:text-gray-400 text-xs font-semibold bg-gray-50/50 dark:bg-gray-800/30">加：银行已收企业未收</div>
                <div class="flex justify-between px-4 py-2 border-b border-dashed border-gray-200 dark:border-gray-800">
                  <span class="text-gray-600 dark:text-gray-400 pl-4">10月30日 收到货款 (客户A)</span>
                  <span class="font-mono text-[#0d121b] dark:text-gray-300">50,000.00</span>
                </div>
              </div>
              <div class="flex-1 min-h-[50px]"></div>
              <div class="bg-primary/5 dark:bg-primary/20 px-4 py-4 border-t border-[#0d121b] dark:border-gray-500 flex justify-between items-center mt-auto">
                <span class="font-bold text-primary dark:text-blue-400">调节后余额</span>
                <span class="font-bold font-mono text-lg text-[#0d121b] dark:text-white">1,299,800.00</span>
              </div>
            </div>
          </div>
          <!-- Right Column: Bank -->
          <div class="flex flex-col">
            <div class="bg-gray-50 dark:bg-gray-800 px-4 py-3 border-b border-[#0d121b] dark:border-gray-500 font-bold text-[#0d121b] dark:text-white flex justify-between items-center">
              <span>项目 (银行对账单)</span>
              <span>金额</span>
            </div>
            <div class="flex flex-col flex-1">
              <div class="flex justify-between px-4 py-3 border-b border-gray-200 dark:border-gray-700">
                <span class="font-bold text-[#0d121b] dark:text-gray-200">银行对账单余额</span>
                <span class="font-bold font-mono text-[#0d121b] dark:text-gray-200">1,249,800.00</span>
              </div>
              <div class="flex flex-col">
                <div class="px-4 py-2 text-[#4c669a] dark:text-gray-400 text-xs font-semibold bg-gray-50/50 dark:bg-gray-800/30">加：企业已收银行未收</div>
                <div class="flex justify-between px-4 py-2 border-b border-dashed border-gray-200 dark:border-gray-800">
                  <span class="text-gray-600 dark:text-gray-400 pl-4">10月30日 现金送存 (未达账)</span>
                  <span class="font-mono text-[#0d121b] dark:text-gray-300">60,000.00</span>
                </div>
              </div>
              <div class="flex-1 min-h-[50px]"></div>
              <div class="bg-primary/5 dark:bg-primary/20 px-4 py-4 border-t border-[#0d121b] dark:border-gray-500 flex justify-between items-center mt-auto">
                <span class="font-bold text-primary dark:text-blue-400">调节后余额</span>
                <span class="font-bold font-mono text-lg text-[#0d121b] dark:text-white">1,299,800.00</span>
              </div>
            </div>
          </div>
        </div>
        <!-- Balance Status -->
        <div class="mt-6 flex justify-center">
          <div class="inline-flex items-center gap-2 px-6 py-2 rounded-full bg-emerald-50 text-emerald-700 border border-emerald-200 dark:bg-emerald-900/30 dark:text-emerald-400 dark:border-emerald-800">
            <span class="material-symbols-outlined text-[24px]">check_circle</span>
            <span class="font-bold text-base tracking-wide">账目平衡 Verified Balanced</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
