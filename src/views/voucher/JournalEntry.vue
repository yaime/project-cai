<template>
  <div class="flex flex-col h-full bg-background-light dark:bg-background-dark p-6 md:px-10 md:py-8 overflow-y-auto pb-24">
    <div class="max-w-[1280px] mx-auto w-full">
      <div class="flex flex-wrap justify-between items-end gap-4 mb-6">
        <div>
          <h1 class="text-3xl font-bold text-slate-900 dark:text-white mb-2">New Journal Entry</h1>
          <p class="text-slate-500 dark:text-slate-400 text-sm">Create and record financial transactions for the general ledger.</p>
        </div>
        <div class="flex gap-3 items-center">
          <span class="text-xs text-slate-400 font-mono hidden lg:block mr-2">
            Auto-saving... <span class="material-symbols-outlined text-[14px] align-middle ml-1">cloud_done</span>
          </span>
          <button class="flex items-center justify-center h-10 px-5 rounded-lg bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 text-slate-700 dark:text-slate-200 font-medium text-sm hover:bg-slate-50 dark:hover:bg-slate-700 transition-colors shadow-sm">
            Cancel
          </button>
          <button class="flex items-center justify-center gap-2 h-10 px-6 rounded-lg bg-primary text-white font-bold text-sm hover:bg-blue-700 transition-colors shadow-sm group">
            <span class="material-symbols-outlined text-[20px]">save</span>
            <span>Save Voucher</span>
            <span class="bg-white/20 text-white text-[10px] px-1.5 py-0.5 rounded font-mono ml-1 opacity-0 group-hover:opacity-100 transition-opacity">Ctrl+S</span>
          </button>
        </div>
      </div>

      <div class="bg-white dark:bg-[#151c2a] rounded-xl shadow-lg border border-slate-200 dark:border-slate-700 overflow-hidden">
        <div class="p-6 md:p-8 border-b border-slate-200 dark:border-slate-700 bg-white dark:bg-[#151c2a]">
          <!-- Header Inputs -->
          <div class="flex flex-col md:flex-row md:items-start justify-between gap-6 mb-8">
            <div class="flex items-center gap-4">
              <div class="size-12 bg-blue-50 dark:bg-blue-900/20 rounded-lg flex items-center justify-center text-primary">
                <span class="material-symbols-outlined text-[28px]">receipt_long</span>
              </div>
              <div>
                <h3 class="text-lg font-bold text-slate-900 dark:text-white">General Journal</h3>
                <p class="text-sm text-slate-500">Manual Entry</p>
              </div>
            </div>
            <div class="flex items-center gap-2 px-4 py-2 bg-slate-50 dark:bg-slate-800 rounded border border-slate-200 dark:border-slate-700">
              <span class="text-xs font-bold text-slate-400 uppercase tracking-wider">Voucher No.</span>
              <span class="text-lg font-mono font-bold text-slate-900 dark:text-white">JV-2023-001</span>
            </div>
          </div>
          <!-- Form -->
          <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
            <label class="flex flex-col gap-2">
              <span class="text-xs font-semibold text-slate-500 uppercase tracking-wider">Transaction Date</span>
              <div class="relative">
                <span class="material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-slate-400 text-[18px]">calendar_today</span>
                <input class="w-full pl-10 pr-3 py-2 bg-slate-50 dark:bg-slate-900 border border-slate-300 dark:border-slate-600 rounded-md text-sm text-slate-900 dark:text-white focus:ring-primary focus:border-primary" type="date" value="2023-10-27"/>
              </div>
            </label>
            <!-- Other inputs -->
          </div>
        </div>

        <!-- Grid -->
        <div class="w-full overflow-x-auto bg-white dark:bg-[#151c2a] relative">
          <table class="w-full border-collapse border-y border-slate-300 dark:border-slate-700 min-w-[1000px]">
            <thead>
              <tr class="bg-slate-100 dark:bg-slate-900">
                <th class="w-12 border-r border-slate-300 dark:border-slate-700 py-3 text-center text-slate-400 font-medium text-xs">#</th>
                <th class="w-12 border-r border-slate-300 dark:border-slate-700 py-3 text-center text-slate-400"><span class="material-symbols-outlined text-[18px]">settings</span></th>
                <th class="w-[30%] border-r border-slate-300 dark:border-slate-700 px-4 py-3 text-left font-semibold text-slate-700 dark:text-slate-200 text-sm">Summary</th>
                <th class="w-[30%] border-r border-slate-300 dark:border-slate-700 px-4 py-3 text-left font-semibold text-slate-700 dark:text-slate-200 text-sm">Account Subject</th>
                <th class="w-[15%] border-r border-slate-300 dark:border-slate-700 px-4 py-3 text-right font-semibold text-slate-700 dark:text-slate-200 text-sm font-mono">Debit</th>
                <th class="w-[15%] px-4 py-3 text-right font-semibold text-slate-700 dark:text-slate-200 text-sm font-mono">Credit</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-slate-300 dark:divide-slate-700">
              <tr class="group hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors">
                <td class="border-r border-slate-300 dark:border-slate-700 text-center text-slate-400 text-xs font-mono bg-slate-50 dark:bg-slate-900/20">1</td>
                <td class="border-r border-slate-300 dark:border-slate-700 text-center">
                  <button class="text-slate-300 hover:text-red-500 dark:text-slate-600 transition-colors mt-1"><span class="material-symbols-outlined text-[18px]">do_not_disturb_on</span></button>
                </td>
                <td class="border-r border-slate-300 dark:border-slate-700 p-0">
                  <input class="table-input w-full h-11 px-4 border-none bg-transparent text-sm text-slate-900 dark:text-white placeholder:text-slate-400" placeholder="Description" type="text" value="Office Rent Payment Oct"/>
                </td>
                <td class="border-r border-slate-300 dark:border-slate-700 p-0 relative">
                  <div class="relative w-full h-full">
                    <button class="ag-cell-focus w-full h-11 px-4 flex items-center justify-between text-left text-sm text-primary bg-white focus:outline-none ring-2 ring-primary z-10 relative shadow-lg">
                      <span class="font-medium truncate">6602 - Admin Expenses</span>
                      <span class="material-symbols-outlined text-[18px]">arrow_drop_up</span>
                    </button>
                  </div>
                </td>
                <td class="border-r border-slate-300 dark:border-slate-700 p-0">
                  <input class="table-input w-full h-11 px-4 border-none bg-transparent text-right font-mono text-sm text-slate-900 dark:text-white" placeholder="0.00" type="text" value="5,000.00"/>
                </td>
                <td class="p-0">
                  <input class="table-input w-full h-11 px-4 border-none bg-transparent text-right font-mono text-sm text-slate-900 dark:text-white placeholder:text-slate-300" placeholder="0.00" type="text"/>
                </td>
              </tr>
            </tbody>
            <tfoot>
              <tr class="bg-slate-50 dark:bg-slate-800/50 border-t border-slate-300 dark:border-slate-700">
                <td class="border-r border-slate-300 dark:border-slate-700 px-4 py-3 text-right" colspan="4">
                  <span class="text-xs font-bold text-slate-500 uppercase tracking-wider mr-2">Total Amount</span>
                </td>
                <td class="border-r border-slate-300 dark:border-slate-700 px-4 py-3 text-right">
                  <span class="font-mono font-bold text-slate-900 dark:text-white">5,000.00</span>
                </td>
                <td class="px-4 py-3 text-right">
                  <span class="font-mono font-bold text-slate-900 dark:text-white">5,000.00</span>
                </td>
              </tr>
            </tfoot>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.table-input:focus {
    outline: none;
    box-shadow: inset 0 0 0 2px #135bec;
    background-color: #fff;
}
.ag-cell-focus {
    box-shadow: inset 0 0 0 2px #135bec !important;
    background-color: white !important;
    z-index: 10;
}
</style>
