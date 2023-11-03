import { Component, For, JSX } from "solid-js";
// @ts-ignore
import styles from "./styles.module.scss";

import {
  store,
  changeSelected,
  invokeCloseFile,
  invokeReset,
} from "../../stateStore";
import { Button, Tabs } from "@kobalte/core";
import Editor from "./components/editor";

type ButtonMouseEvent = JSX.EventHandler<HTMLButtonElement, MouseEvent>;

const TextEditor: Component = () => {
  const closeFile: ButtonMouseEvent = (evt) => {
    const id = parseInt(evt.currentTarget?.id);
    invokeCloseFile(id);
  };

  const handleReset: ButtonMouseEvent = () => {
    invokeReset();
  };

  return (
    <div class={styles.container}>
      <div id={styles["reset-btn-div"]}>
        <Button.Root onClick={handleReset} id={styles["reset-btn"]}>
          Reset
        </Button.Root>
      </div>
      <Tabs.Root value={store.selectedFile} onChange={changeSelected}>
        <Tabs.List>
          <For each={store.openedFile}>
            {(file) => (
              <Tabs.Trigger value={file.name}>
                <div>
                  <div>{file.same_name_exist ? file.path : null}</div>
                  {file.name}
                  <Button.Root id={file.id.toString()} onClick={closeFile}>
                    &#9747;
                  </Button.Root>
                </div>
              </Tabs.Trigger>
            )}
          </For>
          <Tabs.Indicator />
        </Tabs.List>
        <For each={store.openedFile}>
          {(file) => (
            <Tabs.Content value={file.name}>
              <Editor fileInfo={file} />
            </Tabs.Content>
          )}
        </For>
      </Tabs.Root>
    </div>
  );
};

export default TextEditor;
