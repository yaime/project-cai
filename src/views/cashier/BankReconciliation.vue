<template>
  <div class="flex flex-col h-full bg-background-light dark:bg-background-dark font-display text-text-main overflow-hidden w-full">
    <!-- Toolbar & Filters -->
    <div class="flex-none bg-white dark:bg-[#1a202c] border-b border-border-light dark:border-border-dark px-6 py-4 z-10">
      <div class="flex flex-wrap items-center justify-between gap-4">
        <div class="flex items-center gap-4">
          <div class="relative min-w-[240px]">
            <label class="absolute -top-2 left-2 px-1 bg-white dark:bg-[#1a202c] text-xs font-medium text-text-secondary">当前账户</label>
            <div class="flex items-center w-full h-10 px-3 rounded-lg border border-border-light dark:border-gray-700 bg-background-light/50 dark:bg-gray-800 text-sm font-medium dark:text-white">
              <span class="truncate">招商银行 (尾号 7890) - 人民币基本户</span>
              <span class="material-symbols-outlined ml-auto text-text-secondary text-[18px]">expand_more</span>
            </div>
          </div>
          <div class="flex items-center bg-background-light dark:bg-gray-800 rounded-lg border border-border-light dark:border-gray-700 h-10 p-1">
            <button class="px-2 h-full flex items-center justify-center text-text-secondary hover:text-primary transition-colors">
              <span class="material-symbols-outlined text-[18px]">chevron_left</span>
            </button>
            <div class="px-2 text-sm font-medium text-text-main dark:text-white border-x border-gray-200 dark:border-gray-600 h-full flex items-center">2023年10月</div>
            <button class="px-2 h-full flex items-center justify-center text-text-secondary hover:text-primary transition-colors">
              <span class="material-symbols-outlined text-[18px]">chevron_right</span>
            </button>
          </div>
          <label class="flex items-center gap-2 cursor-pointer select-none">
            <div class="relative inline-flex items-center cursor-pointer">
              <input checked class="sr-only peer" type="checkbox" />
              <div class="w-9 h-5 bg-gray-200 peer-focus:outline-none rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-primary"></div>
            </div>
            <span class="text-sm font-medium text-text-secondary dark:text-gray-400">仅显示未勾对</span>
          </label>
        </div>
        <div class="flex items-center gap-3">
          <button class="flex items-center gap-2 px-5 h-10 bg-primary hover:bg-primary-hover text-white text-sm font-bold rounded-lg shadow-md transition-all active:scale-95">
            <span class="material-symbols-outlined text-[20px]">sync_alt</span>
            自动勾对
          </button>
          <button class="flex items-center gap-2 px-4 h-10 bg-white dark:bg-gray-800 border border-border-light dark:border-gray-700 hover:bg-background-light dark:hover:bg-gray-700 text-text-main dark:text-white text-sm font-medium rounded-lg transition-colors">
            <span class="material-symbols-outlined text-[20px]">link</span>
            手动匹配
          </button>
        </div>
        <div class="flex items-center gap-3">
          <button class="flex items-center gap-2 px-4 h-10 bg-white dark:bg-gray-800 border border-border-light dark:border-gray-700 hover:border-primary/50 hover:text-primary text-text-main dark:text-white text-sm font-medium rounded-lg transition-all group">
            <span class="material-symbols-outlined text-[20px] text-text-secondary group-hover:text-primary transition-colors">table_view</span>
            生成余额调节表
          </button>
        </div>
      </div>
    </div>
    <!-- Main Content: Split View -->
    <main class="flex-1 flex overflow-hidden relative">
      <!-- Left Panel: Bank Journal -->
      <div class="w-1/2 flex flex-col border-r border-border-light dark:border-border-dark bg-white dark:bg-[#1a202c] h-full">
        <div class="flex-none px-4 py-3 border-b border-border-light dark:border-border-dark bg-gray-50/50 dark:bg-gray-800/30 flex justify-between items-center">
          <div class="flex items-center gap-2">
            <div class="size-2 rounded-full bg-blue-500"></div>
            <h3 class="font-bold text-text-main dark:text-white">银行日记账 (企业内部)</h3>
            <span class="bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-300 px-2 py-0.5 rounded text-xs font-medium border border-gray-200 dark:border-gray-600">124 笔</span>
          </div>
          <div class="flex gap-2 text-xs">
            <span class="text-text-secondary">借: <span class="font-mono font-medium text-text-main dark:text-white">24</span></span>
            <span class="text-text-secondary">贷: <span class="font-mono font-medium text-text-main dark:text-white">100</span></span>
          </div>
        </div>
        <!-- Table Header -->
        <div class="flex-none grid grid-cols-[40px_90px_100px_1fr_100px_100px] gap-2 px-4 py-2 bg-background-light dark:bg-gray-800 border-b border-border-light dark:border-border-dark text-xs font-medium text-text-secondary uppercase tracking-wider">
          <div class="flex items-center justify-center"><input class="rounded border-gray-300 text-primary focus:ring-primary size-4 cursor-pointer custom-checkbox" type="checkbox"/></div>
          <div class="flex items-center">日期</div>
          <div class="flex items-center">凭证号</div>
          <div class="flex items-center">摘要</div>
          <div class="flex items-center justify-end">借方</div>
          <div class="flex items-center justify-end">贷方</div>
        </div>
        <!-- Table Body -->
        <div class="flex-1 overflow-y-auto custom-scrollbar p-0">
          <div class="grid grid-cols-[40px_90px_100px_1fr_100px_100px] gap-2 px-4 py-3 border-b border-border-light dark:border-border-dark text-sm text-text-main dark:text-white items-center table-row-hover cursor-pointer group transition-colors">
            <div class="flex items-center justify-center"><input class="rounded border-gray-300 text-primary focus:ring-primary size-4 cursor-pointer custom-checkbox" type="checkbox"/></div>
            <div class="font-mono text-gray-600 dark:text-gray-400">2023-10-01</div>
            <div class="font-mono text-blue-600 hover:underline cursor-pointer">PZ-2023001</div>
            <div class="truncate text-gray-700 dark:text-gray-300" title="收到货款 A公司">收到货款 A公司</div>
            <div class="font-mono text-right font-medium">12,500.00</div>
            <div class="font-mono text-right text-gray-400">-</div>
          </div>
          <!-- More rows... -->
        </div>
        <!-- Left Footer -->
        <div class="flex-none p-3 border-t border-border-light dark:border-border-dark bg-gray-50 dark:bg-gray-800 flex justify-between items-center text-sm">
          <span class="text-text-secondary">选中: <span class="font-bold text-text-main dark:text-white">1</span> 笔</span>
          <div class="flex gap-4">
            <span class="text-text-secondary">选中金额: <span class="font-mono font-bold text-text-main dark:text-white">-2,000.00</span></span>
          </div>
        </div>
      </div>
      <!-- Right Panel: Bank Statement -->
      <div class="w-1/2 flex flex-col bg-white dark:bg-[#1a202c] h-full relative">
        <div class="flex-none px-4 py-3 border-b border-border-light dark:border-border-dark bg-gray-50/50 dark:bg-gray-800/30 flex justify-between items-center">
          <div class="flex items-center gap-2">
            <div class="size-2 rounded-full bg-orange-500"></div>
            <h3 class="font-bold text-text-main dark:text-white">银行对账单 (外部流水)</h3>
            <button class="text-primary text-xs font-medium hover:underline flex items-center gap-0.5 ml-2">
              <span class="material-symbols-outlined text-[14px]">add</span>
              导入
            </button>
          </div>
          <div class="flex gap-2 text-xs">
            <span class="text-text-secondary">存: <span class="font-mono font-medium text-text-main dark:text-white">30</span></span>
            <span class="text-text-secondary">支: <span class="font-mono font-medium text-text-main dark:text-white">90</span></span>
          </div>
        </div>
        <!-- Table Header -->
        <div class="flex-none grid grid-cols-[40px_90px_1fr_100px_100px_100px] gap-2 px-4 py-2 bg-background-light dark:bg-gray-800 border-b border-border-light dark:border-border-dark text-xs font-medium text-text-secondary uppercase tracking-wider">
          <div class="flex items-center justify-center"><input class="rounded border-gray-300 text-primary focus:ring-primary size-4 cursor-pointer custom-checkbox" type="checkbox"/></div>
          <div class="flex items-center">交易日期</div>
          <div class="flex items-center">对方户名/摘要</div>
          <div class="flex items-center justify-end">收入金额</div>
          <div class="flex items-center justify-end">支出金额</div>
          <div class="flex items-center justify-end">余额</div>
        </div>
        <!-- Table Body -->
        <div class="flex-1 overflow-y-auto custom-scrollbar p-0">
          <div class="grid grid-cols-[40px_90px_1fr_100px_100px_100px] gap-2 px-4 py-3 border-b border-border-light dark:border-border-dark text-sm text-text-main dark:text-white items-center table-row-hover cursor-pointer group transition-colors">
            <div class="flex items-center justify-center"><input class="rounded border-gray-300 text-primary focus:ring-primary size-4 cursor-pointer custom-checkbox" type="checkbox"/></div>
            <div class="font-mono text-gray-600 dark:text-gray-400">2023-10-01</div>
            <div class="flex flex-col">
              <span class="truncate font-medium text-text-main dark:text-white">深圳市A科技有限公司</span>
              <span class="text-xs text-text-secondary truncate">货款转账</span>
            </div>
            <div class="font-mono text-right font-medium text-green-600">12,500.00</div>
            <div class="font-mono text-right text-gray-400">-</div>
            <div class="font-mono text-right text-text-secondary">1,212,500.00</div>
          </div>
          <!-- More rows -->
        </div>
        <!-- Right Footer -->
        <div class="flex-none p-3 border-t border-border-light dark:border-border-dark bg-gray-50 dark:bg-gray-800 flex justify-between items-center text-sm">
          <span class="text-text-secondary">选中: <span class="font-bold text-text-main dark:text-white">1</span> 笔</span>
          <div class="flex gap-4">
            <span class="text-text-secondary">选中金额: <span class="font-mono font-bold text-text-main dark:text-white">-2,000.00</span></span>
          </div>
        </div>
      </div>
    </main>
    <!-- Global Footer -->
    <footer class="flex-none bg-white dark:bg-[#1a202c] border-t border-border-light dark:border-border-dark px-8 py-3 shadow-2xl z-20">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-12">
          <div class="flex flex-col gap-0.5">
            <span class="text-xs text-text-secondary font-medium">企业日记账余额</span>
            <span class="text-lg font-mono font-bold text-text-main dark:text-white tracking-tight">¥ 1,166,855.00</span>
          </div>
          <div class="flex flex-col gap-0.5">
            <span class="text-xs text-text-secondary font-medium">银行对账单余额</span>
            <span class="text-lg font-mono font-bold text-text-main dark:text-white tracking-tight">¥ 1,166,855.00</span>
          </div>
        </div>
        <div class="flex items-center bg-green-50 dark:bg-green-900/20 px-6 py-2 rounded-lg border border-green-200 dark:border-green-800">
          <div class="flex flex-col items-end mr-4">
            <span class="text-xs text-green-700 dark:text-green-300 font-bold uppercase tracking-wider">对账平衡</span>
            <span class="text-xs text-green-600/80 dark:text-green-400/80">无需生成调节表</span>
          </div>
          <div class="h-8 w-px bg-green-200 dark:bg-green-800 mx-4"></div>
          <div class="flex flex-col items-end">
            <span class="text-xs text-text-secondary mb-0.5">对账差额</span>
            <span class="text-xl font-mono font-bold text-green-600 dark:text-green-400">0.00</span>
          </div>
          <div class="ml-4 flex items-center justify-center size-8 rounded-full bg-green-100 dark:bg-green-900 text-green-600 dark:text-green-300">
            <span class="material-symbols-outlined">check</span>
          </div>
        </div>
        <div class="flex items-center">
          <button class="text-primary hover:text-primary-hover text-sm font-bold flex items-center gap-1">
            查看对账报告
            <span class="material-symbols-outlined text-[16px]">arrow_forward</span>
          </button>
        </div>
      </div>
    </footer>
  </div>
</template>
