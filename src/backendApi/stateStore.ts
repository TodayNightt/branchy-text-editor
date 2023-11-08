import { createStore } from "solid-js/store";
import {
  FileSystemInfo,
  OpenFile,
  getFileSystemInfo,
  openFile,
  closeFile,
  reset,
} from "./bindings";
import { catchIfAny } from "./invocation";



type Store = {
  fileSystem: FileSystemInfo;
  openedFile: Array<OpenFile>;
  selectedFile: string;
};

const [store, setStore] = createStore<Store>({
  fileSystem: {
    current_directory: "",
    directory_items: [],
  },
  openedFile: [],
  selectedFile: "",
});

export const invokeChangeDir = async (dir: string | null) => {
  const files = await catchIfAny(getFileSystemInfo(dir));
  if (files) {
    setStore("fileSystem", files);
  }
};

export const invokeOpenFile = async (path: string) => {
  const exist = store.openedFile.find((item) => item.path == path);
  if (exist) {
    setStore("selectedFile", exist.name);
    return;
  }
  const file = await catchIfAny(openFile(path));
  if (file) {
    setStore("openedFile", (prev) => [...prev, file]);
    setStore("selectedFile", file.name);
  }
};

export const changeSelected = (value: string) => {
  setStore("selectedFile", value);
};

export const invokeCloseFile = async (id: number) => {
  await catchIfAny(closeFile(id));
  setStore("openedFile", (prev) => prev.filter((item) => item.id != id));
};

export const invokeReset = async () => {
  await catchIfAny(reset());
  setStore("openedFile", []);
};


export { store };
