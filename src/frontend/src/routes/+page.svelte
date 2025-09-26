<script>
  import "../index.scss";
  import { backend } from "$lib/canisters";
  import Hello from "$lib/components/Hello.svelte"
  import Todo from '$lib/components/Todo.svelte';

  let greeting = "";

  function onSubmit(event) {
    const name = event.target.name.value;
    backend.greet(name).then((response) => {
      greeting = response;
    });
    return false;
  }



  async function sayHello(event) {
    const name = event.target.form.name.value;
    greeting = await backend.greet(name);
  }

  async function sayGoodbye(event) {
    const name = event.target.form.name.value;
    greeting = await backend.goodbye(name);
  }  

  let persons = [
    { name: "Alice", age: 24, role: "Developer" },
    { name: "Bob", age: 30, role: "Designer" },
    { name: "Charlie", age: 28, role: "Manager" }
  ];



  let lastId = 0;

  // This is a function that creates a todo object.
  const createTodo = (text, done = false) => ({id: ++lastId, text, done});

  let todoText = '';

  // The app starts with two todos already created.
  let todos = [
    createTodo('learn Svelte', true),
    createTodo('build a Svelte app')
  ];

  // This is recomputed when the todos array changes.
  $: uncompletedCount = todos.filter(t => !t.done).length;

  // This is recomputed when the todos array or uncompletedCount changes.
  $: status = `${uncompletedCount} of ${todos.length} remaining`;

  function addTodo() {
		todos.push(createTodo(todoText));
		todos = todos; // triggers reactivity
    todoText = ''; // clears the input
  }

  // This keeps all the todos that are not done.
  const archiveCompleted = () => (todos = todos.filter(t => !t.done));

  // This deletes the todo with a given id.
  const deleteTodo = todoId => (todos = todos.filter(t => t.id !== todoId));

  function toggleDone(todo) {
    const {id} = todo;
    todos = todos.map(t => (t.id === id ? {...t, done: !t.done} : t));
  }


</script>

<main>
  <img src="/logo2.svg" alt="DFINITY logo" />
  <br />
  <br />
  <form action="#" on:submit|preventDefault>
    <label for="name">Enter your name: &nbsp;</label>
    <input id="name" alt="Name" type="text" />

    <button type="button" on:click={sayHello}>Say Hello</button>
    <button type="button" on:click={sayGoodbye}>Say Goodbye</button>  
  </form>
  <section id="greeting">{greeting}</section>

  <h1>Welcome to My Canister Frontend</h1>
  <Hello name="DFINITY" />

  <table>
    <thead>
      <tr>
        <th>Name</th>
        <th>Age</th>
        <th>Role</th>
      </tr>
    </thead>
    <tbody>
      {#each persons as person}
        <tr>
          <td>{person.name}</td>
          <td>{person.age}</td>
          <td>{person.role}</td>
        </tr>
      {/each}
    </tbody>
  </table>

<div>
	<h1>
		To Do List
	</h1>
  <div>
    {status}
    <button on:click={archiveCompleted}>Archive Completed</button>
  </div>
  <form on:submit|preventDefault={addTodo}>
    <input size="30"
      placeholder="enter new todo here"
      bind:value={todoText} />
    <button disabled={!todoText}>Add</button>
  </form>
  <ul>
    {#each todos as todo}
      <Todo
        {todo}
        on:delete={() => deleteTodo(todo.id)}
        on:toggleDone={() => toggleDone(todo)} />
    {/each}
  </ul>
</div>
	

</main>





