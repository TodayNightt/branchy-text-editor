import { createSignal } from "solid-js";
import "./App.css";
import FileExplorer from "./component/fileExplorer/fileExplorer";
import TextEditor from "./component/textEditor/textEditor";

import { ParentComponent } from "solid-js";

const App: ParentComponent = () => {
  const [path, setPath] = createSignal<string>("");
  return (
    <>
      <FileExplorer setPath={setPath} />
      <TextEditor path={path()} />
      <a href="https://www.flaticon.com/free-icons/ide" title="ide icons">
        Ide icons created by Flat Icons - Flaticon
      </a>
    </>
  );
};

export default App;
