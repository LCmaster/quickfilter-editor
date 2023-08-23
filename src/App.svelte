<script>
  import init, { greet } from "../wasm/pkg/wasm";
  init().then(() => greet());
</script>

<main>
  <h1>Hello World!</h1>
</main>
