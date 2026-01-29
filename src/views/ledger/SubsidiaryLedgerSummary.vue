<template>
  <div class="flex flex-col h-full bg-background-light dark:bg-background-dark overflow-hidden">
    <!-- Breadcrumbs -->
    <div class="flex items-center gap-2 px-6 py-4 text-sm shrink-0">
      <a class="text-gray-500 hover:text-primary" href="#">首页</a>
      <span class="text-gray-400">/</span>
      <a class="text-gray-500 hover:text-primary" href="#">账簿查询</a>
      <span class="text-gray-400">/</span>
      <span class="text-gray-900 dark:text-white font-medium">辅助核算总账汇总</span>
    </div>

    <!-- Page Header & Filter -->
    <div class="px-6 pb-4 shrink-0">
      <div class="flex flex-col gap-4 bg-white dark:bg-[#1a2235] p-5 rounded-xl border border-[#e7ebf3] dark:border-[#2d3a54] shadow-sm">
        <div class="flex justify-between items-center">
          <h1 class="text-xl font-bold text-gray-900 dark:text-white">辅助核算总账汇总表</h1>
          <div class="flex items-center gap-2">
            <button class="flex items-center gap-1 bg-[#e7ebf3] dark:bg-[#2d3a54] px-3 py-1.5 rounded-lg text-sm font-medium hover:bg-gray-200 transition-colors text-gray-700 dark:text-gray-200">
              <span class="material-symbols-outlined text-base">download</span>
              导出 Excel
            </button>
            <button class="flex items-center gap-1 bg-[#e7ebf3] dark:bg-[#2d3a54] px-3 py-1.5 rounded-lg text-sm font-medium hover:bg-gray-200 transition-colors text-gray-700 dark:text-gray-200">
              <span class="material-symbols-outlined text-base">print</span>
              打印
            </button>
          </div>
        </div>
        <div class="flex flex-wrap items-center gap-6 pt-2 border-t border-[#e7ebf3] dark:border-[#2d3a54]">
          <div class="flex items-center gap-3">
            <span class="text-sm text-gray-500">会计期间:</span>
            <div class="flex items-center gap-2">
              <select class="bg-background-light dark:bg-[#2d3a54] border-none rounded-lg text-sm py-1.5 focus:ring-primary text-gray-700 dark:text-white">
                <option>2023-01</option>
                <option selected>2023-10</option>
              </select>
              <span class="text-gray-400">至</span>
              <select class="bg-background-light dark:bg-[#2d3a54] border-none rounded-lg text-sm py-1.5 focus:ring-primary text-gray-700 dark:text-white">
                <option selected>2023-12</option>
              </select>
            </div>
          </div>
          <div class="flex items-center gap-3">
            <span class="text-sm text-gray-500">汇总维度:</span>
            <div class="flex bg-background-light dark:bg-[#2d3a54] p-1 rounded-lg">
              <label class="flex items-center px-4 py-1 rounded-md text-sm font-medium cursor-pointer bg-white dark:bg-[#1a2235] shadow-sm text-primary">
                <input type="radio" name="dimension" class="hidden" checked>
                按部门汇总
              </label>
              <label class="flex items-center px-4 py-1 rounded-md text-sm font-medium cursor-pointer text-gray-500 hover:text-gray-700 dark:text-gray-400">
                <input type="radio" name="dimension" class="hidden">
                按项目汇总
              </label>
              <label class="flex items-center px-4 py-1 rounded-md text-sm font-medium cursor-pointer text-gray-500 hover:text-gray-700 dark:text-gray-400">
                <input type="radio" name="dimension" class="hidden">
                按员工汇总
              </label>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Main Data Grid Area -->
    <div class="flex-1 overflow-auto px-6 pb-6 custom-scrollbar">
      <div class="bg-white dark:bg-[#1a2235] border border-[#e7ebf3] dark:border-[#2d3a54] rounded-xl overflow-hidden shadow-sm">
        <table class="w-full text-left border-collapse min-w-[1000px]">
          <thead class="bg-[#f8f9fc] dark:bg-[#1e293b] sticky top-0 z-10 border-b border-[#e7ebf3] dark:border-[#2d3a54]">
            <tr>
              <th class="px-6 py-3.5 text-xs font-bold text-gray-500 uppercase tracking-wider w-[350px]">科目 / 核算项目</th>
              <th class="px-6 py-3.5 text-xs font-bold text-gray-500 uppercase tracking-wider text-right">期初余额</th>
              <th class="px-6 py-3.5 text-xs font-bold text-gray-500 uppercase tracking-wider text-right">本期借方</th>
              <th class="px-6 py-3.5 text-xs font-bold text-gray-500 uppercase tracking-wider text-right">本期贷方</th>
              <th class="px-6 py-3.5 text-xs font-bold text-gray-500 uppercase tracking-wider text-right">期末余额</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-100 dark:divide-[#2d3a54]">
            <!-- Row 1: Parent Subject -->
            <tr class="bg-[#fdfdfe] dark:bg-[#20293a] font-semibold text-gray-900 dark:text-white">
              <td class="px-6 py-4 flex items-center gap-2">
                <span class="material-symbols-outlined text-primary text-lg cursor-pointer">expand_more</span>
                <span>6602 管理费用</span>
              </td>
              <td class="px-6 py-4 text-right font-mono text-base">45,000.00</td>
              <td class="px-6 py-4 text-right font-mono text-base">125,400.00</td>
              <td class="px-6 py-4 text-right font-mono text-base">0.00</td>
              <td class="px-6 py-4 text-right font-mono text-base text-primary">170,400.00</td>
            </tr>
            <!-- Sub-rows -->
            <tr class="hover:bg-gray-50 dark:hover:bg-[#2d3a54] transition-colors">
              <td class="px-6 py-3 pl-10 text-gray-600 dark:text-gray-300 flex items-center gap-2">
                <span class="w-1.5 h-1.5 rounded-full bg-gray-300"></span>
                <span>人力资源部</span>
              </td>
              <td class="px-6 py-3 text-right font-mono text-sm">12,000.00</td>
              <td class="px-6 py-3 text-right font-mono text-sm">34,500.00</td>
              <td class="px-6 py-3 text-right font-mono text-sm">0.00</td>
              <td class="px-6 py-3 text-right font-mono text-sm">46,500.00</td>
            </tr>
            <tr class="hover:bg-gray-50 dark:hover:bg-[#2d3a54] transition-colors">
              <td class="px-6 py-3 pl-10 text-gray-600 dark:text-gray-300 flex items-center gap-2">
                <span class="w-1.5 h-1.5 rounded-full bg-gray-300"></span>
                <span>研发中心</span>
              </td>
              <td class="px-6 py-3 text-right font-mono text-sm">33,000.00</td>
              <td class="px-6 py-3 text-right font-mono text-sm">90,900.00</td>
              <td class="px-6 py-3 text-right font-mono text-sm">0.00</td>
              <td class="px-6 py-3 text-right font-mono text-sm">123,900.00</td>
            </tr>
            <!-- Row 2 -->
            <tr class="bg-[#fdfdfe] dark:bg-[#20293a] font-semibold text-gray-900 dark:text-white border-t-2 border-gray-100 dark:border-[#2d3a54]">
              <td class="px-6 py-4 flex items-center gap-2">
                <span class="material-symbols-outlined text-primary text-lg cursor-pointer">expand_more</span>
                <span>6601 销售费用</span>
              </td>
              <td class="px-6 py-4 text-right font-mono text-base">82,150.00</td>
              <td class="px-6 py-4 text-right font-mono text-base">210,000.00</td>
              <td class="px-6 py-4 text-right font-mono text-base">0.00</td>
              <td class="px-6 py-4 text-right font-mono text-base text-primary">292,150.00</td>
            </tr>
             <!-- Sub-rows -->
            <tr class="hover:bg-gray-50 dark:hover:bg-[#2d3a54] transition-colors">
              <td class="px-6 py-3 pl-10 text-gray-600 dark:text-gray-300 flex items-center gap-2">
                <span class="w-1.5 h-1.5 rounded-full bg-gray-300"></span>
                <span>市场部</span>
              </td>
              <td class="px-6 py-3 text-right font-mono text-sm">56,000.00</td>
              <td class="px-6 py-3 text-right font-mono text-sm">150,000.00</td>
              <td class="px-6 py-3 text-right font-mono text-sm">0.00</td>
              <td class="px-6 py-3 text-right font-mono text-sm">206,000.00</td>
            </tr>
          </tbody>
          <tfoot class="bg-[#f1f4f9] dark:bg-[#1e293b] font-bold">
            <tr>
              <td class="px-6 py-4 text-gray-900 dark:text-white">合计</td>
              <td class="px-6 py-4 text-right font-mono text-base">437,150.00</td>
              <td class="px-6 py-4 text-right font-mono text-base">385,400.00</td>
              <td class="px-6 py-4 text-right font-mono text-base">120,000.00</td>
              <td class="px-6 py-4 text-right font-mono text-base text-primary">842,550.00</td>
            </tr>
          </tfoot>
        </table>
      </div>
    </div>

    <!-- Footer -->
    <footer class="h-10 border-t border-[#e7ebf3] dark:border-[#2d3a54] bg-white dark:bg-[#1a2235] px-6 flex items-center justify-between text-xs text-gray-500 shrink-0">
        <div class="flex items-center gap-4">
            <span class="flex items-center gap-1"><span class="w-2 h-2 rounded-full bg-green-500"></span> 系统已连接</span>
            <span>数据最后更新: 2023-11-27 14:35:12</span>
        </div>
        <div class="flex items-center gap-4">
            <span>显示 1-3 条，共 3 条会计科目</span>
            <div class="flex items-center gap-2">
                <button class="material-symbols-outlined text-base p-1 hover:bg-gray-100 rounded">chevron_left</button>
                <span class="bg-primary text-white px-2 py-0.5 rounded leading-none font-medium">1</span>
                <button class="material-symbols-outlined text-base p-1 hover:bg-gray-100 rounded">chevron_right</button>
            </div>
        </div>
    </footer>
  </div>
</template>
