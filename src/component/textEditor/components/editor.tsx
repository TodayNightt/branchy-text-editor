import { Component, createSignal, onCleanup, onMount } from "solid-js";
import {
  invokeGetSourceCode,
  invokeHandleFileChanges,
  invokeSaveFile,
} from "../../../backendApi/invocation";

import {
  OpenFileTab,
  getLanguageThemeIfAnyElseDefault,
} from "../../../backendApi/stateStore";
import * as monaco from "monaco-editor/esm/vs/editor/editor.api";
import { registerSemanticTokenProvider } from "./tokenProvider";
// @ts-ignore
import styles from "../styles.module.css";

const Editor: Component<{ tabs: OpenFileTab }> = (props) => {
  const fileInfo = props.tabs.fileInfo;
  let editor = props.tabs.editor;
  let registration: monaco.IDisposable | undefined;
  const id = fileInfo.id;
  const language = fileInfo.language!;
  const [currentEndPosition, setCurrentEndPosition] = createSignal({
    row: 0,
    col: 0,
  });

  const enum Action {
    Save,
  }

  const handleKey = async (
    action: Action,
    _editor?: monaco.editor.ICodeEditor
  ) => {
    switch (action) {
      case Action.Save:
        console.log("saving");
        await invokeSaveFile(fileInfo.id);
        break;
      default:
        console.log("Verify Not Reached");
    }
  };

  const handleChange = async (e: monaco.editor.IModelContentChangedEvent) => {
    let change = e.changes[0];

    const startLine =
      change.range.startLineNumber === 0 ? 0 : change.range.startLineNumber - 1;
    const startColumn =
      change.range.startColumn === 0 ? 0 : change.range.startColumn - 1;
    const endLine =
      change.range.endLineNumber === 0 ? 0 : change.range.endLineNumber - 1;
    const endColumn =
      change.range.endColumn === 0 ? 0 : change.range.endColumn - 1;

    const startByte = change.rangeOffset;
    const endByte = startByte + change.text.length;

    console.log(change);

    await invokeHandleFileChanges(
      id,
      {
        start_byte: startByte,
        new_end_byte: endByte,
        old_end_byte: change.rangeOffset + change.rangeLength,
        start_position: { row: startLine, column: startColumn },
        old_end_position: {
          row: currentEndPosition().row,
          column: currentEndPosition().col,
        },
        new_end_position: { row: endLine, column: endColumn },
      },
      editor?.getValue()!
    );

    setCurrentEndPosition({ row: endLine, col: endColumn });
  };

  let editorEl!: HTMLDivElement;

  onMount(async () => {
    let lang = "plaintext";
    if (language) {
      lang = language.toLowerCase();
    }
    //Create a Theme based on the get_theme_api
    const rules = getLanguageThemeIfAnyElseDefault(lang).rules;
    monaco.editor.defineTheme("custom", {
      base: "vs-dark",
      rules: rules,
      inherit: true,
      colors: {},
    });

    //Get the default value which the source file have
    const defaultValue = await invokeGetSourceCode(id);

    const langId = "custom-" + lang;
    monaco.languages.register({
      id: langId,
    });

    // Create a new instance of the editor
    if (!editor) {
      editor = monaco.editor.create(editorEl, {
        value: defaultValue,
        language: langId,
        automaticLayout: true,
        theme: "custom",
        "semanticHighlighting.enabled": true,
        folding: false,
        minimap: { enabled: false },
      });
    }

    //Create an Action when pressing ctrl+s to save
    editor.addAction({
      id: "save",
      keybindings: [monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS],
      label: "",
      run: function (
        //@ts-ignore
        editor: monaco.editor.ICodeEditor,
        //@ts-ignore
        ...args: any[]
      ): void | Promise<void> {
        handleKey(Action.Save);
      },
    });

    if (language) {
      registration = await registerSemanticTokenProvider(language, id, langId);
    }

    //Initialize the currentEndPosition for the tree-sitter parsing
    const range = editor.getModel()?.getFullModelRange();
    setCurrentEndPosition({
      row: range?.endLineNumber ? range.endLineNumber : 0,
      col: range?.endColumn ? range.endColumn : 0,
    });

    // Handle the on change event
    editor.onDidChangeModelContent((e) => handleChange(e));
  });
  onCleanup(() => {
    editor?.getModel()?.dispose();
    registration?.dispose();
  });
  return <div class={styles.editor} ref={editorEl}></div>;
};

export default Editor;
