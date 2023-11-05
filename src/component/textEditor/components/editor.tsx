import { Component, createSignal, onMount } from "solid-js";
import {
  invokeGetSourceCode,
  invokeParseFile,
  invokeSaveFile,
} from "../../../stateStore";
import { ChangesRange, OpenFile } from "../../../bindings";
import * as monaco from "monaco-editor/esm/vs/editor/editor.api";
// @ts-ignore
import styles from "./styles.module.scss";

const Editor: Component<{ fileInfo: OpenFile }> = (props) => {
  let currentEndPosition: { row: number; col: number };
  const handleKey = async (model: monaco.editor.ITextModel) => {
    await invokeSaveFile(props.fileInfo.id, model.getValue());
  };

  const handleChange = (e: monaco.editor.IModelContentChangedEvent) => {
    let change = e.changes[0];

    const startLine = change.range.startLineNumber;
    const startColumn = change.range.startColumn;
    const endLine = change.range.endLineNumber;
    const endColumn = change.range.endColumn;

    const startByte = change.rangeOffset;
    const endByte = startByte + change.text.length;

    invokeParseFile(
      props.fileInfo.id,
      {
        start_byte: startByte,
        new_end_byte: endByte,
        old_end_byte: change.rangeOffset + change.rangeLength,
        start_position: { row: startLine, column: startColumn },
        old_end_position: {
          row: currentEndPosition.row,
          column: currentEndPosition.col,
        },
        new_end_position: { row: endLine, column: endColumn },
      },
      editor.getValue()!
    );

    currentEndPosition = { row: endLine, col: endColumn };
  };

  let editorEl!: HTMLDivElement;
  let editor: monaco.editor.IStandaloneCodeEditor;

  onMount(async () => {
    const lang = props.fileInfo.language?.toLowerCase();
    const defaultValue = await invokeGetSourceCode(props.fileInfo.id);
    if (editor) {
      editor = monaco.editor.create(editorEl, {
        value: defaultValue,
        language: lang,
        automaticLayout: true,
        theme: "vs",
      });
    }

    //Create an Action when pressing ctrl+s to save
    editor.addAction({
      id: "save",
      keybindings: [monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS],
      label: "",
      run: function (
        editor: monaco.editor.ICodeEditor,
        //@ts-ignore
        ...args: any[]
      ): void | Promise<void> {
        handleKey(editor.getModel()!);
      },
    });

    //Initialize the currentEndPosition for the tree-sitter parsing
    const range = editor.getModel()?.getFullModelRange();
    currentEndPosition = {
      row: range?.endLineNumber ? range.endLineNumber : 0,
      col: range?.endColumn ? range.endColumn : 0,
    };

    // Handle the on change event
    editor.onDidChangeModelContent((e) => handleChange(e));
  });
  return <div class={styles.editor} ref={editorEl}></div>;
};

export default Editor;
