import { Component, For, JSX } from "solid-js";
// @ts-ignore
import styles from "./styles.module.css";
import "./userWorker";

import {
  store,
  changeSelected,
  invokeCloseFile,
} from "../../backendApi/stateStore";
import { Button, Tabs } from "@kobalte/core";
import Editor from "./components/editor";
import { VsChromeClose } from "solid-icons/vs";

type ButtonMouseEvent = JSX.EventHandler<HTMLButtonElement, MouseEvent>;

const TextEditor: Component = () => {
  const closeFile: ButtonMouseEvent = (evt) => {
    const id = parseInt(evt.currentTarget?.id);
    invokeCloseFile(id);
  };

  return (
    <div class="text-editor">
      <Tabs.Root
        value={store.selectedFile}
        onChange={changeSelected}
        class={styles["tabs"]}
      >
        <Tabs.List class={styles["tab-list"]}>
          <For each={store.openedFile}>
            {(file) => (
              <Tabs.Trigger
                class={styles["tab-trigger"]}
                value={file.fileInfo.path}
              >
                <div class={styles["tab-name"]}>
                  <div>{file.fileInfo.name}</div>
                  <Button.Root
                    id={file.fileInfo.id.toString()}
                    onClick={closeFile}
                    class={styles["close-tab-btn"]}
                  >
                    <VsChromeClose />
                  </Button.Root>
                </div>
              </Tabs.Trigger>
            )}
          </For>
          <Tabs.Indicator />
        </Tabs.List>
        <For each={store.openedFile}>
          {(file) => (
            <Tabs.Content
              value={file.fileInfo.path}
              class={styles["tab-content"]}
            >
              <Editor tabs={file} />
            </Tabs.Content>
          )}
        </For>
      </Tabs.Root>
    </div>
  );
};

export default TextEditor;
