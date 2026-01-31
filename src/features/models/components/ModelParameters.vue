<template>
  <div class="bg-background-surface border border-border rounded-lg shadow-sm overflow-hidden transition-all duration-300">
    <div class="px-4 py-3 border-b border-border bg-background-element font-semibold text-sm flex justify-between items-center text-text-main">
      <span class="flex items-center gap-2">
        <Settings2 class="w-4 h-4" /> {{ $t('models.parameters.title') }}
      </span>
    </div>

    <div class="p-4 space-y-4">
      <!-- Common Parameters -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <!-- Context Window -->
        <div class="space-y-2">
          <div class="flex items-center justify-between px-1">
            <label class="text-[13px] font-semibold text-text-main">
              {{ $t('models.parameters.ctx') }}
            </label>
            <span class="text-[10px] text-text-sub font-mono opacity-50">num_ctx</span>
          </div>
          <div class="relative group">
            <input 
              v-model.number="params.num_ctx"
              type="number"
              class="custom-number-input w-full bg-background-element border border-border/50 rounded-lg px-4 py-2 text-sm text-text-main focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all duration-200"
              @change="updateParams"
            >
          </div>
        </div>

        <!-- Max Prediction -->
        <div class="space-y-2">
          <div class="flex items-center justify-between px-1">
            <label class="text-[13px] font-semibold text-text-main">
              {{ $t('models.parameters.predict') }}
            </label>
            <span class="text-[10px] text-text-sub font-mono opacity-50">num_predict</span>
          </div>
          <div class="relative group">
            <input 
              v-model.number="params.num_predict"
              type="number"
              class="custom-number-input w-full bg-background-element border border-border/50 rounded-lg px-4 py-2 text-sm text-text-main focus:border-primary focus:ring-4 focus:ring-primary/10 outline-none transition-all duration-200"
              placeholder="-1"
              @change="updateParams"
            >
          </div>
        </div>
      </div>

      <!-- Advanced Toggle -->
      <div class="pt-2">
        <button 
          class="flex items-center gap-2 text-xs font-bold text-primary/80 hover:text-primary transition-colors select-none group"
          @click="showAdvanced = !showAdvanced"
        >
          <div class="w-5 h-5 rounded-full bg-primary/5 flex items-center justify-center group-hover:bg-primary/10 transition-colors">
            <ChevronRight 
              class="w-3 h-3 transition-transform duration-300"
              :class="{ 'rotate-90': showAdvanced }"
            />
          </div>
          <span class="uppercase tracking-wider">{{ $t('models.parameters.advanced') }}</span>
        </button>
      </div>

      <!-- Advanced Parameters (Collapsible) -->
      <div 
        v-show="showAdvanced"
        class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 pt-4 animate-in fade-in slide-in-from-top-2 duration-300"
      >
        <!-- Temperature -->
        <div class="space-y-2">
          <div class="flex items-center justify-between px-1">
            <label class="text-[12px] font-medium text-text-sub">
              {{ $t('models.parameters.temperature') }}
            </label>
            <span class="text-[10px] text-text-sub/40 font-mono">temp</span>
          </div>
          <input 
            v-model.number="params.temperature"
            type="number"
            step="0.1"
            class="custom-number-input w-full bg-background-element border border-border/50 rounded-lg px-3 py-2 text-sm text-text-main focus:border-primary/50 focus:ring-4 focus:ring-primary/5 outline-none transition-all duration-200"
            @change="updateParams"
          >
        </div>

        <!-- Top K -->
        <div class="space-y-2">
          <div class="flex items-center justify-between px-1">
            <label class="text-[12px] font-medium text-text-sub">
              {{ $t('models.parameters.topK') }}
            </label>
            <span class="text-[10px] text-text-sub/40 font-mono">top_k</span>
          </div>
          <input 
            v-model.number="params.top_k"
            type="number"
            class="custom-number-input w-full bg-background-element border border-border/50 rounded-lg px-3 py-2 text-sm text-text-main focus:border-primary/50 focus:ring-4 focus:ring-primary/5 outline-none transition-all duration-200"
            @change="updateParams"
          >
        </div>

        <!-- Top P -->
        <div class="space-y-2">
          <div class="flex items-center justify-between px-1">
            <label class="text-[12px] font-medium text-text-sub">
              {{ $t('models.parameters.topP') }}
            </label>
            <span class="text-[10px] text-text-sub/40 font-mono">top_p</span>
          </div>
          <input 
            v-model.number="params.top_p"
            type="number"
            step="0.1"
            class="custom-number-input w-full bg-background-element border border-border/50 rounded-lg px-3 py-2 text-sm text-text-main focus:border-primary/50 focus:ring-4 focus:ring-primary/5 outline-none transition-all duration-200"
            @change="updateParams"
          >
        </div>

        <!-- Repeat Penalty -->
        <div class="space-y-2">
          <div class="flex items-center justify-between px-1">
            <label class="text-[12px] font-medium text-text-sub">
              {{ $t('models.parameters.repeatPenalty') }}
            </label>
            <span class="text-[10px] text-text-sub/40 font-mono">penalty</span>
          </div>
          <input 
            v-model.number="params.repeat_penalty"
            type="number"
            step="0.1"
            class="custom-number-input w-full bg-background-element border border-border/50 rounded-lg px-3 py-2 text-sm text-text-main focus:border-primary/50 focus:ring-4 focus:ring-primary/5 outline-none transition-all duration-200"
            @change="updateParams"
          >
        </div>

        <!-- Seed -->
        <div class="space-y-2">
          <div class="flex items-center justify-between px-1">
            <label class="text-[12px] font-medium text-text-sub">
              {{ $t('models.parameters.seed') }}
            </label>
            <span class="text-[10px] text-text-sub/40 font-mono">seed</span>
          </div>
          <input 
            v-model.number="params.seed"
            type="number"
            class="custom-number-input w-full bg-background-element border border-border/50 rounded-lg px-3 py-2 text-sm text-text-main focus:border-primary/50 focus:ring-4 focus:ring-primary/5 outline-none transition-all duration-200"
            placeholder="-1"
            @change="updateParams"
          >
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, watch } from 'vue'
import { useSettingsStore } from '../../../store/settings'
import { Settings2, ChevronRight } from 'lucide-vue-next'

const settingsStore = useSettingsStore()
const showAdvanced = ref(false)

// Local state for inputs to avoid stuttering, sync on change
const params = reactive({ ...settingsStore.generationParameters })

// Watch for external changes (e.g. persistence load)
watch(() => settingsStore.generationParameters, (newVal) => {
  Object.assign(params, newVal)
}, { deep: true })

const updateParams = () => {
  settingsStore.updateGenerationParameters(params)
}
</script>

<style scoped>
/* Hide native spin buttons for a cleaner look */
.custom-number-input::-webkit-outer-spin-button,
.custom-number-input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

.custom-number-input {
  -moz-appearance: textfield;
}
</style>