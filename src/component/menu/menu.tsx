import { Button, DropdownMenu } from "@kobalte/core";
import { appWindow } from "@tauri-apps/api/window";
import { Component, JSX } from "solid-js";
// @ts-ignore
import styles from "./style.module.css";
import { TbArrowsMaximize } from "solid-icons/tb";
import { VsChromeClose, VsChromeMinimize } from "solid-icons/vs";
import { FaSolidFolder } from "solid-icons/fa";
import { catchIfAny } from "../../backendApi/invocation";
import {
  invokeChangeDir,
  invokeOpenFile,
  invokeReset,
} from "../../backendApi/stateStore";
import { open } from "@tauri-apps/api/dialog";
import About from "./component/about/about";
import {
  BiRegularArrowToRight,
  BiSolidFileBlank,
  BiSolidNote,
} from "solid-icons/bi";

const Menu: Component = () => {
  type ButtonMouseEvent = JSX.EventHandler<HTMLDivElement, MouseEvent>;
  const handleReset: ButtonMouseEvent = () => {
    invokeReset();
  };
  const handleOpenFolder = async () => {
    const path = (await catchIfAny(
      open({
        directory: true,
        multiple: false,
      })
    )) as string;

    if (path) {
      invokeChangeDir(path);
    }
  };

  const handleOpenFile = async () => {
    const path = (await catchIfAny(
      open({
        directory: false,
        multiple: false,
      })
    )) as string;
    if (path) {
      invokeOpenFile(path);
    }
  };
  return (
    <div data-tauri-drag-region class="menu">
      <div class={styles["panel"]}>
        <BiSolidNote class={styles["icon"]} size={"2rem"} />
        <DropdownMenu.Root>
          <DropdownMenu.Trigger class={styles["button"]}>
            <span>File</span>
          </DropdownMenu.Trigger>
          <DropdownMenu.Portal>
            <DropdownMenu.Content class={styles["dropdown-menu__content"]}>
              <DropdownMenu.Item
                onClick={handleOpenFile}
                class={styles["dropdown-menu__item"]}
              >
                Open File
                <div>
                  <BiSolidFileBlank />
                </div>
              </DropdownMenu.Item>
              <DropdownMenu.Item
                onClick={handleOpenFolder}
                class={styles["dropdown-menu__item"]}
              >
                Open Folder
                <div class={styles["float-right"]}>
                  <FaSolidFolder />
                </div>
              </DropdownMenu.Item>
            </DropdownMenu.Content>
          </DropdownMenu.Portal>
        </DropdownMenu.Root>
        <DropdownMenu.Root>
          <DropdownMenu.Trigger class={styles["button"]}>
            <span>Settings</span>
          </DropdownMenu.Trigger>
          <DropdownMenu.Portal>
            <DropdownMenu.Content class={styles["dropdown-menu__content"]}>
              <DropdownMenu.Item
                onClick={handleReset}
                class={styles["dropdown-menu__item"]}
              >
                Reset Backend
              </DropdownMenu.Item>
            </DropdownMenu.Content>
          </DropdownMenu.Portal>
        </DropdownMenu.Root>

        <DropdownMenu.Root>
          <DropdownMenu.Trigger class={styles["button"]}>
            <span>Help</span>
          </DropdownMenu.Trigger>
          <DropdownMenu.Portal>
            <DropdownMenu.Content class={styles["dropdown-menu__content"]}>
              <DropdownMenu.Sub overlap gutter={4} shift={-8}>
                <DropdownMenu.SubTrigger class={styles["dropdown-menu__item"]}>
                  About
                  <BiRegularArrowToRight />
                </DropdownMenu.SubTrigger>
                <DropdownMenu.SubContent>
                  <About />
                </DropdownMenu.SubContent>
              </DropdownMenu.Sub>
            </DropdownMenu.Content>
          </DropdownMenu.Portal>
        </DropdownMenu.Root>
      </div>
      <div class={styles["panel"]}>
        <Button.Root class={styles.button} onClick={appWindow.minimize}>
          <VsChromeMinimize />
        </Button.Root>
        <Button.Root
          class={styles.button}
          onClick={async () => {
            const isMaximize = await catchIfAny(appWindow.isMaximized());
            if (isMaximize) {
              appWindow.unmaximize();
            } else {
              appWindow.maximize();
            }
          }}
        >
          <TbArrowsMaximize />
        </Button.Root>

        <Button.Root class={styles.button} onClick={appWindow.close}>
          <VsChromeClose />
        </Button.Root>
      </div>
    </div>
  );
};

export default Menu;
