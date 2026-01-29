<template>
  <div class="flex flex-col h-full bg-background-light dark:bg-background-dark p-6 overflow-hidden">
    <div class="max-w-[1280px] mx-auto w-full flex flex-col gap-6 h-full">
        <!-- Header -->
        <div class="flex justify-between items-center shrink-0">
            <div>
                <h1 class="text-2xl font-bold">期末结账向导</h1>
                <p class="text-gray-500 text-sm">2023年第10期</p>
            </div>
            <button class="bg-gray-100 px-4 py-2 rounded font-bold">退出向导</button>
        </div>

        <!-- Stepper -->
        <div class="bg-white dark:bg-slate-900 p-6 rounded-xl border shadow-sm shrink-0">
            <div class="flex justify-between items-center px-10">
                <div class="flex flex-col items-center gap-2">
                    <div class="w-8 h-8 rounded-full bg-primary text-white flex items-center justify-center font-bold">1</div>
                    <span class="text-xs font-bold text-primary">凭证检查</span>
                </div>
                <div class="h-0.5 flex-1 bg-gray-200 mx-4"></div>
                <div class="flex flex-col items-center gap-2 opacity-50">
                    <div class="w-8 h-8 rounded-full border-2 flex items-center justify-center font-bold">2</div>
                    <span class="text-xs">账簿核对</span>
                </div>
                <!-- ... other steps ... -->
            </div>
        </div>

        <!-- Content -->
        <div class="flex flex-1 gap-6 overflow-hidden">
            <div class="flex-1 flex flex-col bg-white dark:bg-slate-900 rounded-xl border shadow-sm overflow-hidden">
                <div class="p-6 border-b flex justify-between items-center bg-gray-50 dark:bg-slate-800">
                    <h3 class="font-bold text-lg">凭证完整性检查</h3>
                    <button class="bg-primary text-white px-4 py-2 rounded text-sm font-bold flex items-center gap-2">
                        <span class="material-symbols-outlined text-sm">sync</span> 开始检查
                    </button>
                </div>
                <div class="p-6 flex-1 overflow-y-auto space-y-4">
                    <div class="flex items-center justify-between p-4 border rounded-lg bg-green-50 border-green-100">
                        <div class="flex items-center gap-4">
                            <span class="material-symbols-outlined text-green-600">check_circle</span>
                            <div>
                                <div class="font-bold">未记账凭证</div>
                                <div class="text-xs text-gray-500">检查是否存在已录入但未记账的凭证</div>
                            </div>
                        </div>
                        <span class="text-green-700 font-bold text-sm">通过</span>
                    </div>

                    <div class="flex items-center justify-between p-4 border rounded-lg bg-red-50 border-red-100">
                        <div class="flex items-center gap-4">
                            <span class="material-symbols-outlined text-red-600">error</span>
                            <div>
                                <div class="font-bold">未审核凭证</div>
                                <div class="text-xs text-gray-500">检查是否存在未审核的凭证</div>
                            </div>
                        </div>
                        <div class="flex items-center gap-4">
                            <span class="text-red-700 font-bold text-sm">2 张</span>
                            <button class="text-primary text-sm font-bold underline">查看</button>
                        </div>
                    </div>
                </div>
            </div>

            <aside class="w-80 flex flex-col gap-6 shrink-0">
                <div class="bg-white dark:bg-slate-900 p-5 rounded-xl border shadow-sm">
                    <h3 class="font-bold mb-4">结账状态</h3>
                    <div class="bg-gray-50 p-3 rounded mb-2">
                        <div class="text-xs text-gray-500">当前期间</div>
                        <div class="flex justify-between items-center">
                            <span class="font-bold text-lg">2023-10</span>
                            <span class="bg-blue-100 text-blue-800 text-xs px-2 py-0.5 rounded-full">进行中</span>
                        </div>
                    </div>
                </div>
            </aside>
        </div>

        <!-- Footer -->
        <div class="bg-white dark:bg-slate-900 p-4 border-t shadow-sm flex justify-between shrink-0 rounded-xl">
            <button class="px-6 py-2 border rounded text-gray-400" disabled>上一步</button>
            <button class="px-6 py-2 bg-primary/50 text-white rounded font-bold cursor-not-allowed" disabled>下一步</button>
        </div>
    </div>
  </div>
</template>
