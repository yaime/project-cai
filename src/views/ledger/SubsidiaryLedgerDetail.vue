<template>
  <div class="flex flex-col h-full bg-background-light dark:bg-background-dark overflow-hidden">
    <!-- Toolbar & Heading -->
    <div class="px-6 py-5 shrink-0 flex flex-col gap-5 bg-background-light dark:bg-background-dark">
      <div class="flex flex-wrap justify-between items-end gap-4">
        <div class="flex flex-col gap-1">
          <div class="flex items-center gap-2 text-xs font-medium text-gray-500 mb-1">
            <span>Accounting</span>
            <span class="material-symbols-outlined text-[10px]">chevron_right</span>
            <span>General Ledger</span>
            <span class="material-symbols-outlined text-[10px]">chevron_right</span>
            <span class="text-primary">Subsidiary Ledger</span>
          </div>
          <div class="flex items-center gap-3">
            <h1 class="text-[#0d121b] dark:text-white text-2xl font-bold leading-tight">Subsidiary Ledger <span class="text-lg font-normal text-gray-500 ml-2">(明细账)</span></h1>
            <span class="px-2 py-0.5 rounded text-xs font-medium bg-green-100 text-green-700 border border-green-200">Active</span>
          </div>
        </div>
        <div class="flex gap-3">
          <button class="flex items-center justify-center gap-2 h-9 px-4 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg text-sm font-medium text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-700 shadow-sm transition-all">
            <span class="material-symbols-outlined text-[18px]">print</span>
            <span>Batch Print</span>
          </button>
          <button class="flex items-center justify-center gap-2 h-9 px-4 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg text-sm font-medium text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-700 shadow-sm transition-all">
            <span class="material-symbols-outlined text-[18px]">file_upload</span>
            <span>Export PDF</span>
          </button>
          <button class="flex items-center justify-center gap-2 h-9 px-4 bg-primary hover:bg-blue-700 text-white rounded-lg text-sm font-medium shadow-sm transition-all">
            <span class="material-symbols-outlined text-[18px]">refresh</span>
            <span>Refresh</span>
          </button>
        </div>
      </div>
      <!-- Filters -->
      <div class="flex flex-wrap items-center gap-4 bg-white dark:bg-[#1a202c] p-3 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700">
        <div class="flex items-center gap-3 border-r border-gray-200 dark:border-gray-700 pr-4">
          <label class="flex flex-col">
            <span class="text-[11px] font-semibold text-gray-500 uppercase tracking-wider mb-1">Period</span>
            <div class="flex items-center gap-2">
              <input class="h-8 border-none bg-transparent p-0 text-sm font-semibold text-gray-900 dark:text-white focus:ring-0 cursor-pointer" type="month" value="2023-10"/>
            </div>
          </label>
        </div>
        <div class="flex-1 min-w-[200px]">
          <label class="flex flex-col w-full">
            <span class="text-[11px] font-semibold text-gray-500 uppercase tracking-wider mb-1">Account Subject</span>
            <div class="relative group">
              <select class="w-full h-8 appearance-none bg-transparent border-none p-0 text-sm font-semibold text-gray-900 dark:text-white focus:ring-0 cursor-pointer pr-8 truncate">
                <option>1002.01 - Bank Deposit - ICBC (RMB)</option>
                <option>1002.02 - Bank Deposit - CMB (USD)</option>
                <option>1122 - Accounts Receivable</option>
              </select>
              <span class="material-symbols-outlined absolute right-0 top-1/2 -translate-y-1/2 pointer-events-none text-gray-500 text-[20px]">arrow_drop_down</span>
            </div>
          </label>
        </div>
      </div>
    </div>

    <!-- Ledger Table Container -->
    <div class="flex-1 overflow-auto px-6 pb-6 custom-scrollbar">
      <div class="min-w-[1200px] bg-white dark:bg-[#1e2532] border border-gray-300 dark:border-gray-700 shadow-sm rounded-md flex flex-col h-full">
        <!-- Sticky Header -->
        <div class="sticky top-0 z-10 bg-gray-50 dark:bg-[#2d3748] border-b border-gray-300 dark:border-gray-600 shadow-sm">
          <div class="flex h-12 border-b border-gray-200 dark:border-gray-600 text-xs font-bold text-gray-700 dark:text-gray-200">
            <div class="w-24 border-r border-gray-200 dark:border-gray-600 flex items-center justify-center bg-gray-50 dark:bg-[#2d3748]">Date</div>
            <div class="w-24 border-r border-gray-200 dark:border-gray-600 flex items-center justify-center bg-gray-50 dark:bg-[#2d3748]">Voucher</div>
            <div class="flex-1 min-w-[200px] border-r border-gray-200 dark:border-gray-600 flex items-center justify-center bg-gray-50 dark:bg-[#2d3748]">Summary (摘要)</div>
            <div class="w-[220px] border-r border-gray-200 dark:border-gray-600 flex items-center justify-center bg-blue-50/50 dark:bg-blue-900/20 text-primary dark:text-blue-300">Debit (借方)</div>
            <div class="w-[220px] border-r border-gray-200 dark:border-gray-600 flex items-center justify-center bg-red-50/50 dark:bg-red-900/20 text-red-600 dark:text-red-300">Credit (贷方)</div>
            <div class="w-12 border-r border-gray-200 dark:border-gray-600 flex items-center justify-center bg-gray-50 dark:bg-[#2d3748]">Dir</div>
            <div class="w-[220px] flex items-center justify-center bg-gray-50 dark:bg-[#2d3748]">Balance (余额)</div>
          </div>
          <!-- Digit Headers -->
          <div class="flex h-6 text-[10px] text-gray-500 dark:text-gray-400 font-medium bg-gray-100 dark:bg-[#2d3748]">
             <div class="w-24 border-r border-gray-200 dark:border-gray-600"></div>
             <div class="w-24 border-r border-gray-200 dark:border-gray-600"></div>
             <div class="flex-1 min-w-[200px] border-r border-gray-200 dark:border-gray-600"></div>
             <!-- Digit Labels -->
             <div class="w-[220px] border-r border-gray-200 dark:border-gray-600 grid grid-cols-11 text-center items-center">
               <span class="border-r border-gray-200 dark:border-gray-600">亿</span>
               <span class="border-r border-gray-200 dark:border-gray-600">千</span>
               <span class="border-r border-gray-200 dark:border-gray-600">百</span>
               <span class="border-r border-gray-200 dark:border-gray-600">十</span>
               <span class="border-r border-gray-200 dark:border-gray-600">万</span>
               <span class="border-r border-gray-200 dark:border-gray-600">千</span>
               <span class="border-r border-gray-200 dark:border-gray-600">百</span>
               <span class="border-r border-gray-200 dark:border-gray-600">十</span>
               <span class="border-r border-gray-300 dark:border-gray-500 font-bold text-gray-800 dark:text-gray-200 bg-gray-200/50 dark:bg-gray-700/50">元</span>
               <span class="border-r border-gray-200 dark:border-gray-600">角</span>
               <span>分</span>
             </div>
             <!-- Repeat for Credit & Balance -->
             <div class="w-[220px] border-r border-gray-200 dark:border-gray-600 grid grid-cols-11 text-center items-center">
               <span class="border-r border-gray-200 dark:border-gray-600">亿</span>
               <span class="border-r border-gray-200 dark:border-gray-600">千</span>
               <span class="border-r border-gray-200 dark:border-gray-600">百</span>
               <span class="border-r border-gray-200 dark:border-gray-600">十</span>
               <span class="border-r border-gray-200 dark:border-gray-600">万</span>
               <span class="border-r border-gray-200 dark:border-gray-600">千</span>
               <span class="border-r border-gray-200 dark:border-gray-600">百</span>
               <span class="border-r border-gray-200 dark:border-gray-600">十</span>
               <span class="border-r border-gray-300 dark:border-gray-500 font-bold text-gray-800 dark:text-gray-200 bg-gray-200/50 dark:bg-gray-700/50">元</span>
               <span class="border-r border-gray-200 dark:border-gray-600">角</span>
               <span>分</span>
             </div>
             <div class="w-12 border-r border-gray-200 dark:border-gray-600"></div>
             <div class="w-[220px] grid grid-cols-11 text-center items-center">
               <span class="border-r border-gray-200 dark:border-gray-600">亿</span>
               <span class="border-r border-gray-200 dark:border-gray-600">千</span>
               <span class="border-r border-gray-200 dark:border-gray-600">百</span>
               <span class="border-r border-gray-200 dark:border-gray-600">十</span>
               <span class="border-r border-gray-200 dark:border-gray-600">万</span>
               <span class="border-r border-gray-200 dark:border-gray-600">千</span>
               <span class="border-r border-gray-200 dark:border-gray-600">百</span>
               <span class="border-r border-gray-200 dark:border-gray-600">十</span>
               <span class="border-r border-gray-300 dark:border-gray-500 font-bold text-gray-800 dark:text-gray-200 bg-gray-200/50 dark:bg-gray-700/50">元</span>
               <span class="border-r border-gray-200 dark:border-gray-600">角</span>
               <span>分</span>
             </div>
          </div>
        </div>

        <!-- Table Body -->
        <div class="bg-white dark:bg-[#1e2532] text-sm">
          <!-- Opening Balance -->
          <div class="flex h-10 border-b border-gray-200 dark:border-gray-700 hover:bg-yellow-50 dark:hover:bg-gray-800/50 transition-colors group">
            <div class="w-24 border-r border-gray-200 dark:border-gray-700 flex items-center justify-center text-gray-500 font-mono text-xs">2023-10-01</div>
            <div class="w-24 border-r border-gray-200 dark:border-gray-700 flex items-center justify-center text-gray-500 text-xs">-</div>
            <div class="flex-1 min-w-[200px] border-r border-gray-200 dark:border-gray-700 flex items-center px-3 text-gray-900 dark:text-white font-medium">Opening Balance (期初余额)</div>
            <!-- Debit Empty -->
            <div class="w-[220px] border-r border-gray-200 dark:border-gray-700"></div>
            <!-- Credit Empty -->
            <div class="w-[220px] border-r border-gray-200 dark:border-gray-700"></div>
            <div class="w-12 border-r border-gray-200 dark:border-gray-700 flex items-center justify-center text-gray-500 text-xs">Dr</div>
            <div class="w-[220px] bg-white dark:bg-transparent">
               <!-- 1,500,000.00 Simulation for grid -->
               <div class="grid grid-cols-11 h-full text-center items-center font-bold">
                 <span></span><span></span><span>1</span><span>5</span><span>0</span><span>0</span><span>0</span><span>0</span><span>0</span><span>0</span><span>0</span>
               </div>
            </div>
          </div>
          <!-- Transaction Row -->
          <div class="flex h-10 border-b border-gray-200 dark:border-gray-700 hover:bg-blue-50/30 dark:hover:bg-blue-900/10 transition-colors">
            <div class="w-24 border-r border-gray-200 dark:border-gray-700 flex items-center justify-center text-gray-600 font-mono text-xs">2023-10-05</div>
            <div class="w-24 border-r border-gray-200 dark:border-gray-700 flex items-center justify-center text-primary text-xs">Rec-042</div>
            <div class="flex-1 min-w-[200px] border-r border-gray-200 dark:border-gray-700 flex items-center px-3 text-gray-700 dark:text-gray-300">Sales Collection - Client A</div>
            <div class="w-[220px] border-r border-gray-200 dark:border-gray-700">
               <div class="grid grid-cols-11 h-full text-center items-center">
                 <span></span><span></span><span></span><span>2</span><span>5</span><span>0</span><span>0</span><span>0</span><span>0</span><span>5</span><span>0</span>
               </div>
            </div>
            <div class="w-[220px] border-r border-gray-200 dark:border-gray-700"></div>
            <div class="w-12 border-r border-gray-200 dark:border-gray-700 flex items-center justify-center text-gray-500 text-xs">Dr</div>
            <div class="w-[220px]">
               <div class="grid grid-cols-11 h-full text-center items-center">
                 <span></span><span></span><span>1</span><span>7</span><span>5</span><span>0</span><span>0</span><span>0</span><span>0</span><span>5</span><span>0</span>
               </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Specific digit grid styling overrides if needed */
</style>
