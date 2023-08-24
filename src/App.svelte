<script>
  import init from "../wasm/pkg/wasm";
  import AppLoader from "./AppLoader.svelte";
  import ImageEditor from "./ImageEditor.svelte";
  import ImageUploader from "./ImageUploader.svelte";

  let imageToEdit;
</script>

<main>
  {#await init()}
    <AppLoader />
  {:then editor}
    {#if imageToEdit}
      <ImageEditor {editor} image={imageToEdit} />
    {:else}
      <ImageUploader on:edit={(ev) => (imageToEdit = ev.detail)} />
    {/if}
  {/await}
</main>
