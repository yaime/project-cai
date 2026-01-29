<template>
  <div class="flex flex-col h-full bg-background-light dark:bg-background-dark overflow-y-auto custom-scrollbar p-6 md:px-10 md:py-8">
    <div class="max-w-[1280px] mx-auto w-full">
      <div class="flex flex-wrap justify-between items-end gap-4 mb-6">
        <div>
          <h1 class="text-3xl font-bold text-slate-900 dark:text-white mb-2">新增记账凭证</h1>
          <p class="text-slate-500 dark:text-slate-400 text-sm">创建并记录总账财务交易。</p>
        </div>
        <div class="flex gap-3">
          <button class="flex items-center justify-center h-10 px-5 rounded-lg bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 text-slate-700 dark:text-slate-200 font-medium text-sm hover:bg-slate-50 dark:hover:bg-slate-700 transition-colors shadow-sm">
            取消
          </button>
          <button class="flex items-center justify-center gap-2 h-10 px-6 rounded-lg bg-primary text-white font-bold text-sm hover:bg-blue-700 transition-colors shadow-sm">
            <span class="material-symbols-outlined text-[20px]">save</span>
            保存凭证
          </button>
        </div>
      </div>

      <div class="bg-white dark:bg-[#151c2a] rounded-xl shadow-lg border border-slate-200 dark:border-slate-700 overflow-visible flex flex-col">
        <div class="p-6 md:p-8 border-b border-slate-200 dark:border-slate-700 bg-white dark:bg-[#151c2a]">
          <div class="flex flex-col md:flex-row md:items-start justify-between gap-6 mb-8">
            <div class="flex items-center gap-4">
              <div class="size-12 bg-blue-50 dark:bg-blue-900/20 rounded-lg flex items-center justify-center text-primary">
                <span class="material-symbols-outlined text-[28px]">receipt_long</span>
              </div>
              <div>
                <h3 class="text-lg font-bold text-slate-900 dark:text-white">记账凭证</h3>
                <p class="text-sm text-slate-500">手工录入</p>
              </div>
            </div>
            <div class="flex items-center gap-2 px-4 py-2 bg-slate-50 dark:bg-slate-800 rounded border border-slate-200 dark:border-slate-700">
              <span class="text-xs font-bold text-slate-400 uppercase tracking-wider">凭证字号</span>
              <span class="text-lg font-mono font-bold text-slate-900 dark:text-white">JV-2023-001</span>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
            <label class="flex flex-col gap-2">
              <span class="text-xs font-semibold text-slate-500 uppercase tracking-wider">业务日期</span>
              <div class="relative">
                <span class="material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-slate-400 text-[18px]">calendar_today</span>
                <input class="w-full pl-10 pr-3 py-2 bg-slate-50 dark:bg-slate-900 border border-slate-300 dark:border-slate-600 rounded-md text-sm text-slate-900 dark:text-white focus:ring-primary focus:border-primary" type="date" value="2023-10-27"/>
              </div>
            </label>
            <label class="flex flex-col gap-2">
              <span class="text-xs font-semibold text-slate-500 uppercase tracking-wider">成本中心</span>
              <select class="w-full pl-3 pr-8 py-2 bg-slate-50 dark:bg-slate-900 border border-slate-300 dark:border-slate-600 rounded-md text-sm text-slate-900 dark:text-white focus:ring-primary focus:border-primary">
                <option>总部 (HQ)</option>
                <option>销售部</option>
                <option>技术部</option>
              </select>
            </label>
            <label class="flex flex-col gap-2">
              <span class="text-xs font-semibold text-slate-500 uppercase tracking-wider">参考号</span>
              <input class="w-full px-3 py-2 bg-slate-50 dark:bg-slate-900 border border-slate-300 dark:border-slate-600 rounded-md text-sm text-slate-900 dark:text-white focus:ring-primary focus:border-primary" placeholder="例如：INV-2023-999" type="text"/>
            </label>
            <div class="flex flex-col gap-2">
              <span class="text-xs font-semibold text-slate-500 uppercase tracking-wider">附件</span>
              <button class="w-full flex items-center justify-between px-3 py-2 bg-white dark:bg-slate-800 border border-dashed border-primary/50 rounded-md text-primary hover:bg-blue-50 dark:hover:bg-blue-900/10 transition-colors group">
                <span class="text-sm font-medium flex items-center gap-2">
                  <span class="material-symbols-outlined text-[18px]">attach_file</span>
                  2个附件
                </span>
                <span class="material-symbols-outlined text-[16px] opacity-0 group-hover:opacity-100 transition-opacity">add</span>
              </button>
            </div>
          </div>
        </div>

        <div class="flex-1 w-full min-h-[500px] bg-white dark:bg-[#151c2a] p-0 flex flex-col">
           <ag-grid-vue
              class="ag-theme-quartz flex-1 w-full"
              :rowData="rowData"
              :columnDefs="columnDefs"
              :defaultColDef="defaultColDef"
              :pinnedBottomRowData="pinnedBottomRowData"
              @grid-ready="onGridReady"
              @cell-value-changed="onCellValueChanged"
              @cell-key-down="onCellKeyDown"
           >
           </ag-grid-vue>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { AgGridVue } from 'ag-grid-vue3';
