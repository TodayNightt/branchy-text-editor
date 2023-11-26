import { Component, For, JSX, onMount } from "solid-js";
import { Button, Collapsible } from "@kobalte/core";

import DirectoryHelper from "./component/directoryArrows.tsx";

// @ts-ignore
import styles from "./style.module.css";
import {
  invokeChangeDir,
  invokeOpenFile,
  store,
} from "../../backendApi/stateStore.ts";
import { DirectoryItem } from "../../backendApi/bindings.ts";
import { BiRegularChevronRight } from "solid-icons/bi";

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
      <div class={styles["ellipsis-div"]}>{props.item.name}</div>
    </Button.Root>
  );
};

const Directory: Component<{
  item: DirectoryItem;
  handleClick: EventHandler;
}> = (props) => {
  return (
    <Collapsible.Root class={styles["collapsible"]}>
      <Collapsible.Trigger class={styles["button"]}>
        <div class={styles["ellipsis-div"]}>
          <BiRegularChevronRight class={styles["trigger-icon"]} />
          {props.item.name}
        </div>
      </Collapsible.Trigger>
      <Collapsible.Content class={styles["folder-content"]}>
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
  onMount(() => invokeChangeDir("../../test_home"));

  const handleClick: EventHandler = (evt) => {
    invokeOpenFile(evt.currentTarget?.value);
  };

  return (
    <div class="file-explorer">
      <div class={styles["directory-path"]}>
        <DirectoryHelper />
      </div>
      <div class={styles["directory-list"]}>
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
    </div>
  );
};

export default FileExplorer;
