<template>
  <div class="flex flex-col h-full bg-background-light dark:bg-background-dark items-center py-10 px-4 overflow-hidden">
    <div class="w-full max-w-[1400px] flex flex-col gap-6 h-full">
        <h1 class="text-2xl font-bold">期初余额录入</h1>
        <div class="bg-white dark:bg-slate-900 rounded-xl shadow border flex-1 overflow-hidden flex flex-col">
            <div class="overflow-auto flex-1 p-0">
                <table class="w-full text-left">
                    <thead class="bg-gray-50 dark:bg-slate-800 sticky top-0">
                        <tr>
                            <th class="p-3 border-b">科目代码</th>
                            <th class="p-3 border-b">科目名称</th>
                            <th class="p-3 border-b text-right">期初余额</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr class="border-b">
                            <td class="p-3 font-mono">1001</td>
                            <td class="p-3">库存现金</td>
                            <td class="p-1"><input class="w-full text-right p-2" value="5,000.00" /></td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="p-4 border-t flex justify-between items-center bg-gray-50">
                <span>试算平衡状态: <span class="text-green-600 font-bold">平衡</span></span>
                <button class="bg-primary text-white px-6 py-2 rounded">完成并启用</button>
            </div>
        </div>
    </div>
  </div>
</template>
