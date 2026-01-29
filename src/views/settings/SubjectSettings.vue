<template>
  <div class="flex flex-col h-full bg-background-light dark:bg-background-dark overflow-hidden">
    <!-- Header -->
    <header class="flex items-center justify-between px-6 py-3 border-b border-border-light dark:border-border-dark bg-white dark:bg-slate-900">
        <h2 class="text-lg font-bold">会计科目表</h2>
    </header>
    <div class="flex flex-1 overflow-hidden">
        <!-- Sidebar Tree -->
        <aside class="w-80 border-r border-border-light bg-white dark:bg-slate-900 flex flex-col">
            <div class="p-4 border-b border-border-light">
                <input class="w-full p-2 border rounded" placeholder="Search..." />
            </div>
            <div class="flex-1 overflow-auto p-2">
                <!-- Tree -->
                <div class="p-2 font-bold">资产</div>
                <div class="pl-4 p-1">流动资产</div>
                <div class="pl-8 p-1 text-primary">1002 银行存款</div>
            </div>
        </aside>
        <!-- Content -->
        <div class="flex-1 p-8 overflow-auto">
            <h1 class="text-2xl font-bold mb-6">银行存款</h1>
            <div class="bg-white dark:bg-slate-900 p-6 rounded shadow">
                <div class="grid grid-cols-2 gap-6">
                    <div>
                        <label class="block mb-2 font-bold">科目代码</label>
                        <input class="w-full p-2 border rounded" value="1002" />
                    </div>
                    <div>
                        <label class="block mb-2 font-bold">科目名称</label>
                        <input class="w-full p-2 border rounded" value="银行存款" />
                    </div>
                </div>
            </div>
        </div>
    </div>
  </div>
</template>
