<script lang="js" setup>
const props = defineProps({
  name: {
    type: String,
    required: true,
  },
});
</script>

<template>
  <svg
    class="icon"
    aria-hidden="true"
    style="width: 1em; height: 1em; fill: currentColor">
    <use :xlink:href="`#icon-${name}`"></use>
  </svg>
</template>