import "ag-grid-community/styles/ag-grid.css";
import "ag-grid-community/styles/ag-theme-quartz.css";
import { invoke } from '@tauri-apps/api/core';

interface Subject {
  code: string;
  name: string;
}

interface RowData {
  summary: string;
  subject: string;
  debit: number | null;
  credit: number | null;
}

const rowData = ref<RowData[]>([
  { summary: '10月办公室房租', subject: '6602 - 管理费用', debit: 5000.00, credit: null },
  ...Array(10).fill(null).map(() => ({ summary: '', subject: '', debit: null, credit: null }))
]);

const columnDefs = ref<any[]>([]);
const defaultColDef = {
  flex: 1,
  editable: true,
  resizable: true,
};
const pinnedBottomRowData = ref<any[]>([
    { summary: 'Total', debit: 0, credit: 0 }
]);

const gridApi = ref(null);

const onGridReady = (params: any) => {
  gridApi.value = params.api;
  calculateTotals();
};

const subjects = ref<string[]>([]);

onMounted(async () => {
  try {
    const result = await invoke<Subject[]>('get_all_subjects');
    subjects.value = result.map(s => `${s.code} - ${s.name}`);

    columnDefs.value = [
      {
        headerName: '#',
        valueGetter: "node.rowIndex + 1",
        width: 50,
        editable: false,
        pinned: 'left'
      },
      { field: 'summary', headerName: '摘要', minWidth: 200 },
      {
        field: 'subject',
        headerName: '会计科目',
        cellEditor: 'agSelectCellEditor',
        cellEditorParams: {
            values: subjects.value
        },
        minWidth: 200
      },
      {
        field: 'debit',
        headerName: '借方金额',
        type: 'numericColumn',
        valueFormatter: (params: any) => params.value ? params.value.toFixed(2) : '',
        valueParser: (params: any) => Number(params.newValue)
      },
      {
        field: 'credit',
        headerName: '贷方金额',
        type: 'numericColumn',
        valueFormatter: (params: any) => params.value ? params.value.toFixed(2) : '',
        valueParser: (params: any) => Number(params.newValue)
      }
    ];
  } catch (e) {
    console.error('Failed to load subjects', e);
  }
});

const calculateTotals = () => {
    let totalDebit = 0;
    let totalCredit = 0;

    // We can use gridApi to iterate or rowData if they are synced
    // gridApi provides access to filtered/sorted data which might be better
    // but rowData is reactive.
    // AG Grid updates the objects inside rowData ref.

    rowData.value.forEach(row => {
        if (row.debit) totalDebit += Number(row.debit);
        if (row.credit) totalCredit += Number(row.credit);
    });

    pinnedBottomRowData.value = [{
        summary: 'Total',
        subject: '',
        debit: totalDebit,
        credit: totalCredit
    }];
};

const onCellValueChanged = () => {
    calculateTotals();
};

const onCellKeyDown = (e: any) => {
    const event = e.event;
    if (event.key === '=') {
        const colId = e.column.colId;
        if (colId === 'debit' || colId === 'credit') {
            event.preventDefault();

            let totalDebit = 0;
            let totalCredit = 0;

             if (gridApi.value) {
                (gridApi.value as any).forEachNode((node: any) => {
                    if (node.data && node.rowIndex !== e.node.rowIndex) {
                         totalDebit += Number(node.data.debit || 0);
                         totalCredit += Number(node.data.credit || 0);
                    }
                });
             }

             let balance = 0;
             if (colId === 'debit') {
                 balance = totalCredit - totalDebit;
             } else {
                 balance = totalDebit - totalCredit;
             }

             if (balance < 0) balance = 0;

             e.node.setDataValue(colId, balance);
             // Ensure totals update
             calculateTotals();
        }
    }
};

</script>
