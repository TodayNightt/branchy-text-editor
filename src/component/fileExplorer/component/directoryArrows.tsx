import { Breadcrumbs, Button } from "@kobalte/core";
import { Component, For, JSX, createComputed } from "solid-js";
import { store, invokeChangeDir } from "../../../stateStore";
// @ts-ignore
import styles from "./style.module.scss";

const DirectoryHelper: Component = () => {
  const handleClick: JSX.EventHandler<HTMLAnchorElement, MouseEvent> = async (
    ev
  ) => {
    await invokeChangeDir(ev.currentTarget?.id);
  };
  return (
    <div>
      <Breadcrumbs.Root separator="&gt;">
        <ol class={styles["breadcrumbs__list"]}>
          <For each={store.fileSystem.current_directory.split("\\")}>
            {(item, index) => (
              <li class={styles["breadcrumbs__item"]}>
                <Breadcrumbs.Link
                  id={store.fileSystem.current_directory
                    .split("\\")
                    .slice(0, index() + 1)
                    .join("\\")}
                  onClick={handleClick}
                >
                  {item}
                </Breadcrumbs.Link>
                {index() !=
                store.fileSystem.current_directory.split("\\").length - 1 ? (
                  <Breadcrumbs.Separator />
                ) : null}
              </li>
            )}
          </For>
        </ol>
      </Breadcrumbs.Root>
      {/* {store.fileSystem.current_directory.split("\\").map((item, index) => (
        <>
          <Button.Root
            class={styles.button}
            id={item}
            value={store.fileSystem.current_directory
              .split("\\")
              .slice(0, index + 1)
              .join("\\")}
            onClick={handleClick}
          >
            {item}
          </Button.Root>
          {index !==
          store.fileSystem.current_directory.split("\\").length - 1 ? (
            <Button.Root>&gt;</Button.Root>
          ) : null}
        </>
      ))} */}
    </div>
  );
};

export default DirectoryHelper;
