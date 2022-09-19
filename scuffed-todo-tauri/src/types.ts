export interface IAllTodos {
  ongoing: Array<string>,
  done: Array<string>
}

export interface ITodosDisplay {
  title: string;
  isEdit: boolean;
}

export type Todos = {
  ongoing: Array<ITodosDisplay>
  done: Array<ITodosDisplay>
  allTodos : IAllTodos
}

export type TodosDisplay = {
  ongoing: Array<ITodosDisplay>,
  done: Array<ITodosDisplay>
}