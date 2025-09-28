<script>
  import { onMount } from "svelte";
  import { backend } from "$lib/canisters";

  export let selectedPerson;

  let todos = [];
  let newTodoText = "";
  let error = null;

  // Fetch to-dos whenever selectedPerson changes
  $: if (selectedPerson) {
    loadTodos();
  }

  async function loadTodos() {
    if (!selectedPerson) return;

    const res = await backend.list_todos({ person_id: selectedPerson.id });

    if ("Ok" in res) {
      todos = res.Ok;
      error = null;
    } else {
      error = res.Err;
      todos = [];
    }
  }

  async function addTodo() {
    const res = await backend.new_todo({
      text: newTodoText,
      done: false,
      person_id: selectedPerson.id
    });

    if ("Ok" in res) {
      todos = [...todos, res.Ok];
      newTodoText = "";
    } else {
      error = res.Err;
    }
  }

  async function doTodo(todo) {
    const res = await backend.update_todo({
      id: todo.id,
      text: todo.text,
      done: true
    });

    if ("Ok" in res) {
      todos = todos.map(t => t.id === todo.id ? res.Ok : t);
    }
  }

  async function deleteTodo(todo) {
    const res = await backend.delete_todo(todo.id);
    if ("Ok" in res) {
      todos = todos.filter(t => t.id !== todo.id);
    }
  }
</script>

{#if !selectedPerson}
  <p>Select a person to see their todos.</p>
{:else}
  {#if error}
    <p style="color:red">Error: {error}</p>
  {/if}

  <form on:submit|preventDefault={addTodo}>
    <input
      type="text"
      placeholder="To-do..."
      bind:value={newTodoText}
    />
    <button type="submit" >Add To-do</button>
  </form>

  {#if todos.length === 0}
    <p>No todos yet.</p>
  {:else}
  <table>
    <thead>
      <tr>
        <th>To-do</th>
        <th></th>
      </tr>
    </thead>
    <tbody>
      {#each todos as todo}
         <tr>
          <td class:done={todo.done}>{todo.text}</td>
          <td>          
            <button   class="small"
              type="button"
              on:click={() => doTodo(todo)}
              style:visibility={todo.done ? 'hidden' : 'visible'}>
              done
            </button>
            <button class="small" on:click={() => deleteTodo(todo)}>❌</button>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
  {/if}
{/if}


<style>
  .done {
    text-decoration: line-through;
    color: gray;
  }

  th:last-child, td:last-child {
    width: 1%;
    white-space: nowrap;
  }

</style>