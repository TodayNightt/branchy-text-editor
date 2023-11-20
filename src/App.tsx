import "./App.css";
import FileExplorer from "./component/fileExplorer/fileExplorer";
import TextEditor from "./component/textEditor/textEditor";
import {
  invokeGetCurrentlySupportedLanguage,
  invokeGetEditorConfig,
} from "./backendApi/stateStore";

import { ParentComponent, onMount } from "solid-js";
import { Button, Toast } from "@kobalte/core";
import { Portal } from "solid-js/web";
import { showToast } from "./component/notification_toast/toast";

const App: ParentComponent = () => {
  onMount(() => {
    invokeGetEditorConfig();
    invokeGetCurrentlySupportedLanguage();
  });

  return (
    <>
      <FileExplorer />
      <TextEditor />

      {/* <a href="https://www.freepik.com/icon/magic-wand_2145127">
        Icon by ultimatearm
      </a> */}

      <Button.Root onClick={showToast}>A</Button.Root>
      <Portal>
        <Toast.Region>
          <Toast.List class="toast__list" />
        </Toast.Region>
      </Portal>
    </>
  );
};

export default App;
