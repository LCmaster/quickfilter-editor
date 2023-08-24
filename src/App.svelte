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
  {:then _}
    <ImageUploader
      on:edit={(ev) => {
        console.log("Time to edit!");
        imageToEdit = ev.detail;
      }}
    />
    {#if imageToEdit}
      <ImageEditor bind:image={imageToEdit} />
    {/if}
  {/await}
</main>
