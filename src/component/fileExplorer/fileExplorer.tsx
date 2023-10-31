import { Component, For, JSX, onMount } from "solid-js";
import { Button, Collapsible } from "@kobalte/core";

import DirectoryHelper from "./component/directoryArrows.tsx";

// @ts-ignore
import styles from "./style.module.scss";
import { invokeChangeDir, invokeOpenFile, store } from "../../stateStore.ts";
import { DirectoryItem } from "../../bindings.ts";

type EventHandler = JSX.EventHandler<HTMLButtonElement, MouseEvent>;

const File: Component<{
  item: DirectoryItem;
  handleClick: EventHandler;
}> = (props) => {
  return (
    <Button.Root
      class={styles["button"]}
      value={props.item.path}
      onClick={props.handleClick}
    >
      {props.item.name}
    </Button.Root>
  );
};

const Directory: Component<{
  item: DirectoryItem;
  handleClick: EventHandler;
}> = (props) => {
  return (
    <Collapsible.Root class={styles.collapsible}>
      <Collapsible.Trigger class={styles["collapsible__trigger"]}>
        {props.item.name}
      </Collapsible.Trigger>
      <Collapsible.Content class={styles.container}>
        <For each={props.item.childrens}>
          {(innerItem) =>
            innerItem.is_file ? (
              <File item={innerItem} handleClick={props.handleClick} />
            ) : (
              <Directory item={innerItem} handleClick={props.handleClick} />
            )
          }
        </For>
      </Collapsible.Content>
    </Collapsible.Root>
  );
};

const FileExplorer: Component = () => {
  onMount(() => invokeChangeDir("."));

  const handleClick: EventHandler = (evt) => {
    invokeOpenFile(evt.currentTarget?.value);
  };

  return (
    <div class={styles.container}>
      <DirectoryHelper />
      <For each={store.fileSystem.directory_items} fallback={null}>
        {(item) =>
          item.is_file ? (
            <File item={item} handleClick={handleClick} />
          ) : (
            <Directory item={item} handleClick={handleClick} />
          )
        }
      </For>
    </div>
  );
};

export default FileExplorer;
