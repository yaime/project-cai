<template>
  <div class="flex h-full bg-slate-100 dark:bg-background-dark overflow-hidden relative">
    <!-- Main Workspace -->
    <section class="flex-1 flex flex-col border-r border-slate-200 dark:border-slate-800 relative group/canvas">
        <div class="absolute top-6 left-1/2 -translate-x-1/2 z-20 flex items-center gap-1 p-1 bg-white/90 rounded-lg shadow border">
            <button class="p-2 hover:bg-slate-100"><span class="material-symbols-outlined">remove</span></button>
            <span class="px-2 text-xs font-bold">100%</span>
            <button class="p-2 hover:bg-slate-100"><span class="material-symbols-outlined">add</span></button>
        </div>
        <div class="flex-1 overflow-auto custom-scrollbar flex justify-center p-12 bg-[#e5e7eb] dark:bg-[#0b101a]">
            <div class="relative shadow-2xl bg-white w-[600px] min-h-[850px]">
                <div class="p-10">
                    <h1 class="text-2xl font-bold opacity-30">INVOICE PREVIEW</h1>
                    <!-- Placeholder for image -->
                    <div class="h-96 bg-gray-100 mt-10 flex items-center justify-center text-gray-400">
                        [Scanned Image Content]
                    </div>
                </div>
            </div>
        </div>
    </section>

    <!-- Sidebar -->
    <aside class="w-[400px] bg-white dark:bg-slate-900 flex flex-col shadow-2xl z-40">
        <div class="px-6 py-5 border-b flex justify-between items-center">
            <h1 class="text-xl font-bold">识别结果</h1>
            <span class="bg-purple-100 text-purple-700 px-2 py-1 rounded text-xs font-bold">AI 置信度高</span>
        </div>
        <div class="flex-1 overflow-y-auto p-6 space-y-6">
            <section>
                <h3 class="text-xs font-bold text-gray-400 mb-4 uppercase">供应商信息</h3>
                <div class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-primary mb-1">销售方名称</label>
                        <input class="w-full p-2 border rounded bg-primary/5 border-primary" value="Acme Corp" />
                    </div>
                    <div class="grid grid-cols-2 gap-4">
                        <div>
                            <label class="block text-sm text-gray-500 mb-1">发票日期</label>
                            <input class="w-full p-2 border rounded" value="2024-10-24" />
                        </div>
                        <div>
                            <label class="block text-sm text-gray-500 mb-1">总金额</label>
                            <input class="w-full p-2 border rounded font-bold" value="$5,170.00" />
                        </div>
                    </div>
                </div>
            </section>

            <section>
                <h3 class="text-xs font-bold text-gray-400 mb-4 uppercase">明细项目 (3)</h3>
                <div class="border rounded overflow-hidden">
                    <table class="w-full text-sm">
                        <thead class="bg-gray-50">
                            <tr>
                                <th class="p-2 text-left">项目</th>
                                <th class="p-2 text-right">金额</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="border-t">
                                <td class="p-2">SaaS 订阅服务</td>
                                <td class="p-2 text-right">4,500.00</td>
                            </tr>
                            <tr class="border-t">
                                <td class="p-2">数据存储</td>
                                <td class="p-2 text-right">200.00</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </section>
        </div>
        <div class="p-4 border-t flex gap-3">
            <button class="flex-1 py-2.5 border rounded font-bold hover:bg-gray-50">驳回</button>
            <button class="flex-[2] py-2.5 bg-primary text-white rounded font-bold hover:bg-blue-700">确认并处理</button>
        </div>
    </aside>
  </div>
</template>
