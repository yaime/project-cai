<template>
  <div class="flex flex-col h-full bg-background-light dark:bg-background-dark items-center py-10 px-4">
    <!-- Stepper -->
    <div class="w-full max-w-4xl mb-10 flex justify-between items-center relative">
        <div class="bg-primary text-white w-10 h-10 rounded-full flex items-center justify-center z-10">1</div>
        <div class="bg-white border-2 w-10 h-10 rounded-full flex items-center justify-center z-10">2</div>
        <div class="bg-white border-2 w-10 h-10 rounded-full flex items-center justify-center z-10">3</div>
        <div class="absolute w-full h-1 bg-gray-200 top-5 left-0"></div>
    </div>

    <!-- Form -->
    <div class="w-full max-w-3xl bg-white dark:bg-slate-900 rounded-xl shadow p-8 space-y-6">
        <h1 class="text-2xl font-bold">账套基本信息设置</h1>
        <div class="space-y-2">
            <label class="font-bold">账套名称</label>
            <input class="w-full p-3 border rounded-lg" placeholder="请输入公司名称" />
        </div>
        <div class="grid grid-cols-2 gap-6">
            <div class="space-y-2">
                <label class="font-bold">本位币</label>
                <select class="w-full p-3 border rounded-lg"><option>CNY</option></select>
            </div>
            <div class="space-y-2">
                <label class="font-bold">启用期间</label>
                <input type="month" class="w-full p-3 border rounded-lg" value="2023-10" />
            </div>
        </div>
        <div class="flex justify-end pt-4">
            <button class="bg-primary text-white px-8 py-2.5 rounded-lg font-bold">下一步</button>
        </div>
    </div>
  </div>
</template>
