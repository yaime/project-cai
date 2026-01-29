<template>
  <div class="flex flex-col h-full bg-background-light dark:bg-background-dark p-6 overflow-hidden">
    <div class="max-w-[1440px] mx-auto w-full flex flex-col gap-6 h-full">
        <div class="flex justify-between items-center shrink-0">
            <h2 class="text-2xl font-black flex items-center gap-2">
                试算平衡表
                <span class="text-xs font-medium bg-gray-100 px-2 py-1 rounded">Step 2 of 3</span>
            </h2>
            <div class="flex gap-2">
                <button class="border px-4 py-2 rounded bg-white">重新测算</button>
                <button class="border px-4 py-2 rounded bg-white">导出 PDF</button>
            </div>
        </div>

        <div class="bg-emerald-50 border border-emerald-100 p-4 rounded-xl flex items-center gap-4 shrink-0">
            <span class="material-symbols-outlined text-emerald-600 text-3xl">check_circle</span>
            <div>
                <h3 class="font-bold text-emerald-900">试算结果：平衡</h3>
                <p class="text-sm text-emerald-700">借贷双方平衡，可以继续。</p>
            </div>
        </div>

        <div class="bg-white dark:bg-slate-900 rounded-xl shadow-sm border flex-1 overflow-hidden flex flex-col">
            <div class="flex-1 overflow-auto custom-scrollbar">
                <table class="w-full text-sm border-collapse">
                    <thead class="bg-gray-50 dark:bg-slate-800 sticky top-0 z-10">
                        <tr>
                            <th class="p-2 border font-bold text-left">科目</th>
                            <th class="p-2 border text-center" colspan="2">期初余额</th>
                            <th class="p-2 border text-center" colspan="2">本期发生</th>
                            <th class="p-2 border text-center" colspan="2">期末余额</th>
                        </tr>
                        <tr>
                            <th class="p-2 border"></th>
                            <th class="p-2 border text-right w-32">借</th>
                            <th class="p-2 border text-right w-32">贷</th>
                            <th class="p-2 border text-right w-32">借</th>
                            <th class="p-2 border text-right w-32">贷</th>
                            <th class="p-2 border text-right w-32">借</th>
                            <th class="p-2 border text-right w-32">贷</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr class="hover:bg-gray-50">
                            <td class="p-2 border">1001 库存现金</td>
                            <td class="p-2 border text-right font-mono">50,000.00</td>
                            <td class="p-2 border text-right font-mono"></td>
                            <td class="p-2 border text-right font-mono">2,000.00</td>
                            <td class="p-2 border text-right font-mono"></td>
                            <td class="p-2 border text-right font-mono">52,000.00</td>
                            <td class="p-2 border text-right font-mono"></td>
                        </tr>
                        <!-- More rows... -->
                    </tbody>
                </table>
            </div>
            <div class="p-4 border-t bg-gray-50 flex justify-between font-bold">
                <span>合计</span>
                <div class="flex gap-8">
                    <span>借: 250,000.00</span>
                    <span>贷: 250,000.00</span>
                </div>
            </div>
        </div>

        <div class="flex justify-end pt-4 shrink-0">
            <button class="bg-primary text-white px-6 py-2 rounded font-bold shadow">下一步：正式结账</button>
        </div>
    </div>
  </div>
</template>
