import "./App.css";
import FileExplorer from "./component/fileExplorer/fileExplorer";
import TextEditor from "./component/textEditor/textEditor";
import {
  invokeGetCurrentlySupportedLanguage,
  invokeGetEditorConfig,
} from "./backendApi/stateStore";

import { ParentComponent, onMount } from "solid-js";
import { Toast } from "@kobalte/core";
import { Portal } from "solid-js/web";
import Menu from "./component/menu/menu";

const App: ParentComponent = () => {
  onMount(() => {
    invokeGetEditorConfig();
    invokeGetCurrentlySupportedLanguage();
  });

  return (
    <>
      <Menu />
      <FileExplorer />
      <TextEditor />

      <Portal>
        <Toast.Region>
          <Toast.List class="toast__list" />
        </Toast.Region>
      </Portal>
    </>
  );
};

export default App;
