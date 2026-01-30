<script setup lang="ts">
import { ref } from 'vue';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

interface OcrResult {
  text: string;
  confidence: number;
  coordinates: number[][];
}

const imagePath = ref('');
const result = ref<OcrResult | null>(null);
const isLoading = ref(false);
const error = ref('');

async function uploadImage() {
    try {
        const file = await open({
            multiple: false,
            filters: [{
                name: 'Image',
                extensions: ['png', 'jpg', 'jpeg', 'bmp', 'webp']
            }]
        });

        if (file) {
            imagePath.value = file as string;
            await runOcr();
        }
    } catch (err: any) {
        error.value = err.toString();
        console.error(err);
    }
}

async function runOcr() {
    if (!imagePath.value) return;
    isLoading.value = true;
    error.value = '';
    result.value = null;
    try {
        const res = await invoke<OcrResult>('perform_ocr', { path: imagePath.value });
        result.value = res;
    } catch (err: any) {
        error.value = 'OCR Failed: ' + err.toString();
        console.error(err);
    } finally {
        isLoading.value = false;
    }
}
</script>

<template>
  <div class="flex h-full bg-slate-100 dark:bg-background-dark overflow-hidden relative">
    <!-- Main Workspace -->
    <section class="flex-1 flex flex-col border-r border-slate-200 dark:border-slate-800 relative group/canvas">
        <div class="absolute top-6 left-1/2 -translate-x-1/2 z-20 flex items-center gap-1 p-1 bg-white/90 rounded-lg shadow border">
            <button class="p-2 hover:bg-slate-100" @click="uploadImage" :disabled="isLoading">
                <span class="material-symbols-outlined">{{ isLoading ? 'hourglass_empty' : 'upload' }}</span>
                <span class="ml-1 text-xs font-bold">Upload</span>
            </button>
        </div>
        <div class="flex-1 overflow-auto custom-scrollbar flex justify-center p-12 bg-[#e5e7eb] dark:bg-[#0b101a]">
            <div class="relative shadow-2xl bg-white w-[600px] min-h-[850px]">
                <div class="p-10">
                    <h1 class="text-2xl font-bold opacity-30">INVOICE PREVIEW</h1>
                    <!-- Image Display -->
                    <div class="h-auto min-h-96 bg-gray-100 mt-10 flex flex-col items-center justify-center text-gray-400 overflow-hidden relative rounded border-2 border-dashed border-gray-300">
                         <img v-if="imagePath" :src="convertFileSrc(imagePath)" class="max-w-full max-h-[600px] object-contain" alt="Scanned Image" />
                         <div v-else class="p-10 text-center cursor-pointer" @click="uploadImage">
                            <span class="material-symbols-outlined text-4xl mb-2">image</span>
                            <p>Click to Upload Image</p>
                         </div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <!-- Sidebar -->
    <aside class="w-[400px] bg-white dark:bg-slate-900 flex flex-col shadow-2xl z-40">
        <div class="px-6 py-5 border-b flex justify-between items-center">
            <h1 class="text-xl font-bold">识别结果 (OCR)</h1>
            <span v-if="result" class="bg-purple-100 text-purple-700 px-2 py-1 rounded text-xs font-bold">
                Conf: {{ (result.confidence * 100).toFixed(1) }}%
            </span>
        </div>

        <div v-if="error" class="p-4 bg-red-100 text-red-700 m-4 rounded text-sm break-words">
            {{ error }}
        </div>

        <div class="flex-1 overflow-y-auto p-6 space-y-6">
            <section v-if="result">
                <h3 class="text-xs font-bold text-gray-400 mb-4 uppercase">Extracted Text</h3>
                <div class="p-4 bg-gray-50 border rounded font-mono text-sm break-all">
                    {{ result.text || 'No text detected' }}
                </div>

                <h3 class="text-xs font-bold text-gray-400 mt-6 mb-4 uppercase">Coordinates</h3>
                 <div class="p-4 bg-gray-50 border rounded font-mono text-xs text-gray-600">
                    {{ JSON.stringify(result.coordinates) }}
                </div>

                <h3 class="text-xs font-bold text-gray-400 mt-6 mb-4 uppercase">Raw Data</h3>
                <pre class="text-xs bg-gray-900 text-white p-4 rounded overflow-auto h-40">{{ JSON.stringify(result, null, 2) }}</pre>
            </section>

            <section v-else>
                 <div class="text-center py-10 opacity-50">
                    <span class="material-symbols-outlined text-4xl mb-2">text_fields</span>
                    <p class="text-sm">Upload an image to see results here.</p>
                 </div>
            </section>
        </div>
        <div class="p-4 border-t flex gap-3">
             <button class="flex-1 py-2.5 bg-primary text-white rounded font-bold hover:bg-blue-700" @click="uploadImage">New Scan</button>
        </div>
    </aside>
  </div>
</template>
