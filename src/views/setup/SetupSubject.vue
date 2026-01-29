<template>
  <div class="flex flex-col h-full bg-background-light dark:bg-background-dark items-center py-10 px-4 overflow-hidden">
    <div class="w-full max-w-[1400px] flex flex-col gap-6 h-full">
        <h1 class="text-2xl font-bold">科目初始化</h1>
        <div class="grid grid-cols-3 gap-6 flex-1 overflow-hidden">
            <div class="col-span-1 bg-white dark:bg-slate-900 rounded-xl shadow p-6 overflow-auto">
                <h2 class="font-bold mb-4">预置科目方案</h2>
                <div class="space-y-3">
                    <label class="block p-4 border rounded-lg bg-primary/5 border-primary">
                        <div class="font-bold">2013 小企业会计准则</div>
                        <div class="text-xs text-gray-500">推荐方案</div>
                    </label>
                    <label class="block p-4 border rounded-lg hover:bg-gray-50">
                        <div class="font-bold">2006 企业会计准则</div>
                    </label>
                </div>
            </div>
            <div class="col-span-2 bg-white dark:bg-slate-900 rounded-xl shadow p-0 overflow-auto flex flex-col">
                <div class="p-4 border-b font-bold">科目预览</div>
                <div class="flex-1 overflow-auto">
                    <table class="w-full text-left">
                        <thead>
                            <tr class="bg-gray-50">
                                <th class="p-3">代码</th>
                                <th class="p-3">名称</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr><td class="p-3">1001</td><td class="p-3">库存现金</td></tr>
                            <tr><td class="p-3">1002</td><td class="p-3">银行存款</td></tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
        <div class="flex justify-end">
             <button class="bg-primary text-white px-8 py-2.5 rounded-lg font-bold">下一步</button>
        </div>
    </div>
  </div>
</template>
