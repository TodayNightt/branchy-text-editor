import { createStore } from "solid-js/store";
import {
  FileSystemInfo,
  OpenFile,
  getFileSystemInfo,
  openFile,
  closeFile,
  reset,
  getEditorConfig,
  Theme,
  LanguageTheme,
  EditorTheme,
} from "./bindings";
import { catchIfAny } from "./invocation";

type EditorConfig = {
  editorTheme: EditorTheme;
  languageTheme: LanguageTheme;
};

type Store = {
  fileSystem: FileSystemInfo;
  openedFile: Array<OpenFile>;
  selectedFile: string;
  editorConfig?: EditorConfig;
};

const [store, setStore] = createStore<Store>({
  fileSystem: {
    current_directory: "",
    directory_items: [],
  },
  openedFile: [],
  selectedFile: "",
  editorConfig: undefined,
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

export const invokeGetEditorConfig = async () => {
  let editorConfig = await catchIfAny(getEditorConfig());
  let languageTheme = editorConfig?.[0];
  let editorTheme = editorConfig?.[1];
  setStore("editorConfig", { editorTheme, languageTheme });
};

export const getLanguageThemeIfAnyElseDefault = (language: string): Theme => {
  const languageTheme: LanguageTheme = store.editorConfig?.languageTheme!;
  // @ts-ignore
  if (languageTheme[language]) {
    // @ts-ignore
    return languageTheme[language];
  }
  return languageTheme.default;
};

export { store };
