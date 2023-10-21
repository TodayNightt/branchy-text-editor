import { createStore } from "solid-js/store";

type File = {
    id:number,
    name :string,
    source_code: string,
}



const [store, setStore] = createStore({
    opened: Array<File>,
}
);


export { store, setStore };


