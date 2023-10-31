import { createStore } from "solid-js/store";
import { FileSystemInfo, OpenFile, getFileSystemInfo,openFile ,getSourceCodeIfAny, closeFile} from "./bindings";

type Store = {
  fileSystem: FileSystemInfo,
  openedFile: Array<OpenFile>
  selectedFile: string,
}

const [store, setStore] = createStore<Store>({
  fileSystem: {
    current_directory: "",
    directory_items : []
  },
  openedFile: [],
  selectedFile:'',
});

export const invokeGetSourceCode = async (id: number): Promise<string> => {
  const code = await catchIfAny(getSourceCodeIfAny(id));
  if (code) return code;
  return ""
}

export const invokeChangeDir = async (dir: string | null) => {
  const files = await catchIfAny(getFileSystemInfo(dir));
  if (files) {
    setStore("fileSystem", files);
  }
}

export const invokeOpenFile = async (path: string) => {
  const exist = store.openedFile.find(item=> item.path == path);
  if (exist) {
    setStore("selectedFile", exist.name);
    return;
  }
  const file = await catchIfAny(openFile(path));
  if (file) {
    setStore('openedFile', prev => [...prev, file]);
    setStore("selectedFile", file.name);
  }
}

export const changeSelected = (value: string) => {
  setStore("selectedFile", value);
}

export const invokeCloseFile = async (id: number) => {
  await catchIfAny(closeFile(id));
  setStore("openedFile", prev => prev.filter(item => item.id != id));
}


//https://gist.github.com/karlhorky/3593d8cd9779cf9313f9852c59260642
export async function catchIfAny<T>(promise: Promise<T>): Promise<T | null>  {
  try {
    return await promise;
  } catch (error) {
    console.log(error)
  }
  return null;
}


export { store }

