<template>
  <div class="flex flex-col h-full bg-[#f8f9fc] dark:bg-[#101622] overflow-hidden">
    <div class="px-6 py-4 border-b bg-white flex justify-between items-center">
        <div>
            <h2 class="text-2xl font-bold">凭证审核</h2>
            <p class="text-gray-500 text-sm">查看并审核待处理凭证</p>
        </div>
        <div class="flex gap-3">
            <button class="bg-primary text-white px-4 py-2 rounded font-bold">新增凭证</button>
        </div>
    </div>

    <div class="flex-1 overflow-auto p-6">
        <div class="bg-white rounded-lg shadow overflow-hidden">
            <table class="w-full text-left">
                <thead class="bg-gray-50">
                    <tr>
                        <th class="p-4 w-10"><input type="checkbox" /></th>
                        <th class="p-4">日期</th>
                        <th class="p-4">凭证号</th>
                        <th class="p-4">摘要</th>
                        <th class="p-4 text-right">金额</th>
                        <th class="p-4 text-center">状态</th>
                    </tr>
                </thead>
                <tbody class="divide-y">
                    <tr class="hover:bg-gray-50 bg-blue-50/30">
                        <td class="p-4"><input type="checkbox" checked /></td>
                        <td class="p-4">2023-10-24</td>
                        <td class="p-4 font-bold text-primary">VCH-2023-089</td>
                        <td class="p-4">Q3 服务器维护费</td>
                        <td class="p-4 text-right font-mono">12,500.00</td>
                        <td class="p-4 text-center"><span class="bg-yellow-100 text-yellow-800 px-2 py-1 rounded text-xs">待审核</span></td>
                    </tr>
                    <tr class="hover:bg-gray-50">
                        <td class="p-4"><input type="checkbox" /></td>
                        <td class="p-4">2023-10-24</td>
                        <td class="p-4 font-bold text-primary">VCH-2023-088</td>
                        <td class="p-4">办公用品</td>
                        <td class="p-4 text-right font-mono">432.50</td>
                        <td class="p-4 text-center"><span class="bg-green-100 text-green-800 px-2 py-1 rounded text-xs">已审核</span></td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>

    <div class="p-4 bg-white border-t flex justify-center sticky bottom-0">
        <div class="bg-gray-900 text-white px-6 py-3 rounded-full shadow-xl flex items-center gap-6">
            <span class="font-bold">3 张凭证已选择</span>
            <div class="flex gap-3">
                <button class="text-red-400 font-bold hover:text-red-300">驳回</button>
                <button class="bg-green-600 hover:bg-green-500 px-4 py-1 rounded font-bold text-white">批量审核</button>
            </div>
        </div>
    </div>
  </div>
</template>
