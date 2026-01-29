<template>
  <div class="flex flex-col h-full bg-[#f0f2f5] dark:bg-black overflow-hidden">
    <header class="h-14 bg-white px-6 flex items-center justify-between border-b shrink-0">
        <h2 class="font-bold">往来核销管理</h2>
        <div class="flex gap-2">
            <input class="border rounded px-2 py-1 text-sm" placeholder="客户名称..." value="上海科技有限公司" />
            <button class="bg-primary text-white px-3 py-1 rounded text-sm">查询</button>
        </div>
    </header>

    <div class="flex-1 flex p-3 gap-3 overflow-hidden">
        <!-- Left: AR List -->
        <div class="flex-1 bg-white rounded border flex flex-col">
            <div class="p-3 border-b font-bold bg-gray-50">应收单据列表</div>
            <div class="flex-1 overflow-auto">
                <table class="w-full text-sm">
                    <thead class="bg-gray-50 sticky top-0">
                        <tr>
                            <th class="p-2"><input type="checkbox" /></th>
                            <th class="p-2">单据编号</th>
                            <th class="p-2 text-right">未核销金额</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr class="hover:bg-blue-50">
                            <td class="p-2"><input type="checkbox" checked /></td>
                            <td class="p-2">AR-20231015-001</td>
                            <td class="p-2 text-right font-bold">5,000.00</td>
                        </tr>
                        <tr class="hover:bg-blue-50">
                            <td class="p-2"><input type="checkbox" checked /></td>
                            <td class="p-2">AR-20231020-009</td>
                            <td class="p-2 text-right font-bold">2,000.00</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>

        <!-- Middle Actions -->
        <div class="flex flex-col justify-center gap-4">
            <button class="w-12 h-12 rounded-full bg-primary text-white flex items-center justify-center shadow">
                <span class="material-symbols-outlined">link</span>
            </button>
        </div>

        <!-- Right: Receipt List -->
        <div class="flex-1 bg-white rounded border flex flex-col">
            <div class="p-3 border-b font-bold bg-gray-50">收款单据列表</div>
            <div class="flex-1 overflow-auto">
                <table class="w-full text-sm">
                    <thead class="bg-gray-50 sticky top-0">
                        <tr>
                            <th class="p-2"><input type="checkbox" /></th>
                            <th class="p-2">流水号</th>
                            <th class="p-2 text-right">可用余额</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr class="hover:bg-green-50">
                            <td class="p-2"><input type="checkbox" checked /></td>
                            <td class="p-2">RC-20231028-110</td>
                            <td class="p-2 text-right font-bold">5,000.00</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    </div>

    <div class="bg-white border-t p-3 flex justify-between items-center shrink-0">
        <div class="font-bold flex gap-4">
            <span class="text-blue-600">应收核销: 7,000.00</span>
            <span>=</span>
            <span class="text-green-600">实收核销: 7,000.00</span>
        </div>
        <button class="bg-primary text-white px-6 py-2 rounded font-bold">确认核销</button>
    </div>
  </div>
</template>
