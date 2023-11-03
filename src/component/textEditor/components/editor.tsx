import { Component, createSignal, onMount } from "solid-js";
import { invokeGetSourceCode, invokeSaveFile } from "../../../stateStore";
import { OpenFile } from "../../../bindings";
import * as monaco from "monaco-editor/esm/vs/editor/editor.api";
// @ts-ignore
import styles from "./styles.module.scss";

const Editor: Component<{ fileInfo: OpenFile }> = (props) => {
  const handleKey = async (model: monaco.editor.ITextModel) => {
    await invokeSaveFile(props.fileInfo.id, model.getValue());
  };

  const handleChange = (e: monaco.editor.IModelContentChangedEvent) => {
    console.log(e.changes.length);
  };

  let editorEl!: HTMLDivElement;
  const [editor, setEditor] =
    createSignal<monaco.editor.IStandaloneCodeEditor | null>(null);

  onMount(async () => {
    const lang = props.fileInfo.language?.toLowerCase();
    const defaultValue = await invokeGetSourceCode(props.fileInfo.id);
    setEditor((editor) => {
      if (editor) return editor;
      return monaco.editor.create(editorEl, {
        value: defaultValue,
        language: lang,
        automaticLayout: true,
        theme: "vs",
      });
    });
    editor()?.addAction({
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

    editor()?.onDidChangeModelContent((e) => handleChange(e));
    // editor().
  });
  return <div class={styles.editor} ref={editorEl}></div>;
};

export default Editor;
