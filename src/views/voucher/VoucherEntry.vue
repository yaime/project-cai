<template>
  <div class="flex flex-col h-full bg-background-light dark:bg-background-dark overflow-y-auto custom-scrollbar p-6 md:px-10 md:py-8">
    <div class="max-w-[1280px] mx-auto w-full">
      <div class="flex flex-wrap justify-between items-end gap-4 mb-6">
        <div>
          <h1 class="text-3xl font-bold text-slate-900 dark:text-white mb-2">新增记账凭证</h1>
          <p class="text-slate-500 dark:text-slate-400 text-sm">创建并记录总账财务交易。</p>
        </div>
        <div class="flex gap-3">
          <button class="flex items-center justify-center h-10 px-5 rounded-lg bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 text-slate-700 dark:text-slate-200 font-medium text-sm hover:bg-slate-50 dark:hover:bg-slate-700 transition-colors shadow-sm">
            取消
          </button>
          <button class="flex items-center justify-center gap-2 h-10 px-6 rounded-lg bg-primary text-white font-bold text-sm hover:bg-blue-700 transition-colors shadow-sm">
            <span class="material-symbols-outlined text-[20px]">save</span>
            保存凭证
          </button>
        </div>
      </div>

      <div class="bg-white dark:bg-[#151c2a] rounded-xl shadow-lg border border-slate-200 dark:border-slate-700 overflow-visible">
        <div class="p-6 md:p-8 border-b border-slate-200 dark:border-slate-700 bg-white dark:bg-[#151c2a]">
          <div class="flex flex-col md:flex-row md:items-start justify-between gap-6 mb-8">
            <div class="flex items-center gap-4">
              <div class="size-12 bg-blue-50 dark:bg-blue-900/20 rounded-lg flex items-center justify-center text-primary">
                <span class="material-symbols-outlined text-[28px]">receipt_long</span>
              </div>
              <div>
                <h3 class="text-lg font-bold text-slate-900 dark:text-white">记账凭证</h3>
                <p class="text-sm text-slate-500">手工录入</p>
              </div>
            </div>
            <div class="flex items-center gap-2 px-4 py-2 bg-slate-50 dark:bg-slate-800 rounded border border-slate-200 dark:border-slate-700">
              <span class="text-xs font-bold text-slate-400 uppercase tracking-wider">凭证字号</span>
              <span class="text-lg font-mono font-bold text-slate-900 dark:text-white">JV-2023-001</span>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
            <label class="flex flex-col gap-2">
              <span class="text-xs font-semibold text-slate-500 uppercase tracking-wider">业务日期</span>
              <div class="relative">
                <span class="material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-slate-400 text-[18px]">calendar_today</span>
                <input class="w-full pl-10 pr-3 py-2 bg-slate-50 dark:bg-slate-900 border border-slate-300 dark:border-slate-600 rounded-md text-sm text-slate-900 dark:text-white focus:ring-primary focus:border-primary" type="date" value="2023-10-27"/>
              </div>
            </label>
            <label class="flex flex-col gap-2">
              <span class="text-xs font-semibold text-slate-500 uppercase tracking-wider">成本中心</span>
              <select class="w-full pl-3 pr-8 py-2 bg-slate-50 dark:bg-slate-900 border border-slate-300 dark:border-slate-600 rounded-md text-sm text-slate-900 dark:text-white focus:ring-primary focus:border-primary">
                <option>总部 (HQ)</option>
                <option>销售部</option>
                <option>技术部</option>
              </select>
            </label>
            <label class="flex flex-col gap-2">
              <span class="text-xs font-semibold text-slate-500 uppercase tracking-wider">参考号</span>
              <input class="w-full px-3 py-2 bg-slate-50 dark:bg-slate-900 border border-slate-300 dark:border-slate-600 rounded-md text-sm text-slate-900 dark:text-white focus:ring-primary focus:border-primary" placeholder="例如：INV-2023-999" type="text"/>
            </label>
            <div class="flex flex-col gap-2">
              <span class="text-xs font-semibold text-slate-500 uppercase tracking-wider">附件</span>
              <button class="w-full flex items-center justify-between px-3 py-2 bg-white dark:bg-slate-800 border border-dashed border-primary/50 rounded-md text-primary hover:bg-blue-50 dark:hover:bg-blue-900/10 transition-colors group">
                <span class="text-sm font-medium flex items-center gap-2">
                  <span class="material-symbols-outlined text-[18px]">attach_file</span>
                  2个附件
                </span>
                <span class="material-symbols-outlined text-[16px] opacity-0 group-hover:opacity-100 transition-opacity">add</span>
              </button>
            </div>
          </div>
        </div>

        <div class="w-full overflow-x-visible bg-white dark:bg-[#151c2a]">
          <table class="w-full border-collapse border-y border-slate-300 dark:border-slate-700 min-w-[1000px]">
            <thead>
              <tr class="bg-slate-100 dark:bg-slate-900">
                <th class="w-12 border-r border-slate-300 dark:border-slate-700 py-3 text-center text-slate-400 font-medium text-xs">#</th>
                <th class="w-12 border-r border-slate-300 dark:border-slate-700 py-3 text-center text-slate-400">
                  <span class="material-symbols-outlined text-[18px]">settings</span>
                </th>
                <th class="w-[30%] border-r border-slate-300 dark:border-slate-700 px-4 py-3 text-left font-semibold text-slate-700 dark:text-slate-200 text-sm">摘要</th>
                <th class="w-[30%] border-r border-slate-300 dark:border-slate-700 px-4 py-3 text-left font-semibold text-slate-700 dark:text-slate-200 text-sm">会计科目</th>
                <th class="w-[15%] border-r border-slate-300 dark:border-slate-700 px-4 py-3 text-right font-semibold text-slate-700 dark:text-slate-200 text-sm">借方金额</th>
                <th class="w-[15%] px-4 py-3 text-right font-semibold text-slate-700 dark:text-slate-200 text-sm">贷方金额</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-slate-300 dark:divide-slate-700">
              <tr class="group bg-blue-50/20 dark:bg-blue-900/10 relative z-10 shadow-[inset_0_0_0_2px_#135bec] transition-colors">
                <td class="border-r border-slate-300 dark:border-slate-700 text-center text-primary font-bold text-xs font-mono">1</td>
                <td class="border-r border-slate-300 dark:border-slate-700 text-center">
                  <button class="text-slate-300 hover:text-red-500 dark:text-slate-600 transition-colors mt-1">
                    <span class="material-symbols-outlined text-[18px]">do_not_disturb_on</span>
                  </button>
                </td>
                <td class="border-r border-slate-300 dark:border-slate-700 p-0">
                  <input class="table-input w-full h-11 px-4 border-none bg-transparent text-sm text-slate-900 dark:text-white placeholder:text-slate-400 focus:ring-0" placeholder="摘要" type="text" value="10月办公室房租"/>
                </td>
                <td class="border-r border-slate-300 dark:border-slate-700 p-0 relative">
                  <button class="table-input w-full h-11 px-4 flex items-center justify-between text-left text-sm text-primary bg-blue-50/40 dark:bg-blue-900/10 focus:outline-none">
                    <span class="font-medium truncate">6602 - 管理费用</span>
                    <span class="material-symbols-outlined text-[18px] opacity-70">arrow_drop_down</span>
                  </button>
                </td>
                <td class="border-r border-slate-300 dark:border-slate-700 p-0">
                  <input class="table-input w-full h-11 px-4 border-none bg-transparent text-right font-mono text-sm text-slate-900 dark:text-white focus:ring-0" placeholder="0.00" type="text" value="5,000.00"/>
                </td>
                <td class="p-0">
                  <input class="table-input w-full h-11 px-4 border-none bg-transparent text-right font-mono text-sm text-slate-900 dark:text-white placeholder:text-slate-300 focus:ring-0" placeholder="0.00" type="text"/>
                </td>
              </tr>
              <!-- More rows as in design -->
            </tbody>
            <tfoot>
              <tr class="bg-emerald-50 dark:bg-emerald-900/20 border-t border-slate-300 dark:border-slate-700">
                <td class="border-r border-emerald-200 dark:border-emerald-800/30 px-4 py-3 text-right" colspan="4">
                  <div class="flex items-center justify-end gap-2 text-emerald-700 dark:text-emerald-400">
                    <span class="material-symbols-outlined text-[18px]">check_circle</span>
                    <span class="text-xs font-bold uppercase tracking-wider">合计金额</span>
                  </div>
                </td>
                <td class="border-r border-emerald-200 dark:border-emerald-800/30 px-4 py-3 text-right bg-emerald-100/30 dark:bg-emerald-900/20">
                  <span class="font-mono font-bold text-emerald-800 dark:text-emerald-300">5,000.00</span>
                </td>
                <td class="px-4 py-3 text-right bg-emerald-100/30 dark:bg-emerald-900/20">
                  <span class="font-mono font-bold text-emerald-800 dark:text-emerald-300">5,000.00</span>
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
    box-shadow: inset 0 0 0 2px #135bec;
}
.paper-texture {
    background-color: #fdfbf7;
    background-image: linear-gradient(0deg, transparent 24%, rgba(0, 0, 0, .02) 25%, rgba(0, 0, 0, .02) 26%, transparent 27%, transparent 74%, rgba(0, 0, 0, .02) 75%, rgba(0, 0, 0, .02) 76%, transparent 77%, transparent), linear-gradient(90deg, transparent 24%, rgba(0, 0, 0, .02) 25%, rgba(0, 0, 0, .02) 26%, transparent 27%, transparent 74%, rgba(0, 0, 0, .02) 75%, rgba(0, 0, 0, .02) 76%, transparent 77%, transparent);
    background-size: 30px 30px;
}
</style>
