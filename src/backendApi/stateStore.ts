import { createStore } from "solid-js/store";
import {
  FileSystemInfo,
  OpenFile,
    commands,
  Theme,
  LanguageTheme,
  EditorTheme,
  Lang
} from "./bindings";
import { catchResponse,catchIfAny } from "./invocation";
import * as monaco from "monaco-editor/esm/vs/editor/editor.api";

type EditorConfig = {
  editorTheme: EditorTheme;
  languageTheme: LanguageTheme;
};

export type OpenFileTab = {
  fileInfo: OpenFile;
  editor: monaco.editor.IStandaloneCodeEditor | null;
};

type Store = {
  fileSystem: FileSystemInfo;
  openedFile: Array<OpenFileTab>;
  selectedFile: string;
  editorConfig?: EditorConfig;
  supportedLanguage: Array<Lang>;
};

const [store, setStore] = createStore<Store>({
  fileSystem: {
    current_directory: "",
    directory_items: [],
  },
  openedFile: [],
  selectedFile: "",
  editorConfig: undefined,
  supportedLanguage: [],
});

export const invokeChangeDir = async (dir: string | null) => {
  const files = await catchResponse(commands.getFileSystemInfo(dir));
  if (files) {
    setStore("fileSystem", files);
  }
};

export const invokeOpenFile = async (path: string) => {
  const exist = store.openedFile.find((item) => item.fileInfo.path == path);
  if (exist) {
    setStore("selectedFile", exist.fileInfo.path);
    return;
  }
  const file = await catchResponse(commands.openFile(path));
  if (file) {
    setStore("openedFile", (prev) => [
      ...prev,
      { fileInfo: file, editor: null },
    ]);
    setStore("selectedFile", file.path);
  }
};

export const changeSelected = (value: string) => {
  setStore("selectedFile", value);
};

export const invokeCloseFile = async (id: number) => {
  console.log("Closing file with id:", id);
  console.log("Opened files:", store.openedFile);

  let item = store.openedFile.find((item) => item.fileInfo.id === id);
  console.log("Found item:", item);

  if (item) {
    item.editor?.dispose();
    await catchResponse(commands.closeFile(id));
    setStore("openedFile", (prev) =>
      prev.filter((item) => item.fileInfo.id !== id)
    );
  } else {
    console.warn(`File with id ${id} not found in openedFile array.`);
  }
};

export const invokeReset = async () => {
  await catchResponse(commands.reset());
  setStore("openedFile", []);
};

export const invokeGetEditorConfig = async () => {
  let editorConfig = await catchResponse(commands.getEditorConfig());
  let languageTheme = editorConfig?.[0];
  let editorTheme = editorConfig?.[1];
  setStore("editorConfig", { editorTheme, languageTheme });
};

// @ts-ignore
export const getLanguageThemeIfAnyElseDefault = (language: string): Theme => {
  const languageTheme: LanguageTheme = store.editorConfig?.languageTheme!;
  //@ts-ignore
  if (languageTheme[language]) {
    // @ts-ignore
    return languageTheme[language];
  }
  return languageTheme.default;
};

export const invokeGetCurrentlySupportedLanguage = async () => {
  const res = await catchIfAny(commands.getCurrentlySupportedLanguage());
  if (res) setStore("supportedLanguage", res);
};

export { store };
