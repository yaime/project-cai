<template>
  <div class="flex flex-col h-full bg-background-light dark:bg-background-dark font-display text-text-main antialiased overflow-hidden w-full">
    <!-- Top Header -->
    <header class="h-16 bg-surface-light dark:bg-[#1a202c] border-b border-border-light dark:border-border-dark flex items-center justify-between px-6 flex-shrink-0 z-10">
      <div class="flex items-center gap-4">
        <div class="flex flex-col">
          <h1 class="text-lg font-bold text-text-main dark:text-white">银行对账单导入与维护</h1>
          <div class="flex items-center gap-2 text-xs text-text-secondary dark:text-gray-400">
            <span>出纳管理</span>
            <span class="material-symbols-outlined text-[10px]">chevron_right</span>
            <span>银行对账单</span>
          </div>
        </div>
      </div>
      <div class="flex items-center gap-4">
        <label class="relative hidden sm:block">
          <span class="absolute inset-y-0 left-0 flex items-center pl-3 text-text-secondary">
            <span class="material-symbols-outlined text-[20px]">search</span>
          </span>
          <input class="block w-64 rounded-lg border-none bg-background-light dark:bg-gray-800 py-2 pl-10 pr-3 text-sm placeholder:text-text-secondary focus:ring-2 focus:ring-primary/20 dark:text-white" placeholder="全局搜索..." type="text"/>
        </label>
        <button class="relative p-2 text-text-secondary hover:bg-background-light dark:hover:bg-gray-800 rounded-full transition-colors">
          <span class="material-symbols-outlined">notifications</span>
          <span class="absolute top-2 right-2 w-2 h-2 bg-red-500 rounded-full border border-surface-light dark:border-[#1a202c]"></span>
        </button>
        <button class="p-2 text-text-secondary hover:bg-background-light dark:hover:bg-gray-800 rounded-full transition-colors">
          <span class="material-symbols-outlined">help</span>
        </button>
      </div>
    </header>
    <!-- Main Workspace -->
    <main class="flex-1 flex flex-col overflow-hidden p-6 gap-6">
      <!-- Filters & Summary Card -->
      <div class="flex flex-col gap-4 flex-shrink-0">
        <!-- Controls Row -->
        <div class="flex flex-wrap items-center justify-between gap-4">
          <div class="flex items-center gap-3 bg-surface-light dark:bg-[#1a202c] p-1.5 rounded-lg border border-border-light dark:border-border-dark shadow-sm">
            <!-- Account Selector -->
            <div class="relative group">
              <button class="flex items-center gap-2 px-3 py-1.5 rounded-md hover:bg-background-light dark:hover:bg-gray-800 text-text-main dark:text-white text-sm font-medium transition-colors">
                <span class="w-6 h-6 rounded bg-red-50 flex items-center justify-center text-red-600 font-bold text-xs border border-red-100">ICBC</span>
                <span>工商银行 (尾号6222)</span>
                <span class="material-symbols-outlined text-[18px] text-text-secondary">keyboard_arrow_down</span>
              </button>
            </div>
            <div class="w-px h-6 bg-border-color dark:bg-gray-700"></div>
            <!-- Period Selector -->
            <div class="relative">
              <button class="flex items-center gap-2 px-3 py-1.5 rounded-md hover:bg-background-light dark:hover:bg-gray-800 text-text-main dark:text-white text-sm font-medium transition-colors">
                <span class="material-symbols-outlined text-[18px] text-text-secondary">calendar_month</span>
                <span>2023年10月</span>
                <span class="material-symbols-outlined text-[18px] text-text-secondary">keyboard_arrow_down</span>
              </button>
            </div>
          </div>
          <div class="flex items-center gap-3">
            <button class="flex items-center gap-2 px-4 py-2 bg-surface-light dark:bg-[#1a202c] border border-border-light dark:border-border-dark hover:bg-gray-50 dark:hover:bg-gray-800 text-text-main dark:text-white text-sm font-medium rounded-lg transition-colors shadow-sm">
              <span class="material-symbols-outlined text-[18px]">edit_note</span>
              手工录入
            </button>
            <button class="flex items-center gap-2 px-4 py-2 bg-primary text-white rounded-lg text-sm font-medium hover:bg-primary-dark transition-colors shadow-sm shadow-blue-200 dark:shadow-none">
              <span class="material-symbols-outlined text-[18px]">upload_file</span>
              批量导入对账单
            </button>
          </div>
        </div>
        <!-- Stats Cards -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <!-- Stat 1 -->
          <div class="bg-surface-light dark:bg-[#1a202c] p-4 rounded-xl border border-border-color dark:border-border-dark shadow-sm flex flex-col gap-1 relative overflow-hidden group">
            <div class="absolute right-0 top-0 p-3 opacity-10 group-hover:opacity-20 transition-opacity">
              <span class="material-symbols-outlined text-6xl text-primary">account_balance</span>
            </div>
            <span class="text-sm font-medium text-text-secondary">银行日记账余额</span>
            <div class="flex items-baseline gap-2">
              <span class="text-2xl font-bold font-mono text-text-main dark:text-white tracking-tight">¥1,240,500.00</span>
            </div>
            <div class="flex items-center gap-1 text-xs text-green-600 font-medium mt-1">
              <span class="material-symbols-outlined text-[14px]">trending_up</span>
              <span>较上期 +0.00%</span>
            </div>
          </div>
          <!-- Stat 2 -->
          <div class="bg-surface-light dark:bg-[#1a202c] p-4 rounded-xl border border-border-color dark:border-border-dark shadow-sm flex flex-col gap-1 relative overflow-hidden group">
            <div class="absolute right-0 top-0 p-3 opacity-10 group-hover:opacity-20 transition-opacity">
              <span class="material-symbols-outlined text-6xl text-purple-600">receipt</span>
            </div>
            <span class="text-sm font-medium text-text-secondary">银行对账单余额</span>
            <div class="flex items-baseline gap-2">
              <span class="text-2xl font-bold font-mono text-text-main dark:text-white tracking-tight">¥1,245,500.00</span>
            </div>
            <div class="flex items-center gap-1 text-xs text-text-secondary mt-1">
              <span>截止日期: 2023-10-31</span>
            </div>
          </div>
          <!-- Stat 3 (Alert) -->
          <div class="bg-surface-light dark:bg-[#1a202c] p-4 rounded-xl border border-orange-100 dark:border-orange-900/30 shadow-sm flex flex-col gap-1 relative overflow-hidden group">
            <div class="absolute inset-0 bg-orange-50/50 dark:bg-orange-900/10 pointer-events-none"></div>
            <div class="absolute right-0 top-0 p-3 opacity-10 group-hover:opacity-20 transition-opacity z-10">
              <span class="material-symbols-outlined text-6xl text-orange-600">difference</span>
            </div>
            <div class="z-10">
              <div class="flex justify-between items-center">
                <span class="text-sm font-medium text-text-secondary">调节后差额</span>
                <span class="text-xs font-bold text-orange-600 bg-orange-100 dark:bg-orange-900/30 px-2 py-0.5 rounded-full">需关注</span>
              </div>
              <div class="flex items-baseline gap-2 mt-1">
                <span class="text-2xl font-bold font-mono text-orange-600 tracking-tight">-¥5,000.00</span>
              </div>
              <div class="flex items-center gap-1 text-xs text-text-secondary mt-1">
                <a class="text-primary hover:underline" href="#">查看差异明细</a>
              </div>
            </div>
          </div>
        </div>
      </div>
      <!-- Data Table Section -->
      <div class="flex-1 bg-surface-light dark:bg-[#1a202c] border border-border-color dark:border-border-dark rounded-xl shadow-sm flex flex-col min-h-0">
        <!-- Table Toolbar -->
        <div class="p-3 border-b border-border-color dark:border-border-dark flex items-center justify-between gap-4 flex-shrink-0">
          <div class="flex items-center gap-2">
            <div class="relative">
              <span class="absolute inset-y-0 left-0 flex items-center pl-2.5 text-text-secondary">
                <span class="material-symbols-outlined text-[18px]">search</span>
              </span>
              <input class="form-input block w-64 rounded-lg border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 py-1.5 pl-9 pr-3 text-sm focus:border-primary focus:ring-primary focus:bg-white dark:focus:bg-gray-900 transition-all placeholder:text-gray-400 dark:text-white" placeholder="搜索摘要、金额或票据号" type="text"/>
            </div>
            <div class="h-6 w-px bg-border-color dark:bg-gray-700 mx-1"></div>
            <div class="flex items-center gap-2">
              <span class="text-sm text-text-secondary font-medium pl-1">状态:</span>
              <label class="inline-flex items-center cursor-pointer">
                <input checked class="form-checkbox h-4 w-4 rounded border-gray-300 text-primary focus:ring-primary/20" type="checkbox"/>
                <span class="ml-2 text-sm text-text-main dark:text-gray-200">未勾对</span>
              </label>
              <label class="inline-flex items-center cursor-pointer ml-3">
                <input checked class="form-checkbox h-4 w-4 rounded border-gray-300 text-gray-400 focus:ring-gray-400" type="checkbox"/>
                <span class="ml-2 text-sm text-text-secondary">已勾对</span>
              </label>
            </div>
          </div>
          <div class="flex items-center gap-2">
            <button class="p-1.5 text-text-secondary hover:text-primary hover:bg-background-light dark:hover:bg-gray-800 rounded transition-colors" title="刷新">
              <span class="material-symbols-outlined text-[20px]">refresh</span>
            </button>
            <button class="p-1.5 text-text-secondary hover:text-primary hover:bg-background-light dark:hover:bg-gray-800 rounded transition-colors" title="列设置">
              <span class="material-symbols-outlined text-[20px]">view_column</span>
            </button>
            <button class="p-1.5 text-text-secondary hover:text-primary hover:bg-background-light dark:hover:bg-gray-800 rounded transition-colors" title="导出">
              <span class="material-symbols-outlined text-[20px]">download</span>
            </button>
          </div>
        </div>
        <!-- Table Container -->
        <div class="flex-1 overflow-auto custom-scrollbar relative">
          <table class="w-full text-left border-collapse">
            <thead class="bg-gray-50 dark:bg-gray-800 sticky top-0 z-10 ring-1 ring-border-color dark:ring-border-dark ring-opacity-5">
              <tr>
                <th class="p-3 pl-4 w-10">
                  <input class="form-checkbox h-4 w-4 rounded border-gray-300 text-primary focus:ring-primary/20" type="checkbox"/>
                </th>
                <th class="p-3 text-xs font-semibold text-text-secondary uppercase tracking-wider whitespace-nowrap">日期</th>
                <th class="p-3 text-xs font-semibold text-text-secondary uppercase tracking-wider whitespace-nowrap">结算方式</th>
                <th class="p-3 text-xs font-semibold text-text-secondary uppercase tracking-wider whitespace-nowrap">票据号</th>
                <th class="p-3 text-xs font-semibold text-text-secondary uppercase tracking-wider whitespace-nowrap min-w-[200px]">摘要</th>
                <th class="p-3 text-xs font-semibold text-text-secondary uppercase tracking-wider whitespace-nowrap text-right">借方 (收入)</th>
                <th class="p-3 text-xs font-semibold text-text-secondary uppercase tracking-wider whitespace-nowrap text-right">贷方 (支出)</th>
                <th class="p-3 text-xs font-semibold text-text-secondary uppercase tracking-wider whitespace-nowrap text-right">余额</th>
                <th class="p-3 text-xs font-semibold text-text-secondary uppercase tracking-wider whitespace-nowrap text-center">状态</th>
                <th class="p-3 pr-4 text-xs font-semibold text-text-secondary uppercase tracking-wider whitespace-nowrap text-right">操作</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-border-color dark:divide-border-dark bg-surface-light dark:bg-[#1a202c]">
              <!-- Row 1 -->
              <tr class="hover:bg-primary/5 transition-colors group">
                <td class="p-3 pl-4">
                  <input class="form-checkbox h-4 w-4 rounded border-gray-300 text-primary focus:ring-primary/20" type="checkbox"/>
                </td>
                <td class="p-3 text-sm text-text-main dark:text-white font-mono whitespace-nowrap">2023-10-12</td>
                <td class="p-3 text-sm text-text-main dark:text-white whitespace-nowrap">
                  <span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-blue-50 text-blue-700">电汇</span>
                </td>
                <td class="p-3 text-sm text-text-secondary font-mono whitespace-nowrap">T20231012001</td>
                <td class="p-3 text-sm text-text-main dark:text-white truncate max-w-[240px]" title="收 上海科技有限公司 货款">收 上海科技有限公司 货款</td>
                <td class="p-3 text-sm font-medium font-mono text-green-700 text-right whitespace-nowrap">50,000.00</td>
                <td class="p-3 text-sm font-medium font-mono text-text-secondary text-right whitespace-nowrap">-</td>
                <td class="p-3 text-sm font-medium font-mono text-text-main dark:text-white text-right whitespace-nowrap">1,250,000.00</td>
                <td class="p-3 text-center whitespace-nowrap">
                  <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-medium bg-green-50 text-green-700 border border-green-100">
                    <span class="material-symbols-outlined text-[14px]">check_circle</span>
                    已勾对
                  </span>
                </td>
                <td class="p-3 pr-4 text-right whitespace-nowrap">
                  <button class="text-text-secondary hover:text-primary transition-colors text-sm font-medium">详情</button>
                </td>
              </tr>
              <!-- Row 3 - Unreconciled -->
              <tr class="hover:bg-primary/5 transition-colors group bg-red-50/30 dark:bg-red-900/10">
                <td class="p-3 pl-4">
                  <input class="form-checkbox h-4 w-4 rounded border-gray-300 text-primary focus:ring-primary/20" type="checkbox"/>
                </td>
                <td class="p-3 text-sm text-text-main dark:text-white font-mono whitespace-nowrap">2023-10-14</td>
                <td class="p-3 text-sm text-text-main dark:text-white whitespace-nowrap">
                  <span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-gray-100 text-gray-700">手续费</span>
                </td>
                <td class="p-3 text-sm text-text-secondary font-mono whitespace-nowrap">-</td>
                <td class="p-3 text-sm text-text-main dark:text-white truncate max-w-[240px]" title="季度账户管理费">季度账户管理费</td>
                <td class="p-3 text-sm font-medium font-mono text-text-secondary text-right whitespace-nowrap">-</td>
                <td class="p-3 text-sm font-medium font-mono text-text-main dark:text-white text-right whitespace-nowrap">5,000.00</td>
                <td class="p-3 text-sm font-medium font-mono text-text-main dark:text-white text-right whitespace-nowrap">1,242,500.00</td>
                <td class="p-3 text-center whitespace-nowrap">
                  <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-medium bg-white dark:bg-gray-700 text-orange-600 border border-orange-200 shadow-sm">
                    <span class="material-symbols-outlined text-[14px]">error</span>
                    未勾对
                  </span>
                </td>
                <td class="p-3 pr-4 text-right whitespace-nowrap">
                  <div class="flex items-center justify-end gap-2">
                    <button class="text-primary hover:text-primary-dark transition-colors text-sm font-medium">手动勾对</button>
                    <button class="text-text-secondary hover:text-text-main dark:hover:text-white transition-colors text-sm">
                      <span class="material-symbols-outlined text-[18px]">more_vert</span>
                    </button>
                  </div>
                </td>
              </tr>
            </tbody>
            <!-- Footer Totals -->
            <tfoot class="bg-gray-50 dark:bg-gray-800 border-t border-border-color dark:border-border-dark sticky bottom-0 z-10 shadow-[0_-4px_12px_rgba(0,0,0,0.03)]">
              <tr>
                <td class="p-3 pl-4 text-sm font-bold text-text-main dark:text-white text-right" colspan="5">本页合计</td>
                <td class="p-3 text-sm font-bold font-mono text-green-700 text-right">70,000.00</td>
                <td class="p-3 text-sm font-bold font-mono text-text-main dark:text-white text-right">132,500.00</td>
                <td colspan="3"></td>
              </tr>
            </tfoot>
          </table>
        </div>
        <!-- Pagination -->
        <div class="p-3 border-t border-border-color dark:border-border-dark flex items-center justify-between flex-shrink-0">
          <div class="text-sm text-text-secondary">
            显示第 <span class="font-medium text-text-main dark:text-white">1</span> 到 <span class="font-medium text-text-main dark:text-white">10</span> 条，共 <span class="font-medium text-text-main dark:text-white">128</span> 条记录
          </div>
          <div class="flex items-center gap-2">
            <select class="form-select py-1 pl-2 pr-8 text-sm border-gray-300 dark:border-gray-700 dark:bg-gray-800 dark:text-white rounded focus:ring-primary/20 focus:border-primary">
              <option>20 条/页</option>
              <option>50 条/页</option>
              <option>100 条/页</option>
            </select>
            <!-- Pagination Nav -->
          </div>
        </div>
      </div>
    </main>
  </div>
</template>
