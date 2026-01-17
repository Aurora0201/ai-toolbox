import { ref, reactive } from 'vue'

const state = reactive({
  show: false,
  title: '',
  message: '',
  confirmText: 'Confirm',
  cancelText: 'Cancel',
  resolve: null
})

/**
 * Composable for managing a global confirmation dialog.
 * Returns a promise that resolves to true on confirm and false on cancel.
 */
export function useConfirm() {
  const confirm = (options = {}) => {
    state.title = options.title || 'Confirm Action'
    state.message = options.message || 'Are you sure?'
    state.confirmText = options.confirmText || 'Confirm'
    state.cancelText = options.cancelText || 'Cancel'
    state.show = true

    return new Promise((resolve) => {
      state.resolve = resolve
    })
  }

  const onConfirm = () => {
    state.show = false
    if (state.resolve) state.resolve(true)
  }

  const onCancel = () => {
    state.show = false
    if (state.resolve) state.resolve(false)
  }

  return {
    state,
    confirm,
    onConfirm,
    onCancel
  }
}
