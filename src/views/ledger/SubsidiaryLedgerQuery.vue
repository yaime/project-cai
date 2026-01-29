<template>
  <div class="flex flex-col h-full bg-background-light dark:bg-background-dark overflow-hidden">
    <div class="bg-white dark:bg-gray-900 border-b p-4 flex justify-between items-end">
        <div>
            <h1 class="text-2xl font-black">1221.01 其他应收款</h1>
            <div class="flex gap-2 mt-2">
                <span class="bg-blue-50 text-blue-600 px-2 py-0.5 rounded text-xs">部门: 研发部</span>
                <span class="bg-blue-50 text-blue-600 px-2 py-0.5 rounded text-xs">人员: 张三</span>
            </div>
        </div>
        <div class="flex gap-2">
            <button class="bg-primary text-white px-4 py-2 rounded font-bold">高级查询</button>
        </div>
    </div>

    <div class="flex flex-1 overflow-hidden">
        <aside class="w-64 border-r bg-white p-4 hidden md:block">
            <h3 class="font-bold mb-4">快速筛选</h3>
            <div class="space-y-4">
                <div>
                    <div class="text-xs font-bold text-gray-500 mb-2">部门</div>
                    <label class="flex items-center gap-2"><input type="checkbox" checked /> 研发部</label>
                    <label class="flex items-center gap-2"><input type="checkbox" /> 销售部</label>
                </div>
            </div>
        </aside>

        <div class="flex-1 overflow-auto bg-white">
            <table class="w-full text-sm border-collapse">
                <thead class="bg-gray-50 sticky top-0">
                    <tr>
                        <th class="p-2 border">日期</th>
                        <th class="p-2 border">凭证</th>
                        <th class="p-2 border">摘要</th>
                        <th class="p-2 border text-right">借方</th>
                        <th class="p-2 border text-right">贷方</th>
                        <th class="p-2 border text-center">方向</th>
                        <th class="p-2 border text-right">余额</th>
                    </tr>
                </thead>
                <tbody>
                    <tr class="hover:bg-blue-50">
                        <td class="p-2 border">2023-01-15</td>
                        <td class="p-2 border text-primary underline">记-001</td>
                        <td class="p-2 border">出差备用金借款</td>
                        <td class="p-2 border text-right font-mono">5,000.00</td>
                        <td class="p-2 border text-right"></td>
                        <td class="p-2 border text-center">借</td>
                        <td class="p-2 border text-right font-mono">5,000.00</td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>
  </div>
</template>
