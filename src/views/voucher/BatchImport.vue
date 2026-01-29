<template>
  <div class="flex flex-col h-full bg-background-light p-8 items-center overflow-y-auto">
    <div class="w-full max-w-5xl">
        <div class="flex justify-center mb-10">
            <div class="flex items-center gap-4">
                <div class="w-10 h-10 rounded-full bg-green-500 text-white flex items-center justify-center font-bold">1</div>
                <div class="w-20 h-1 bg-green-500"></div>
                <div class="w-10 h-10 rounded-full bg-primary text-white flex items-center justify-center font-bold">2</div>
                <div class="w-20 h-1 bg-gray-200"></div>
                <div class="w-10 h-10 rounded-full bg-white border-2 flex items-center justify-center font-bold text-gray-400">3</div>
            </div>
        </div>

        <h2 class="text-3xl font-black mb-8 text-center">Map Your Columns</h2>

        <div class="grid grid-cols-3 gap-8">
            <div class="col-span-2 space-y-6">
                <div class="bg-white rounded-xl shadow border p-6">
                    <h3 class="font-bold mb-6">Field Mapping</h3>
                    <div class="space-y-4">
                        <div class="flex items-center gap-4 p-3 bg-gray-50 rounded">
                            <div class="w-1/3 font-bold">Transaction Date *</div>
                            <div class="text-gray-400">-></div>
                            <select class="flex-1 p-2 border rounded border-green-500 bg-green-50 text-green-700 font-bold"><option>Date</option></select>
                        </div>
                        <div class="flex items-center gap-4 p-3 bg-white rounded border hover:bg-gray-50">
                            <div class="w-1/3 font-bold">Summary</div>
                            <div class="text-gray-400">-></div>
                            <select class="flex-1 p-2 border rounded"><option>Description</option></select>
                        </div>
                    </div>
                </div>
            </div>

            <div class="col-span-1">
                <div class="bg-white rounded-xl shadow border p-6 sticky top-4">
                    <h4 class="font-bold mb-4">Mapping Status</h4>
                    <div class="w-full bg-gray-100 rounded-full h-2 mb-4">
                        <div class="bg-green-500 h-2 rounded-full" style="width: 100%"></div>
                    </div>
                    <button class="w-full bg-primary text-white py-3 rounded font-bold shadow">Next: Preview</button>
                </div>
            </div>
        </div>
    </div>
  </div>
</template>
