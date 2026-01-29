<template>
  <div class="flex flex-col h-full bg-white dark:bg-slate-900 p-4">
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-xl font-bold">Chart of Accounts</h2>
      <n-button type="primary" @click="openCreateModal">Add Subject</n-button>
    </div>

    <n-data-table
      :columns="columns"
      :data="treeData"
      :row-key="(row) => row.code"
      default-expand-all
      class="flex-1"
      flex-height
    />

    <n-modal v-model:show="showModal" preset="card" :title="isEdit ? 'Edit Subject' : 'Add Subject'" class="w-[600px]">
      <n-form
        ref="formRef"
        :model="formModel"
        :rules="rules"
        label-placement="left"
        label-width="120"
      >
        <n-form-item label="Code" path="code">
          <n-input v-model:value="formModel.code" :disabled="isEdit" placeholder="e.g. 1001" />
        </n-form-item>
        <n-form-item label="Name" path="name">
          <n-input v-model:value="formModel.name" placeholder="Subject Name" />
        </n-form-item>
        <n-form-item label="Direction" path="direction">
          <n-select v-model:value="formModel.direction" :options="directionOptions" />
        </n-form-item>
        <n-form-item label="Type" path="subject_type">
          <n-select v-model:value="formModel.subject_type" :options="typeOptions" />
        </n-form-item>
        <n-form-item label="Parent" path="parent_id">
             <n-tree-select
                v-model:value="formModel.parent_id"
                :options="treeData"
                key-field="code"
                label-field="name"
                placeholder="Select Parent"
                clearable
            />
        </n-form-item>
         <n-form-item label="Auxiliary" path="is_auxiliary">
          <n-switch v-model:value="formModel.is_auxiliary" />
        </n-form-item>
      </n-form>
      <template #footer>
        <div class="flex justify-end gap-2">
            <n-button @click="showModal = false">Cancel</n-button>
            <n-button type="primary" @click="handleSubmit">Save</n-button>
        </div>
      </template>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { h, ref, onMounted, computed } from 'vue';
import { NDataTable, NButton, NModal, NForm, NFormItem, NInput, NSelect, NTreeSelect, NSwitch, useMessage, useDialog, type DataTableColumns, type FormInst } from 'naive-ui';
import { invoke } from '@tauri-apps/api/core';

interface Subject {
  code: string;
  name: string;
  direction: string;
  type: string;
  is_auxiliary: boolean;
  parent_id: string | null;
  children?: Subject[];
  [key: string]: any;
}

const subjects = ref<Subject[]>([]);
const showModal = ref(false);
const isEdit = ref(false);
const formRef = ref<FormInst | null>(null);
const message = useMessage();
const dialog = useDialog();

const formModel = ref({
    code: '',
    name: '',
    direction: 'Debit',
    subject_type: 'Asset',
    is_auxiliary: false,
    parent_id: null as string | null
});

const directionOptions = [
    { label: 'Debit', value: 'Debit' },
    { label: 'Credit', value: 'Credit' }
];

const typeOptions = [
    { label: 'Asset', value: 'Asset' },
    { label: 'Liability', value: 'Liability' },
    { label: 'Equity', value: 'Equity' },
    { label: 'Cost', value: 'Cost' },
    { label: 'ProfitLoss', value: 'ProfitLoss' }
];

const rules = {
    code: { required: true, message: 'Code is required', trigger: 'blur' },
    name: { required: true, message: 'Name is required', trigger: 'blur' },
    direction: { required: true, message: 'Direction is required', trigger: 'blur' },
    subject_type: { required: true, message: 'Type is required', trigger: 'blur' }
};

const treeData = computed(() => {
    // Deep copy to avoid modifying original subjects when building tree
    const data = JSON.parse(JSON.stringify(subjects.value));
    const map = new Map();
    const roots: Subject[] = [];

    data.forEach((item: Subject) => {
        item.children = [];
        map.set(item.code, item);
    });

    data.forEach((item: Subject) => {
        if (item.parent_id && map.has(item.parent_id)) {
            map.get(item.parent_id).children!.push(item);
        } else {
            roots.push(item);
        }
    });

    // Function to remove empty children arrays
    const cleanChildren = (nodes: Subject[]) => {
        nodes.forEach(node => {
            if (node.children && node.children.length === 0) {
                delete node.children;
            } else if (node.children) {
                cleanChildren(node.children);
            }
        });
    };
    cleanChildren(roots);

    return roots;
});

const columns: DataTableColumns<Subject> = [
    { title: 'Code', key: 'code', width: 150 },
    { title: 'Name', key: 'name', width: 200 },
    { title: 'Direction', key: 'direction', width: 100 },
    { title: 'Type', key: 'type', width: 100 },
    { title: 'Auxiliary', key: 'is_auxiliary', width: 100, render: (row) => row.is_auxiliary ? 'Yes' : 'No' },
    {
        title: 'Actions',
        key: 'actions',
        render(row) {
            return h('div', { class: 'flex gap-2' }, [
                h(NButton, {
                    size: 'small',
                    onClick: () => openEditModal(row)
                }, { default: () => 'Edit' }),
                h(NButton, {
                    size: 'small',
                    type: 'error',
                    onClick: () => handleDelete(row)
                }, { default: () => 'Delete' })
            ]);
        }
    }
];

const fetchSubjects = async () => {
    try {
        subjects.value = await invoke('get_all_subjects');
    } catch (e) {
        message.error('Failed to fetch subjects: ' + e);
    }
};

const openCreateModal = () => {
    isEdit.value = false;
    formModel.value = {
        code: '',
        name: '',
        direction: 'Debit',
        subject_type: 'Asset',
        is_auxiliary: false,
        parent_id: null
    };
    showModal.value = true;
};

const openEditModal = (row: Subject) => {
    isEdit.value = true;
    formModel.value = {
        code: row.code,
        name: row.name,
        direction: row.direction,
        subject_type: row.type,
        is_auxiliary: row.is_auxiliary,
        parent_id: row.parent_id
    };
    showModal.value = true;
};

const handleSubmit = (e: MouseEvent) => {
    e.preventDefault();
    formRef.value?.validate(async (errors) => {
        if (!errors) {
            const { subject_type, ...rest } = formModel.value;
            const payload = {
                ...rest,
                type: subject_type
            };

            try {
                if (isEdit.value) {
                    await invoke('update_subject', { args: payload });
                    message.success('Subject updated');
                } else {
                    await invoke('create_subject', { args: payload });
                    message.success('Subject created');
                }
                showModal.value = false;
                fetchSubjects();
            } catch (err) {
                message.error('Operation failed: ' + err);
            }
        }
    });
};

const handleDelete = (row: Subject) => {
    dialog.warning({
        title: 'Confirm Delete',
        content: `Are you sure you want to delete ${row.name}?`,
        positiveText: 'Delete',
        negativeText: 'Cancel',
        onPositiveClick: async () => {
            try {
                await invoke('delete_subject', { code: row.code });
                message.success('Subject deleted');
                fetchSubjects();
            } catch (err) {
                message.error('Failed to delete: ' + err);
            }
        }
    });
};

onMounted(() => {
    fetchSubjects();
});
</script>
