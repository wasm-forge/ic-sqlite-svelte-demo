<script>
  import { onMount } from "svelte";
  import { backend } from "$lib/canisters";

  export let persons = [];  
  export let autoLoad = true; 
  export let selectedPerson = null;

  let newPerson = {
    name: "",
    occupation: "",
    email: ""
  };

  let error = null;

  onMount(async () => {
    if (autoLoad) {
      loadPersons()
    }
  });

  export async function loadPersons() {
    const res = await backend.get_persons();
    if ("Ok" in res) {
      persons = res.Ok;
      error = null;
    } else {
      error = res.Err;
    }
  }

  function selectPerson(person) {
    selectedPerson = person;
  }

  async function deletePerson(person) {
    const res = await backend.delete_person(person.id);
    if ("Ok" in res) {
      persons = persons.filter(t => t.id !== person.id);
      selectedPerson = null;
    }
  }

  async function addPerson() {
    if (!newPerson.name.trim()) return;

    const res = await backend.new_person(newPerson);
    if ("Ok" in res) {
      persons = [...persons, res.Ok];
      newPerson = { name: "", occupation: "", email: "" };
      error = null;
    } else {
      error = res.Err;
    }
  }
</script>

{#if error}
  <p style="color:red">Error: {error}</p>
{/if}

<form on:submit|preventDefault={addPerson} style="margin-bottom:1rem">
  <input
    type="text"
    placeholder="Name"
    bind:value={newPerson.name}
    required
  />
  <input
    type="text"
    placeholder="Occupation"
    bind:value={newPerson.occupation}
  />
  <input
    type="email"
    placeholder="Email"
    bind:value={newPerson.email}
  />
  <button type="submit">Add Person</button>
</form>

{#if persons.length === 0}
  <p>No persons found.</p>
{:else}
  <table>
    <thead>
      <tr>
        <th>Name</th>
        <th>Occupation</th>
        <th>E-mail</th>
        <th></th>
      </tr>
    </thead>
    <tbody>
      {#each persons as person}
         <tr class:selected={person === selectedPerson} on:click={() => selectPerson(person)}>
          <td>{person.name}</td>
          <td>{person.occupation}</td>
          <td>{person.email}</td>
          <td><button class="small" on:click={() => deletePerson(person)}>❌</button></td>
        </tr>
      {/each}

    </tbody>
  </table>
{/if}

<style>
  .selected {
    background-color: lightgray;
  }

  th:last-child, td:last-child {
    width: 1%;
    white-space: nowrap;
  }
</style>