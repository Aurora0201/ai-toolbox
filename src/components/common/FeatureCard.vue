<template>
  <div
    class="feature-card panel clickable"
    @click="$emit('click')"
  >
    <div class="panel-body">
      <div class="card-icon">
        <slot name="icon" />
      </div>
      <div class="card-content">
        <h3 class="card-title">
          {{ title }}
        </h3>
        <p class="card-description">
          {{ description }}
        </p>
      </div>
      <div class="card-arrow">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="20"
          height="20"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        ><line
          x1="5"
          y1="12"
          x2="19"
          y2="12"
        /><polyline points="12 5 19 12 12 19" /></svg>
      </div>
    </div>
  </div>
</template>

<script setup>
/**
 * Common component for feature cards displayed on the home page.
 */
defineProps({
  title: {
    type: String,
    required: true
  },
  description: {
    type: String,
    required: true
  }
})

defineEmits(['click'])
</script>

<style scoped>
.feature-card {
  transition: all 0.3s cubic-bezier(0.25, 0.8, 0.25, 1);
  border: 1px solid var(--border-color);
  height: 100%;
}

.feature-card:hover {
  transform: translateY(-4px);
  border-color: var(--primary-color);
  box-shadow: var(--shadow-md);
}

.panel-body {
  display: flex;
  align-items: center;
  gap: 20px;
  padding: 24px;
}

.card-icon {
  font-size: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 56px;
  height: 56px;
  background-color: var(--bg-secondary);
  border-radius: var(--radius-md);
}

.card-content {
  flex: 1;
}

.card-title {
  margin: 0 0 4px 0;
  font-size: 18px;
  color: var(--text-primary);
}

.card-description {
  margin: 0;
  color: var(--text-secondary);
  font-size: 14px;
  line-height: 1.4;
}

.card-arrow {
  color: var(--text-muted);
  transition: transform 0.2s ease;
}

.feature-card:hover .card-arrow {
  color: var(--primary-color);
  transform: translateX(4px);
}

.clickable {
  cursor: pointer;
}
</style>
