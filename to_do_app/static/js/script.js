const todoInput = document.getElementById('todo-input');
const todoList = document.getElementById('todo-list');

async function showTodos(){
    const response = await fetch('/api/todos');
    const todos = await response.json();
    render(todos);

}

async function addTodo() {
    const task = todoInput.value;
    if (!task) return;

    const newTodo = {
        id: Date.now(),
        task: task,
        completed: false
    };

    const response = await fetch('/api/todos', {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify(newTodo)
    });

    if (response.ok) {
        todoInput.value = '';
        showTodos();
    }
}

function render(todos) {
    todoList.innerHTML = '';
    todos.forEach(todo => {
        const li = document.createElement('li');
        li.textContent = todo.task;
        if(todo.completed) li.classList.add('completed');
        todoList.appendChild(li);
    });
}

async function toggleTodo(id) {
    await fetch(`/api/todos/${id}/toggle`, { method: 'POST' });
    loadTodos();
}
function render(todos) {
    const todoList = document.getElementById('todo-list');
    todoList.innerHTML = '';
    
    todos.forEach(todo => {
        const li = document.createElement('li');
        if (todo.completed) li.classList.add('completed');

        li.innerHTML = `
            <span>${todo.task}</span>
            <button onclick="toggleTodo(${todo.id})">
                ${todo.completed ? 'Undo' : 'Done'}
            </button>
        `;
        todoList.appendChild(li);
    });
}

showTodos();