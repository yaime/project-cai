import { createRouter, createWebHistory } from 'vue-router'
import MainLayout from '../layouts/MainLayout.vue'
import Dashboard from '../views/Dashboard.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    // Setup Wizard Routes (Standalone)
    {
      path: '/setup/basic',
      name: 'setup-basic',
      component: () => import('../views/setup/SetupBasic.vue'),
      meta: { title: 'Setup - Basic Info' }
    },
    {
      path: '/setup/opening',
      name: 'setup-opening',
      component: () => import('../views/setup/SetupOpening.vue'),
      meta: { title: 'Setup - Opening Balances' }
    },
    {
      path: '/setup/subject',
      name: 'setup-subject',
      component: () => import('../views/setup/SetupSubject.vue'),
      meta: { title: 'Setup - Subjects' }
    },
    // Main App Routes
    {
      path: '/',
      component: MainLayout,
      redirect: '/dashboard',
      children: [
        {
          path: 'dashboard',
          name: 'dashboard',
          component: Dashboard,
          meta: { title: 'Dashboard' }
        },
        // Ledger
        {
          path: 'ledger/summary',
          name: 'ledger-summary',
          component: () => import('../views/ledger/SubsidiaryLedgerSummary.vue'),
          meta: { title: 'Subsidiary Ledger Summary' }
        },
        {
          path: 'ledger/detail',
          name: 'ledger-detail',
          component: () => import('../views/ledger/SubsidiaryLedgerDetail.vue'),
          meta: { title: 'Subsidiary Ledger Detail' }
        },
        {
          path: 'ledger/query',
          name: 'ledger-query',
          component: () => import('../views/ledger/SubsidiaryLedgerQuery.vue'),
          meta: { title: 'Ledger Query' }
        },
        {
          path: 'ledger/verification',
          name: 'ledger-verification',
          component: () => import('../views/ledger/Verification.vue'),
          meta: { title: 'Verification' }
        },
        {
          path: 'ledger/auxiliary-modal',
          name: 'ledger-auxiliary-modal',
          component: () => import('../views/ledger/AuxiliaryModalDemo.vue'),
          meta: { title: 'Auxiliary Modal Demo' }
        },
        // Voucher
        {
          path: 'voucher/entry',
          name: 'voucher-entry',
          component: () => import('../views/voucher/VoucherEntry.vue'),
          meta: { title: 'Voucher Entry' }
        },
        {
          path: 'voucher/audit',
          name: 'voucher-audit',
          component: () => import('../views/voucher/VoucherAudit.vue'),
          meta: { title: 'Voucher Audit' }
        },
        {
          path: 'voucher/journal',
          name: 'journal-entry',
          component: () => import('../views/voucher/JournalEntry.vue'),
          meta: { title: 'Journal Entry' }
        },
        {
          path: 'voucher/batch-audit',
          name: 'batch-audit',
          component: () => import('../views/voucher/BatchAudit.vue'),
          meta: { title: 'Batch Audit' }
        },
        {
          path: 'voucher/batch-audit-en',
          name: 'batch-audit-en',
          component: () => import('../views/voucher/BatchAuditEn.vue'),
          meta: { title: 'Batch Audit (EN)' }
        },
        {
          path: 'voucher/batch-import',
          name: 'batch-import',
          component: () => import('../views/voucher/BatchImport.vue'),
          meta: { title: 'Batch Import' }
        },
        {
          path: 'voucher/ai-invoice',
          name: 'ai-invoice',
          component: () => import('../views/voucher/AIInvoice.vue'),
          meta: { title: 'AI Invoice' }
        },
        // Assets
        {
          path: 'assets/list',
          name: 'assets-list',
          component: () => import('../views/assets/FixedAssetsList.vue'),
          meta: { title: 'Fixed Assets' }
        },
        {
          path: 'assets/depreciation',
          name: 'assets-depreciation',
          component: () => import('../views/assets/AssetDepreciation.vue'),
          meta: { title: 'Asset Depreciation' }
        },
        // Cashier
        {
          path: 'cashier/statement',
          name: 'bank-statement',
          component: () => import('../views/cashier/BankStatement.vue'),
          meta: { title: 'Bank Statement' }
        },
        {
          path: 'cashier/reconciliation',
          name: 'bank-reconciliation',
          component: () => import('../views/cashier/BankReconciliation.vue'),
          meta: { title: 'Bank Reconciliation' }
        },
        {
          path: 'cashier/adjustment',
          name: 'balance-adjustment',
          component: () => import('../views/cashier/BalanceAdjustment.vue'),
          meta: { title: 'Balance Adjustment' }
        },
        // Closing
        {
          path: 'closing/check',
          name: 'period-end-check',
          component: () => import('../views/closing/PeriodEndCheck.vue'),
          meta: { title: 'Period End Check' }
        },
        {
          path: 'closing/trial-balance',
          name: 'trial-balance',
          component: () => import('../views/closing/TrialBalance.vue'),
          meta: { title: 'Trial Balance' }
        },
        {
          path: 'closing/transfer',
          name: 'auto-transfer',
          component: () => import('../views/closing/AutoTransferSettings.vue'),
          meta: { title: 'Auto Transfer' }
        },
        {
          path: 'closing/transfer-preview',
          name: 'transfer-preview',
          component: () => import('../views/closing/TransferPreview.vue'),
          meta: { title: 'Transfer Preview' }
        },
        // Reports
        {
          path: 'report/monthly',
          name: 'monthly-report',
          component: () => import('../views/report/MonthlyReport.vue'),
          meta: { title: 'Monthly Report' }
        },
        // Settings
        {
          path: 'settings/subject',
          name: 'subject-settings',
          component: () => import('../views/settings/SubjectSettings.vue'),
          meta: { title: 'Subject Settings' }
        }
      ]
    }
  ]
})

export default router
