import { Breadcrumbs } from "@kobalte/core";
import { Component, For, JSX, createEffect, createSignal } from "solid-js";
import { store, invokeChangeDir } from "../../../backendApi/stateStore";
// @ts-ignore
import styles from "./style.module.css";
import { BiRegularChevronsRight } from "solid-icons/bi";

type SimplifiedPath = {
  simplified: boolean;
  simplifiedPath: Array<string>;
  fullPath: Array<string>;
  currentIndex: number;
};

const DirectoryHelper: Component = () => {
  const [simplifiedPath, setSimplifiedPath] = createSignal<SimplifiedPath>({
    simplified: false,
    simplifiedPath: [],
    fullPath: [],
    currentIndex: 0,
  });
  const handleClick: JSX.EventHandler<HTMLAnchorElement, MouseEvent> = async (
    ev
  ) => {
    await invokeChangeDir(ev.currentTarget?.id);
  };

  createEffect(() => {
    let path = store.fileSystem.current_directory.split("\\");
    if (path.length > 4) {
      setSimplifiedPath({
        simplified: true,
        simplifiedPath: path.slice(path.length - 3),
        fullPath: path,
        currentIndex: path.length - 3,
      });
    } else {
      setSimplifiedPath({
        simplified: false,
        simplifiedPath: path,
        fullPath: path,
        currentIndex: 0,
      });
    }
  });

  return (
    <>
      <Breadcrumbs.Root
        class={styles["breadcrumbs"]}
        separator={<BiRegularChevronsRight size={"1rem"} />}
      >
        <ol class={styles["breadcrumbs__list"]}>
          {simplifiedPath().simplified && (
            <li class={styles["breadcrumbs__item"]}>
              <Breadcrumbs.Link
                id={simplifiedPath()
                  .fullPath.slice(0, simplifiedPath().fullPath.length - 4)
                  .join("\\")}
                onClick={handleClick}
              >
                ...
              </Breadcrumbs.Link>
              <Breadcrumbs.Separator />
            </li>
          )}
          <For each={simplifiedPath().simplifiedPath}>
            {(item, index) => (
              <li class={styles["breadcrumbs__item"]}>
                <Breadcrumbs.Link
                  id={simplifiedPath()
                    .fullPath.slice(
                      0,
                      simplifiedPath().currentIndex + index() + 1
                    )
                    .join("\\")}
                  onClick={handleClick}
                >
                  {item}
                </Breadcrumbs.Link>
                {index() !== simplifiedPath().simplifiedPath.length - 1 && (
                  <Breadcrumbs.Separator />
                )}
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
    </>
  );
};

export default DirectoryHelper;
