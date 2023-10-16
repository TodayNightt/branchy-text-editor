// import * as commands from "../../bindings.ts";
import { invoke } from "@tauri-apps/api";
import { Component, JSX, createEffect, createSignal } from "solid-js";
// @ts-ignore
import styles from "./editor.module.scss";

const TextEditor: Component<{ path: string }> = (props) => {
  const [lines, setLines] = createSignal<Array<string>>([]);

  createEffect(async () => {
    await invoke("open_file", { path: props.path })
      .then((res) => console.log(res))
      .catch((e) => setLines([e.toString()]));
  }, [props.path]);

  const handleType: JSX.EventHandler<HTMLDivElement, KeyboardEvent> = (evt) => {
    console.log(evt);
    const result = evt.target.id.match(/(\d+)/);
    if (!result) {
      return;
    }
    const index = Number.parseInt(result[0]);
    const line = evt.currentTarget.innerText;
    let arr = lines();
    arr[index] = line;
    console.log(arr);
    setLines(arr);
  };
  return (
    <div class={styles.container} contenteditable={true}>
      {lines().map((value, index) => (
        <div class={styles["lines"]} id={"line" + index} onkeydown={handleType}>
          <p contentEditable={false}>{index + 1}</p>
          {value}
        </div>
      ))}
    </div>
  );
};

export default TextEditor;
