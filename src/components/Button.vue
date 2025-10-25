<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  addClass?: string
  variant?: 'neutral' | 'primary' | 'secondary' | 'accent' | 'info' | 'success' | 'warning' | 'error'
  styling?: 'outline' | 'ghost' | 'link' | 'dash' | 'soft' | string
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
}

const props = withDefaults(defineProps<Props>(), {
  addClass: '',
  variant: 'info',
  styling: '',
  size: 'sm'
})

const selectedVariant = computed(() => props.variant ?? 'info')
const selectedStyling = computed(() => props.styling ?? '')
const selectedSize = computed(() => props.size ?? 'sm')

const BASE_BTN_CLASSES = 'btn text-xs py-4 px-8 focus:outline-none rounded-xl' as const

const SIZE_CLASSES = {
  xs: 'btn-xs',
  sm: 'btn-sm',
  md: 'btn-md',
  lg: 'btn-lg',
  xl: 'btn-xl',
} as const

const VARIANT_CLASSES = {
  neutral: 'btn-neutral',
  primary: 'btn-primary',
  secondary: 'btn-secondary',
  accent: 'btn-accent',
  info: 'btn-info',
  success: 'btn-success',
  warning: 'btn-warning',
  error: 'btn-error',
} as const

const STYLING_CLASSES = {
  outline: 'btn-outline',
  ghost: 'btn-ghost',
  link: 'btn-link',
  dash: 'btn-dash',
  soft: 'btn-soft',
} as const

const variantClass = computed(() => VARIANT_CLASSES[selectedVariant.value as keyof typeof VARIANT_CLASSES])
const stylingClass = computed(() => STYLING_CLASSES[selectedStyling.value as keyof typeof STYLING_CLASSES])
const sizeClass = computed(() => SIZE_CLASSES[selectedSize.value as keyof typeof SIZE_CLASSES])
const additionalClasses = computed(() => stylingClass.value === undefined ? 'text-white' : 'hover:text-white')
</script>

<template>
  <button :class="[BASE_BTN_CLASSES, sizeClass, variantClass, props.addClass, stylingClass, additionalClasses]">
    <slot />
  </button>
</template>

<style>
@import "tailwindcss";
@plugin "daisyui";
</style>