import { ref } from 'vue'

const toasts = ref([])
let idCounter = 0

export function useToast() {
  /**
   * Add a new toast notification.
   * @param {Object} options
   * @param {string} options.message - The message to display.
   * @param {'success'|'info'|'warning'|'error'} [options.type='info'] - The type of notification.
   * @param {number} [options.duration=3000] - Duration in ms. 0 for persistent.
   */
  const addToast = ({ message, type = 'info', duration = 3000 }) => {
    const id = idCounter++
    const toast = {
      id,
      message,
      type,
      duration
    }
    
    // Add to the beginning (top) or end? Usually stacks stack up or down.
    // Let's add to the array. The container determines rendering order.
    toasts.value.push(toast)

    if (duration > 0) {
      setTimeout(() => {
        removeToast(id)
      }, duration)
    }
  }

  const removeToast = (id) => {
    const index = toasts.value.findIndex(t => t.id === id)
    if (index !== -1) {
      toasts.value.splice(index, 1)
    }
  }

  return {
    toasts,
    addToast,
    removeToast
  }
}
