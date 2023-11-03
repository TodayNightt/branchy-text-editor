import "./App.css";
import FileExplorer from "./component/fileExplorer/fileExplorer";
import TextEditor from "./component/textEditor/textEditor";

import { ParentComponent } from "solid-js";

const App: ParentComponent = () => {
  return (
    <>
      <FileExplorer />
      <TextEditor />

      {/* <a href="https://www.freepik.com/icon/magic-wand_2145127">
        Icon by ultimatearm
      </a> */}
    </>
  );
};

export default App;
