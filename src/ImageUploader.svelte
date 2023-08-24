<script>
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  const fileReader = new FileReader();

  fileReader.onloadend = () => {
    const data = fileReader.result
      .toString()
      .replace(/^data:image\/(png|jpeg|jpg);base64,/, "");
    dispatch("edit", data);
  };

  function submitImageToEdit(el) {
    if (el.target.files.length) {
      fileReader.readAsDataURL(el.target.files[0]);
    }
  }
</script>

<div class="md:w-3/4 mx-auto p-2">
  <div class="flex flex-col gap-4">
    <h2 class="font-bold text-3xl">Image Effects</h2>
    <p class="text-zinc-500">
      Need to do some basic image manipulation? Just upload your image below.
      We'll handle the rest.
    </p>
    <label
      class="py-3 px-6 font-bold text-white bg-purple-800 rounded-md cursor-pointer hover:bg-purple-600"
    >
      <input
        accept="image/png, image/jpeg"
        on:change={submitImageToEdit}
        type="file"
        hidden
      />
      Upload PNG image
    </label>
  </div>
</div>
