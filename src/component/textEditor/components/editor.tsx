import { Component, createSignal, onMount } from "solid-js";
import { invokeGetSourceCode } from "../../../stateStore";
import { OpenFile } from "../../../bindings";
import * as monaco from "monaco-editor/esm/vs/editor/editor.api";
// @ts-ignore
import styles from "./styles.module.scss";

const Editor: Component<{ fileInfo: OpenFile }> = (props) => {
  let editorEl!: HTMLDivElement;
  const [editor, setEditor] =
    createSignal<monaco.editor.IStandaloneCodeEditor | null>(null);
  onMount(async () => {
    // if (editorEl) {
    let lang = props.fileInfo.language?.toLowerCase();
    setEditor((editor) => {
      if (editor) return editor;
      return monaco.editor.create(editorEl, {
        language: lang,
        automaticLayout: true,
        // theme: "vs-dark",
      });
    });
    // }
    editor()?.setValue(await invokeGetSourceCode(props.fileInfo.id));
    console.log(editor()?.getValue());
    // editor()?.onDidChangeModelContent((e) => console.log(e));
  });
  return <div class={styles.editor} ref={editorEl}></div>;
};

export default Editor;
