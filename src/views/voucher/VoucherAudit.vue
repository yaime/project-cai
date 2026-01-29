<template>
  <div class="flex flex-col h-full bg-slate-100 dark:bg-[#0c101a] overflow-hidden">
    <!-- Sticky Toolbar -->
    <div class="sticky top-0 z-10 bg-white/80 dark:bg-slate-900/80 backdrop-blur-md border-b border-slate-200 dark:border-slate-800 px-6 py-4 flex flex-wrap items-center justify-between gap-4 shrink-0">
      <!-- Breadcrumbs -->
      <div class="flex items-center gap-2 text-sm">
        <a class="text-slate-500 hover:text-primary dark:text-slate-400" href="#">Finance</a>
        <span class="text-slate-300 dark:text-slate-600">/</span>
        <a class="text-slate-500 hover:text-primary dark:text-slate-400" href="#">Vouchers</a>
        <span class="text-slate-300 dark:text-slate-600">/</span>
        <span class="font-semibold text-slate-900 dark:text-white flex items-center gap-2">
          #V-2024-001
          <span class="px-2 py-0.5 rounded-full bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-400 text-xs font-medium">Pending Audit</span>
        </span>
      </div>
      <!-- Action Buttons -->
      <div class="flex items-center gap-3">
        <button class="hidden lg:flex items-center gap-2 px-4 py-2 text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-lg text-sm font-medium transition-colors border border-transparent hover:border-slate-200 dark:hover:border-slate-700">
          <span class="material-symbols-outlined text-[20px]">file_download</span>
          Export PDF
        </button>
        <div class="h-6 w-px bg-slate-300 dark:bg-slate-700 hidden lg:block mx-2"></div>
        <button class="px-4 py-2 bg-white dark:bg-transparent border border-slate-300 dark:border-slate-600 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-800 rounded-lg text-sm font-medium transition-colors shadow-sm">
          Reverse Audit
        </button>
        <button class="px-4 py-2 bg-white dark:bg-transparent border border-red-200 dark:border-red-900 text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg text-sm font-medium transition-colors shadow-sm">
          Reject
        </button>
        <button class="px-4 py-2 bg-primary hover:bg-primary-dark text-white rounded-lg text-sm font-medium transition-colors shadow-sm flex items-center gap-2">
          <span class="material-symbols-outlined text-[20px]">done_all</span>
          Batch Approve
        </button>
        <button class="px-6 py-2 bg-primary hover:bg-primary-dark text-white rounded-lg text-sm font-bold transition-colors shadow-md flex items-center gap-2">
          <span class="material-symbols-outlined text-[20px]">check</span>
          Approve
        </button>
      </div>
    </div>

    <!-- Scrollable Content -->
    <div class="flex-1 overflow-hidden flex flex-row">
      <!-- Voucher Area -->
      <div class="flex-1 overflow-y-auto p-8 lg:p-12 flex justify-center custom-scrollbar">
        <!-- The Paper Voucher -->
        <div class="relative w-full max-w-[850px] min-h-[1100px] bg-white text-slate-900 shadow-paper rounded-sm p-12 lg:p-16 flex flex-col gap-8 transition-transform">
          <!-- Paper Texture Overlay -->
          <div class="absolute inset-0 pointer-events-none opacity-[0.03] mix-blend-multiply" style="background-image: url('https://lh3.googleusercontent.com/aida-public/AB6AXuBSxvcr9I32LBT59O1A29ARst_So_s3zNAaa2StBCrfvXgkDURCi_edTEoaXcM6VAoFeuhUeKMAnDkV5R8Cc2R7thix9SmT9KxQGP1-GpdbWoLeKbpFp3ax_FUEi-S_zVjawYl8GxXZqxhKXELShN1VWaHBBsLRefDNMUBZVFKWM0z-gM8w22hAZMn10RztCo8MAe37Na1Cfjo0Vtq3wXDy3dThbw48H72PGEbQpLDmUAlnJ3DzWqnuR5KbALpIpZVoC3twhg7L3w');"></div>

          <!-- Voucher Header -->
          <div class="flex justify-between items-start border-b-2 border-slate-900 pb-8">
            <div class="flex flex-col gap-4">
              <div class="flex items-center gap-3">
                <div class="bg-slate-900 text-white p-2 rounded-sm">
                  <span class="material-symbols-outlined text-3xl">account_balance</span>
                </div>
                <div>
                  <h1 class="text-2xl font-bold uppercase tracking-wide">Acme Corp Ltd.</h1>
                  <p class="text-xs text-slate-500 uppercase tracking-widest">Financial Department</p>
                </div>
              </div>
              <div class="mt-4 text-sm text-slate-600 space-y-1">
                <p>123 Financial District Blvd</p>
                <p>New York, NY 10005</p>
                <p>Tax ID: US-882910-X</p>
              </div>
            </div>
            <div class="text-right">
              <h2 class="text-3xl font-mono font-bold text-slate-900 mb-2">PAYMENT VOUCHER</h2>
              <div class="inline-grid grid-cols-2 gap-x-4 gap-y-2 text-right text-sm">
                <span class="font-bold text-slate-500">Voucher No:</span>
                <span class="font-mono font-bold">#PV-89201</span>
                <span class="font-bold text-slate-500">Date:</span>
                <span class="font-mono">Oct 24, 2023</span>
                <span class="font-bold text-slate-500">Currency:</span>
                <span class="font-mono">USD</span>
              </div>
            </div>
          </div>

          <!-- Payee Info -->
          <div class="flex gap-8 py-4">
            <div class="w-2/3">
              <p class="text-xs font-bold text-slate-500 uppercase mb-1">Pay To</p>
              <p class="text-lg font-medium border-b border-slate-300 pb-1">Cloud Infrastructure Services Inc.</p>
            </div>
            <div class="w-1/3">
              <p class="text-xs font-bold text-slate-500 uppercase mb-1">Payment Method</p>
              <p class="text-lg font-medium border-b border-slate-300 pb-1">Wire Transfer</p>
            </div>
          </div>

          <!-- Accounting Grid -->
          <div class="flex-1">
            <table class="w-full text-sm border-collapse">
              <thead>
                <tr class="border-b-2 border-slate-900">
                  <th class="py-3 text-left font-bold uppercase w-20">Code</th>
                  <th class="py-3 text-left font-bold uppercase">Description / Particulars</th>
                  <th class="py-3 text-right font-bold uppercase w-32">Debit</th>
                  <th class="py-3 text-right font-bold uppercase w-32">Credit</th>
                </tr>
              </thead>
              <tbody class="font-mono text-slate-700">
                <tr class="border-b border-slate-200">
                  <td class="py-3 align-top">6001</td>
                  <td class="py-3 align-top">
                    <p class="font-bold text-slate-900">Server Infrastructure Q4 Hosting</p>
                    <p class="text-xs text-slate-500 mt-1">Inv #INV-2023-992, Period: Oct-Dec</p>
                  </td>
                  <td class="py-3 align-top text-right font-medium">12,500.00</td>
                  <td class="py-3 align-top text-right text-slate-400">-</td>
                </tr>
                <tr class="border-b border-slate-200">
                  <td class="py-3 align-top">6005</td>
                  <td class="py-3 align-top">
                    <p class="font-bold text-slate-900">SaaS Subscription - Monitoring Tools</p>
                    <p class="text-xs text-slate-500 mt-1">Monthly recurring fee</p>
                  </td>
                  <td class="py-3 align-top text-right font-medium">450.00</td>
                  <td class="py-3 align-top text-right text-slate-400">-</td>
                </tr>
                <tr class="border-b border-slate-200">
                  <td class="py-3 align-top">2100</td>
                  <td class="py-3 align-top">
                    <p class="font-bold text-slate-900">Accounts Payable</p>
                    <p class="text-xs text-slate-500 mt-1">Vendor Payment Clearing</p>
                  </td>
                  <td class="py-3 align-top text-right text-slate-400">-</td>
                  <td class="py-3 align-top text-right font-medium">12,950.00</td>
                </tr>
              </tbody>
              <tfoot>
                <tr class="bg-slate-50 border-t-2 border-slate-900">
                  <td class="py-3 pl-4 font-bold text-right uppercase text-slate-500 text-xs tracking-wider" colspan="2">Total Amount</td>
                  <td class="py-3 text-right font-bold font-mono text-lg text-slate-900 border-l border-r border-slate-200 bg-white">12,950.00</td>
                  <td class="py-3 text-right font-bold font-mono text-lg text-slate-900 border-r border-slate-200 bg-white">12,950.00</td>
                </tr>
              </tfoot>
            </table>
            <div class="mt-4 flex items-center gap-2 text-sm">
              <span class="font-bold text-slate-500 uppercase text-xs">Amount in words:</span>
              <span class="italic font-medium text-slate-800 border-b border-dotted border-slate-400 flex-1">Twelve Thousand Nine Hundred Fifty Dollars Only</span>
            </div>
          </div>

          <!-- Digital Stamp -->
          <div class="absolute -top-10 right-20 w-40 h-40 stamp-container rounded-full flex flex-col items-center justify-center rotate-[-12deg] opacity-80 pointer-events-none select-none z-10 bg-transparent shadow-sm backdrop-blur-[1px]">
            <div class="border-2 border-red-500 rounded-full w-[90%] h-[90%] flex flex-col items-center justify-center">
              <div class="flex items-center gap-1 text-red-500 mb-1">
                <span class="material-symbols-outlined text-2xl">verified</span>
                <span class="text-xs font-bold uppercase tracking-widest">FinLedger</span>
              </div>
              <span class="text-2xl font-black uppercase tracking-widest text-red-600">AUDITED</span>
              <span class="text-[10px] font-mono mt-1 font-bold text-red-500">2023-10-24</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Right Sidebar: Audit History -->
      <aside class="w-80 lg:w-96 bg-white dark:bg-slate-900 border-l border-slate-200 dark:border-slate-800 flex flex-col shadow-xl z-20 shrink-0">
        <div class="p-5 border-b border-slate-100 dark:border-slate-800">
          <h3 class="font-bold text-slate-900 dark:text-white flex items-center gap-2">
            <span class="material-symbols-outlined text-slate-400">history</span>
            Audit History
          </h3>
        </div>
        <div class="flex-1 overflow-y-auto p-5 custom-scrollbar">
          <div class="relative pl-4 border-l-2 border-slate-100 dark:border-slate-800 space-y-8">
            <div class="relative">
              <div class="absolute -left-[21px] top-1 h-3 w-3 rounded-full bg-slate-300 dark:bg-slate-600 ring-4 ring-white dark:ring-slate-900"></div>
              <div class="flex flex-col gap-1">
                <span class="text-xs font-mono text-slate-400">Oct 24, 10:00 AM</span>
                <div class="flex items-center gap-2">
                  <div class="size-6 rounded-full bg-blue-100 text-blue-600 flex items-center justify-center text-xs font-bold">JD</div>
                  <p class="text-sm font-medium text-slate-900 dark:text-white">Voucher Created</p>
                </div>
                <p class="text-xs text-slate-500 dark:text-slate-400">Prepared by John Doe from Accounts Payable.</p>
              </div>
            </div>
            <!-- More Timeline items -->
          </div>
        </div>
        <div class="p-5 border-t border-slate-200 dark:border-slate-800 bg-slate-50 dark:bg-slate-900/50">
          <label class="block text-xs font-bold text-slate-500 uppercase mb-2">Add Internal Note</label>
          <textarea class="w-full text-sm rounded-lg border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-800 focus:ring-primary focus:border-primary resize-none placeholder-slate-400 dark:text-white" placeholder="Type your comment or rejection reason..." rows="3"></textarea>
          <div class="flex justify-end mt-2">
            <button class="bg-slate-900 dark:bg-slate-700 text-white text-xs font-bold py-2 px-4 rounded-md hover:bg-slate-800 dark:hover:bg-slate-600 transition-colors">
              Post Comment
            </button>
          </div>
        </div>
      </aside>
    </div>
  </div>
</template>

<style scoped>
.stamp-container {
    border: 4px solid #ef4444;
    color: #ef4444;
    mask-image: url('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=');
    mix-blend-mode: multiply;
}
.dark .stamp-container {
    mix-blend-mode: normal;
    opacity: 0.8;
}
.shadow-paper {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06), 0 20px 25px -5px rgba(0, 0, 0, 0.1);
}
</style>
