<script>
  import { grayscale } from "../wasm/pkg/wasm";
  export let image = null;

  let result;
  $: result = grayscale(image);
</script>

<div class="flex justify-center">
  {#if result}
    <img src={result} alt="result after edition" />
  {/if}
</div>
