import { Component, createSignal, onMount } from "solid-js";
import {
  invokeGetSourceCode,
  invokeGetTokensLegend,
  invokeHandleFileChanges,
  invokeHighlights,
  invokeSaveFile,
} from "../../../backendApi/invocation";

import { getLanguageThemeIfAnyElseDefault } from "../../../backendApi/stateStore";
import { OpenFile } from "../../../backendApi/bindings";
import * as monaco from "monaco-editor/esm/vs/editor/editor.api";
// @ts-ignore
import styles from "./styles.module.scss";

const Editor: Component<{ fileInfo: OpenFile }> = (props) => {
  const fileInfo = props.fileInfo;
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
    _model?: monaco.editor.ITextModel
  ) => {
    switch (action) {
      case Action.Save:
        console.log("saving");
        await invokeSaveFile(props.fileInfo.id);
        break;
      default:
        console.log("Verify Not Reached");
    }
  };

  const handleChange = async (e: monaco.editor.IModelContentChangedEvent) => {
    let change = e.changes[0];

    const startLine = change.range.startLineNumber;
    const startColumn = change.range.startColumn;
    const endLine = change.range.endLineNumber;
    const endColumn = change.range.endColumn;

    const startByte = change.rangeOffset;
    const endByte = startByte + change.text.length;

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
      editor.getValue()!
    );

    setCurrentEndPosition({ row: endLine, col: endColumn });
  };

  let editorEl!: HTMLDivElement;
  let editor: monaco.editor.IStandaloneCodeEditor;

  onMount(async () => {
    const lang = language.toLowerCase();
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

    //Create A SemanticTokenProvider
    const legend = await invokeGetTokensLegend(language)!;
    monaco.languages.registerDocumentRangeSemanticTokensProvider(langId, {
      getLegend() {
        return {
          tokenTypes: legend!._token_types,
          tokenModifiers: [""],
        };
      },
      //@ts-ignore
      provideDocumentRangeSemanticTokens: async function (
        model,
        range,
        _token
      ) {
        const rangedSourceCode = model.getValueInRange(range);

        const data = await invokeHighlights(id, rangedSourceCode);

        for (let i = 3; i < data.length; i += 5) {
          console.log(
            `(${data[i - 3]},${data[i - 2]}) length: ${data[i - 1]} ${
              this.getLegend().tokenTypes[data[i]]
            }`
          );
        }

        return {
          data: new Uint32Array(data),
          resultId: null,
        };
      },
    });

    //Initialize the currentEndPosition for the tree-sitter parsing
    const range = editor.getModel()?.getFullModelRange();
    setCurrentEndPosition({
      row: range?.endLineNumber ? range.endLineNumber : 0,
      col: range?.endColumn ? range.endColumn : 0,
    });

    // Handle the on change event
    editor.onDidChangeModelContent((e) => handleChange(e));
  });
  return <div class={styles.editor} ref={editorEl}></div>;
};

export default Editor;
