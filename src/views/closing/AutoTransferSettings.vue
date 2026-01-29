<template>
  <div class="flex flex-col h-full bg-background-light dark:bg-background-dark p-6 overflow-y-auto">
    <div class="max-w-[1440px] mx-auto w-full flex flex-col gap-6">
        <h2 class="text-3xl font-black">自动结转模板设置</h2>

        <div class="bg-white dark:bg-slate-900 p-6 rounded-xl border shadow-sm">
            <h3 class="font-bold mb-4">凭证生成设置</h3>
            <div class="grid grid-cols-4 gap-6">
                <div>
                    <label class="block text-sm font-medium text-gray-500 mb-1">凭证字</label>
                    <select class="w-full p-2 border rounded"><option>转</option></select>
                </div>
                <div class="col-span-2">
                    <label class="block text-sm font-medium text-gray-500 mb-1">摘要模板</label>
                    <input class="w-full p-2 border rounded" value="结转本期损益" />
                </div>
            </div>
        </div>

        <div class="flex gap-6 h-[500px]">
            <div class="flex-1 bg-white dark:bg-slate-900 rounded-xl border shadow-sm flex flex-col">
                <div class="p-4 border-b font-bold flex justify-between">
                    <span>转出科目 (损益类)</span>
                    <span class="text-xs bg-blue-100 text-blue-700 px-2 py-1 rounded">自动平衡</span>
                </div>
                <div class="flex-1 overflow-auto p-0">
                    <table class="w-full text-left">
                        <thead class="bg-gray-50 sticky top-0">
                            <tr>
                                <th class="p-3 w-10"><input type="checkbox" checked /></th>
                                <th class="p-3">科目</th>
                                <th class="p-3 text-right">余额</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="border-b">
                                <td class="p-3"><input type="checkbox" checked /></td>
                                <td class="p-3">5001 主营业务收入</td>
                                <td class="p-3 text-right font-mono">125,000.00</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>

            <div class="w-80 bg-white dark:bg-slate-900 rounded-xl border shadow-sm p-6">
                <div class="bg-gradient-to-r from-primary to-blue-600 p-4 rounded-lg text-white mb-4">
                    <h3 class="font-bold">转入目标</h3>
                    <p class="text-sm opacity-90">本年利润</p>
                </div>
                <div class="p-4 border rounded bg-gray-50">
                    <div class="text-2xl font-black font-mono">4103</div>
                    <div class="font-bold">本年利润</div>
                </div>
            </div>
        </div>

        <div class="fixed bottom-0 left-0 w-full bg-white border-t p-4 flex justify-between items-center px-10 shadow-lg">
            <div class="font-bold text-lg">借贷总计: 133,500.00</div>
            <button class="bg-primary text-white px-8 py-3 rounded font-bold shadow hover:bg-blue-700">一键生成结转凭证</button>
        </div>
    </div>
  </div>
</template>
