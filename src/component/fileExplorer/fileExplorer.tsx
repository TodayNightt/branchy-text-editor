import { Component, JSX, Setter, createSignal, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api";
import { Button, Collapsible } from "@kobalte/core";
import { store, setStore } from "../../stateStore.ts";
// @ts-ignore
import styles from "./style.module.scss";

interface DirFile {
  name: string;
  path: string;
  children?: Array<DirFile>;
}

const FileExplorer: Component<{ setPath: Setter<string> }> = (props) => {
  const [currentDirectoryItems, setCurrentDirectoryItems] = createSignal<
    Array<DirFile>
  >([], {
    equals: false,
  });

  onMount(async () => {
    setCurrentDirectoryItems(await invoke("get_current_dir_items"));
    console.log(currentDirectoryItems());
  });

  const handleClick: JSX.EventHandler<HTMLButtonElement, MouseEvent> = (
    evt
  ) => {
    props.setPath(evt.currentTarget.id);
  };

  return (
    <div class={styles.container}>
      {currentDirectoryItems().map((value) =>
        value.children ? (
          <Collapsible.Root class={styles["current-dir"]}>
            <Collapsible.Trigger class={styles["current-dir"]}>
              {value.name}
            </Collapsible.Trigger>
            <Collapsible.Content>
              <div class={styles.container}>
                {value.children.map((value) => (
                  <Button.Root
                    class={styles["next-dir"]}
                    id={value.path}
                    onClick={handleClick}
                  >
                    {value.name}
                  </Button.Root>
                ))}
              </div>
            </Collapsible.Content>
          </Collapsible.Root>
        ) : (
          <Button.Root
            id={value.path}
            class={styles["current-dir"]}
            onClick={handleClick}
          >
            {value.name}
          </Button.Root>
        )
      )}
      {/* 
      <p>{state.name}</p> */}
    </div>
  );
};

export default FileExplorer;
