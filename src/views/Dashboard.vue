<template>
  <div class="flex-1 overflow-y-auto p-6 custom-scrollbar h-full">
    <div class="max-w-[1400px] mx-auto space-y-6">
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        <div class="bg-white dark:bg-gray-800 rounded-xl p-5 border border-border-light dark:border-gray-700 shadow-card flex flex-col justify-between h-[140px]">
          <div class="flex justify-between items-start">
            <div class="flex flex-col gap-1">
              <p class="text-sm font-medium text-text-sub">银行存款</p>
              <h3 class="text-2xl font-bold text-text-main font-mono-nums">¥1,240,500</h3>
            </div>
            <div class="flex items-center gap-1 text-green-600 bg-green-50 px-2 py-1 rounded text-xs font-medium">
              <span class="material-symbols-outlined text-[16px]">trending_up</span>
              2.4%
            </div>
          </div>
          <div class="h-10 w-full mt-2">
            <!-- SVG Chart Placeholder -->
            <svg class="w-full h-full text-primary" preserveAspectRatio="none" viewBox="0 0 100 25">
              <path d="M0 20 Q 10 22 20 15 T 40 10 T 60 18 T 80 5 T 100 12" fill="none" stroke="currentColor" stroke-width="2" vector-effect="non-scaling-stroke"></path>
              <defs>
                <linearGradient id="gradient1" x1="0%" x2="0%" y1="0%" y2="100%">
                  <stop offset="0%" style="stop-color:currentColor;stop-opacity:0.2"></stop>
                  <stop offset="100%" style="stop-color:currentColor;stop-opacity:0"></stop>
                </linearGradient>
              </defs>
              <path d="M0 20 Q 10 22 20 15 T 40 10 T 60 18 T 80 5 T 100 12 V 25 H 0 Z" fill="url(#gradient1)" stroke="none"></path>
            </svg>
          </div>
        </div>
        <div class="bg-white dark:bg-gray-800 rounded-xl p-5 border border-border-light dark:border-gray-700 shadow-card flex flex-col justify-between h-[140px]">
          <div class="flex justify-between items-start">
            <div class="flex flex-col gap-1">
              <p class="text-sm font-medium text-text-sub">应收账款</p>
              <h3 class="text-2xl font-bold text-text-main font-mono-nums">¥342,000</h3>
            </div>
            <div class="flex items-center gap-1 text-red-600 bg-red-50 px-2 py-1 rounded text-xs font-medium">
              <span class="material-symbols-outlined text-[16px]">trending_down</span>
              1.2%
            </div>
          </div>
          <div class="h-10 w-full mt-2">
             <svg class="w-full h-full text-yellow-500" preserveAspectRatio="none" viewBox="0 0 100 25">
              <path d="M0 10 Q 25 25 50 15 T 100 20" fill="none" stroke="currentColor" stroke-width="2" vector-effect="non-scaling-stroke"></path>
            </svg>
          </div>
        </div>
        <div class="bg-white dark:bg-gray-800 rounded-xl p-5 border border-border-light dark:border-gray-700 shadow-card flex flex-col justify-between h-[140px]">
          <div class="flex justify-between items-start">
            <div class="flex flex-col gap-1">
              <p class="text-sm font-medium text-text-sub">应付账款</p>
              <h3 class="text-2xl font-bold text-text-main font-mono-nums">¥120,000</h3>
            </div>
            <div class="flex items-center gap-1 text-green-600 bg-green-50 px-2 py-1 rounded text-xs font-medium">
              <span class="material-symbols-outlined text-[16px]">trending_up</span>
              0.5%
            </div>
          </div>
          <div class="h-10 w-full mt-2">
             <svg class="w-full h-full text-green-500" preserveAspectRatio="none" viewBox="0 0 100 25">
              <path d="M0 22 L 20 18 L 40 20 L 60 10 L 80 15 L 100 5" fill="none" stroke="currentColor" stroke-width="2" vector-effect="non-scaling-stroke"></path>
            </svg>
          </div>
        </div>
        <div class="bg-white dark:bg-gray-800 rounded-xl p-5 border border-border-light dark:border-gray-700 shadow-card flex flex-col justify-between h-[140px]">
          <div class="flex justify-between items-start">
            <div class="flex flex-col gap-1">
              <p class="text-sm font-medium text-text-sub">净利润</p>
              <h3 class="text-2xl font-bold text-text-main font-mono-nums">¥840,000</h3>
            </div>
            <div class="flex items-center gap-1 text-green-600 bg-green-50 px-2 py-1 rounded text-xs font-medium">
              <span class="material-symbols-outlined text-[16px]">trending_up</span>
              5.8%
            </div>
          </div>
          <div class="h-10 w-full mt-2">
             <svg class="w-full h-full text-indigo-500" preserveAspectRatio="none" viewBox="0 0 100 25">
              <path d="M0 20 C 20 20 20 5 40 5 C 60 5 60 15 80 15 C 90 15 90 2 100 2" fill="none" stroke="currentColor" stroke-width="2" vector-effect="non-scaling-stroke"></path>
            </svg>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-12 gap-6 h-auto lg:h-[600px]">
        <div class="lg:col-span-5 bg-white dark:bg-gray-800 rounded-xl border border-border-light dark:border-gray-700 shadow-card flex flex-col overflow-hidden">
          <div class="px-5 py-4 border-b border-border-light dark:border-gray-700 flex justify-between items-center bg-white dark:bg-gray-800 z-10">
            <h3 class="font-bold text-base text-text-main">待审核凭证</h3>
            <button class="text-primary hover:text-primary-hover text-sm font-medium">查看全部</button>
          </div>
          <div class="overflow-y-auto custom-scrollbar flex-1">
            <table class="w-full text-sm text-left">
              <thead class="text-xs text-text-sub bg-gray-50 dark:bg-gray-900 sticky top-0 z-10 border-b border-border-light">
                <tr>
                  <th class="px-5 py-3 font-medium" scope="col">凭证编号</th>
                  <th class="px-5 py-3 font-medium" scope="col">收款人</th>
                  <th class="px-5 py-3 font-medium text-right" scope="col">金额</th>
                  <th class="px-5 py-3 font-medium text-center" scope="col">状态</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-border-light dark:divide-gray-700">
                <tr class="hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors group cursor-pointer">
                  <td class="px-5 py-3 font-medium text-text-main">
                    <div class="flex flex-col">
                      <span class="font-mono-nums">#V-202301</span>
                      <span class="text-[10px] text-text-sub font-normal">Oct 24</span>
                    </div>
                  </td>
                  <td class="px-5 py-3 text-text-main truncate max-w-[100px]">Vendor Alpha LLC</td>
                  <td class="px-5 py-3 text-right font-mono-nums font-medium">¥4,500.00</td>
                  <td class="px-5 py-3 text-center">
                    <span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-yellow-100 text-yellow-800 border border-yellow-200">
                      待复核
                    </span>
                  </td>
                </tr>
                <tr class="hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors group cursor-pointer">
                  <td class="px-5 py-3 font-medium text-text-main">
                    <div class="flex flex-col">
                      <span class="font-mono-nums">#V-202302</span>
                      <span class="text-[10px] text-text-sub font-normal">Oct 23</span>
                    </div>
                  </td>
                  <td class="px-5 py-3 text-text-main truncate max-w-[100px]">TechSolutions Inc.</td>
                  <td class="px-5 py-3 text-right font-mono-nums font-medium">¥1,250.00</td>
                  <td class="px-5 py-3 text-center">
                    <span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-blue-100 text-blue-800 border border-blue-200">
                      处理中
                    </span>
                  </td>
                </tr>
                 <!-- More rows as in design -->
                 <tr class="hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors group cursor-pointer">
                  <td class="px-5 py-3 font-medium text-text-main">
                    <div class="flex flex-col">
                      <span class="font-mono-nums">#V-202303</span>
                      <span class="text-[10px] text-text-sub font-normal">Oct 23</span>
                    </div>
                  </td>
                  <td class="px-5 py-3 text-text-main truncate max-w-[100px]">Global Office Supplies</td>
                  <td class="px-5 py-3 text-right font-mono-nums font-medium">¥320.50</td>
                  <td class="px-5 py-3 text-center">
                    <span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-blue-100 text-blue-800 border border-blue-200">
                      处理中
                    </span>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
          <div class="p-4 border-t border-border-light bg-gray-50 dark:bg-gray-900 flex justify-end">
            <button class="bg-primary text-white text-xs font-medium px-4 py-2 rounded-lg hover:bg-primary-hover transition-colors shadow-sm">
              审核选中项
            </button>
          </div>
        </div>

        <div class="lg:col-span-7 bg-white dark:bg-gray-800 rounded-xl border border-border-light dark:border-gray-700 shadow-card flex flex-col">
          <div class="px-6 py-4 border-b border-border-light dark:border-gray-700 flex justify-between items-center">
            <div>
              <h3 class="font-bold text-base text-text-main">现金流量趋势</h3>
              <p class="text-xs text-text-sub mt-1">收入与支出对比 (最近12个月)</p>
            </div>
            <select class="bg-gray-50 border border-border-light text-text-sub text-xs rounded-lg focus:ring-primary focus:border-primary block p-2 outline-none">
              <option>最近12个月</option>
              <option>今年</option>
              <option>上季度</option>
            </select>
          </div>
          <div class="flex-1 p-6 flex flex-col relative min-h-[300px]">
             <!-- Chart Visualization Placeholder matching Design -->
             <div class="flex items-end justify-between h-full w-full gap-2 pt-6 pb-2 pl-8 border-l border-b border-dashed border-gray-200 dark:border-gray-700 relative">
                <div class="absolute left-0 top-0 h-full -ml-8 flex flex-col justify-between text-[10px] text-text-sub font-mono-nums py-2">
                  <span>100万</span>
                  <span>75万</span>
                  <span>50万</span>
                  <span>25万</span>
                  <span>0</span>
                </div>
                <!-- Bars or Lines would go here -->
                <div class="w-full h-full flex items-center justify-center text-gray-400 text-sm">
                   [Chart Visualization]
                </div>
             </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
