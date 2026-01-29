<template>
  <div class="fixed inset-0 bg-black/40 backdrop-blur-sm z-50 flex items-center justify-center p-4">
    <div class="relative w-full max-w-[640px] bg-white dark:bg-[#1a2133] shadow-2xl rounded-xl overflow-hidden border border-gray-200">
        <div class="flex items-center justify-between px-6 py-4 border-b">
            <h2 class="text-lg font-bold flex items-center gap-2">
                <span class="material-symbols-outlined text-primary">account_balance_wallet</span>
                辅助核算明细 - <span class="text-primary">1002.01</span>
            </h2>
            <button class="text-gray-400 hover:text-gray-600"><span class="material-symbols-outlined">close</span></button>
        </div>

        <div class="p-6 space-y-5">
            <div class="grid grid-cols-12 items-center gap-4">
                <label class="col-span-3 text-sm font-medium">供应商</label>
                <div class="col-span-9">
                    <input class="w-full p-2.5 border rounded-lg bg-gray-50" value="华为技术有限公司" />
                </div>
            </div>

            <div class="grid grid-cols-12 items-center gap-4">
                <label class="col-span-3 text-sm font-medium">部门</label>
                <div class="col-span-9 flex items-center border rounded-lg p-2 bg-gray-50">
                    <span class="bg-gray-200 px-2 py-0.5 rounded text-xs mr-2">研发部</span>
                    <span class="text-sm">架构组</span>
                </div>
            </div>

            <div class="mt-4 p-4 rounded-lg bg-primary/5 border border-primary/20 flex justify-between items-center">
                <div>
                    <span class="text-xs text-gray-500">本次分摊金额</span>
                    <div class="text-2xl font-bold text-primary">125,000.00</div>
                </div>
            </div>
        </div>

        <div class="px-6 py-4 bg-gray-50 border-t flex justify-end gap-3">
            <button class="px-5 py-2 text-sm font-medium text-gray-600 hover:bg-gray-200 rounded-lg">取消</button>
            <button class="px-6 py-2 text-sm font-medium text-white bg-primary hover:bg-primary/90 rounded-lg">确定入账</button>
        </div>
    </div>
  </div>
</template>
