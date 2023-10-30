import { createStore } from "solid-js/store";
import { FileSystemInfo, OpenFile, getFileSystemInfo,openFile } from "./bindings";

type Store = {
  fileSystem: FileSystemInfo,
  openedFile:Array<OpenFile>
}

export const invokeChangeDir = async (dir: string | null) => {
  const files = await catchIfAny(getFileSystemInfo(dir));
  if(files) setStore("fileSystem",files);
}

export const invokeOpenFile = async (path: string) => {
  const file = await catchIfAny(openFile(path));
  if (file) setStore('openedFile', prev => [...prev, file]);
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

const [store, setStore] = createStore<Store>({
  fileSystem: {
    current_directory: "",
    directory_items : []
  },
  openedFile:[]
});

export { store }

