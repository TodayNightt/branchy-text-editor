import { Component } from "solid-js";
//@ts-ignore
import styles from "../../style.module.css";

const AboutMenu: Component = () => {
  return (
    <div class={styles["about-menu-div"]}>
      <h3>branchy text editor</h3>
      <h4>A text editor build from tauri and solidjs</h4>
      <div class={styles["attribute"]}>
        App Icon by :
        <ul>
          <li>
            <a
              href="https://www.flaticon.com/free-icons/branch"
              title="branch icons"
            >
              Branch icons created by Good Ware - Flaticon
            </a>
          </li>
          <li>
            <a href="https://github.com/atisawd/boxicons">BoxIcons</a>
          </li>
          <li>
            <a href="https://github.com/FortAwesome/Font-Awesome">
              Font-Awesome
            </a>
          </li>
          <li>
            <a href="https://github.com/tabler/tabler-icons">
              Tabler Icons (MIT)
            </a>
          </li>
          <li>
            <a href="https://github.com/microsoft/vscode-codicons">
              VS Code Icons
            </a>
          </li>
        </ul>
      </div>
    </div>
  );
};

export default AboutMenu;
