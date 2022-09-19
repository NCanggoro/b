import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Todos, ITodosDisplay, IAllTodos } from '../types';

function App() { 
  const [todos, setTodos] = useState<Todos>({
    ongoing: [],
    done: [],
    allTodos: {
      ongoing: [],
      done: []
    }
  })
  const [newTodo, setNewTodo] = useState("")
  const listInputRef = useRef(null)

  const greet = async () => {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    await invoke("greet")
      .then((res: IAllTodos) => {
        let ongoing: Array<ITodosDisplay> = [];
        let done: Array<ITodosDisplay> = [];

        res.ongoing.forEach(x => {
          ongoing.push({
            title: x,
            isEdit: false
          })
        })

        res.done.forEach(x => {
          done.push({
            title: x,
            isEdit: false
          })
        })

        setTodos({ ongoing: ongoing, done: done, allTodos: { ongoing: res.ongoing, done: res.done } })
      }).catch((err) => {
        console.log(err)
      })
  }

  const addTodo = async () => {
    if(newTodo == "") return
    await invoke("add_todo", { todo: newTodo})
      .then(_ => {
        setNewTodo("");
        greet()
      }).catch(err => {
        console.log(err)
      })
  }

  const editTodo = async (i: number, type: string) => {
    if(type == "ongoing") {
      todos.allTodos.ongoing[i] = todos.ongoing[i].title
    } else {
      todos.allTodos.done[i] = todos.done[i].title
    }
    await invoke("edit_todo", { todos: todos.allTodos })
      .then(_ => {
        greet()
      }).catch(err => {
        alert(err);
      })

  }

  const  deleteTodo = async (i: number, type: string) => {
    let ongoingArr: Array<string> = [];
    let doneArr: Array<string> = [];
    if (type == "ongoing") {
      ongoingArr = todos.allTodos.ongoing.filter((_, index) => index != i);
      doneArr = todos.allTodos.done
    } else {
      ongoingArr = todos.allTodos.ongoing;
      doneArr = todos.allTodos.done.filter((_, index) => index != i);
    }

    let todoList = { ongoing: ongoingArr, done: doneArr };

    await invoke("delete_todo", { todos: todoList }).then(_ => {
      greet()
    }).catch(err => {
      alert(err)
    });
  }

  const doneTodo = async (todo_title: string, i: number) => {
    await invoke("done_todo", { 
      lists: todos.allTodos.ongoing, 
      index: i, 
      title: todo_title
    }).then(_ => {
      greet()
    }).catch(err => {
      alert(err)  
    })
  }

  const enableEditButton = (i: number, type: string) => {
    listInputRef.current.focus();
    if(type == "ongoing") {
      todos.ongoing[i].isEdit = true
      setTodos(prevState => ({ ongoing: todos.ongoing,  ...prevState }))
    } else {
      todos.done[i].isEdit = true
      setTodos(prevState => ({ done: todos.ongoing,  ...prevState }))
    }
    
  } 

  const cancelEditButton = (i: number, type: string) => {
    if(type == "ongoing") {
      todos.ongoing[i].isEdit = false
      todos.ongoing[i].title = todos.allTodos.ongoing[i]
      setTodos(prevState => ({ ongoing: todos.ongoing,  ...prevState }))
    } else {
      todos.done[i].isEdit = false
      todos.done[i].title = todos.allTodos.done[i]
      setTodos(prevState => ({ done: todos.done,  ...prevState }))
    }
    
  } 

  const onChangeTodoTitle = (title: string, type: string, index: number) => {
    if(type == "ongoing") {
      todos.ongoing[index].title = title;
      setTodos(prevState=> ({
        ongoing: todos.ongoing,
        ...prevState
      }))
    } else {
      todos.done[index].title = title;
      setTodos(prevState=> ({
        done: todos.ongoing,
        ...prevState
      }))
    }
  }

  const list = (list: Array<ITodosDisplay>, type: string) => {
    let lists = list.map((todo, i) => {
      return (
        <div style={{ display: 'flex'}}>
          <input 
            ref={listInputRef} 
            onChange={(e) => onChangeTodoTitle(e.currentTarget.value, type, i)}
            value={todo.title} 
            disabled={!todo.isEdit} 
          />
          <button className="list-button" type="button" onClick={() => !todo.isEdit ? enableEditButton(i, type) : editTodo(i, type)}>
            edit
          </button>
          { !todo.isEdit &&
            <>
              { type === "ongoing" &&
                <button className="list-button" type="button" onClick={() => doneTodo(todo.title, i)}>
                  done
                </button>
              }
            </>
          }
            <button className="list-button" type="button" onClick={() => todo.isEdit ? cancelEditButton(i, type) : deleteTodo(i, type)}>
              {todo.isEdit ? "cancel" : "delete"}
            </button>
        </div>
      )
    })

    return lists
  }

  useEffect(() => {
    console.log('halo')
    greet()
  }, [])

  return (
    <div className="container">
      <div className="row">
          <input
            id="greet-input"
            onChange={(e) => setNewTodo(e.currentTarget.value)}
            placeholder="Enter a name..."
            value={newTodo}
          />
          <button className="input-button" type="button" onClick={() => addTodo()}>
            add
          </button>
      </div>
      <div style={{ backgroundColor: 'green', margin: '20px 35px 0 35px', display: 'flex', justifyContent: 'space-around'}}>
        <div>
          <h2>Ongoing</h2>
          {list(todos.ongoing, "ongoing")}
        </div>
        <div>
          <h2>Done</h2>
          {list(todos.done, "done")}
        </div>
      </div>
    </div>
  );
}

export default App;
