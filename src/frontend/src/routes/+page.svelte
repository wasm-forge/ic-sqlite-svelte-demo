<script>
  import "../index.scss";
  import TodoList from '$lib/components/TodoList.svelte';
  import Persons from "$lib/components/Persons.svelte";
  import { backend } from "$lib/canisters";

  let selectedPerson = null;
  let personsRef;

  async function resetDatabase() {
    try {
      await backend.reset_base();
      selectedPerson = null; 

      if (personsRef?.loadPersons) {
        await personsRef.loadPersons();
      }

    } catch (err) {
      console.error(err);
    }
  }

</script>

<main>
  <img src="/logo2.svg" alt="DFINITY logo" />
  <br /><br />

  <div style="width: 75%; margin: 10px auto; margin-bottom: 20px;">
    Original source: <a href="https://github.com/wasm-forge/ic-sqlite-svelte-demo">GitHub repository</a>.
    <br />

    <section>
      <h1>📋 Persons & To-dos</h1>
      <button on:click={resetDatabase}>Reset Database</button>
    </section>

    <section>
      <h2>Persons</h2>
      <Persons bind:this={personsRef} bind:selectedPerson />
    </section>

    <section>
      {#if selectedPerson}
        <h2>{selectedPerson.name}'s To-dos</h2>
        <TodoList {selectedPerson} />
      {:else}
        <p>Select a person to see their to-dos.</p>
      {/if}
    </section>

  </div>

</main>