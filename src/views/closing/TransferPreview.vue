<template>
  <div class="flex items-center justify-center min-h-full p-8 bg-gray-100">
    <div class="bg-white w-full max-w-5xl shadow-lg border p-8 relative min-h-[600px]">
        <div class="text-center mb-8 border-b-4 border-double border-slate-300 pb-2">
            <h1 class="text-3xl font-bold tracking-widest">记账凭证</h1>
        </div>

        <div class="flex justify-between mb-2 text-sm">
            <span>核算单位: 杭州未来科技有限公司</span>
            <div class="flex gap-4">
                <span>日期: 2023-10-31</span>
                <span>字号: 转-001</span>
            </div>
        </div>

        <div class="border-2 border-primary">
            <table class="w-full text-center">
                <thead class="bg-gray-50 border-b">
                    <tr>
                        <th class="p-2 border-r w-1/4">摘要</th>
                        <th class="p-2 border-r">科目</th>
                        <th class="p-2 border-r w-1/4">借方</th>
                        <th class="p-2 w-1/4">贷方</th>
                    </tr>
                </thead>
                <tbody>
                    <tr class="h-12 border-b">
                        <td class="border-r">结转本期损益</td>
                        <td class="border-r">主营业务收入</td>
                        <td class="border-r font-mono text-right pr-2">10,000.00</td>
                        <td class="font-mono text-right pr-2"></td>
                    </tr>
                    <tr class="h-12 border-b">
                        <td class="border-r">结转本期损益</td>
                        <td class="border-r">本年利润</td>
                        <td class="border-r font-mono text-right pr-2"></td>
                        <td class="font-mono text-right pr-2">10,000.00</td>
                    </tr>
                </tbody>
            </table>
        </div>

        <div class="flex justify-between mt-8 text-sm">
            <span>制单: 系统自动生成</span>
            <span>审核: [待审核]</span>
        </div>

        <div class="absolute top-10 right-10 opacity-20 rotate-12 border-4 border-red-500 rounded-full w-24 h-24 flex items-center justify-center text-red-500 font-bold text-xs">
            PREVIEW
        </div>
    </div>
  </div>
</template>
