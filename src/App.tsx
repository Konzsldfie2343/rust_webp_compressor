import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { listen } from "@tauri-apps/api/event";
import { motion } from "framer-motion";
import "./App.css";

interface checkbox {
  text: string,
  value: boolean,
  setValue: (value: boolean) => void
}

const Checkbox = ({ text, value, setValue }: checkbox) => {
  const className = value ? "checkbox-checked" : "checkbox-unchecked";
  return (
    <label>
      <motion.button
        whileHover={{ scale: 1.1 }}
        whileTap={{ scale: 0.9 }}
        className={className} onClick={() => setValue(!value)}></motion.button>
      <motion.span
        whileHover={{ textShadow: "0 0 3px white", transition: { duration: 0.3 } }}
        whileTap={{ textShadow: "0 0 3px white", scale: 0.97, transition: { duration: 0.3 } }}
      >{text}</motion.span>
    </label>
  )
}

const select_folder = async ({ setPath }: { setPath: (path: string) => void }) => {
  try {
    const selectedPath = await open({ directory: true });
    if (typeof selectedPath === "string") {
      setPath(selectedPath);
    }
  } catch (error) {
    console.error("Error selecting folder:", error);
  }
}

interface run_convert_props {
  path: string,
  ratio: number,
  isReplace: boolean,
  isRecursive: boolean,
  setResult: (result: string) => void
}

const run_convert = async ({ path, ratio, isReplace, isRecursive, setResult }: run_convert_props) => {
  try {
    setResult("変換中...");
    const response = await invoke<string>("convert_to_webp", { path, isReplace, isRecursive, ratio });
    setResult(response);
  } catch (error) {
    setResult(String(error));
  }
}

const MenuButton = ({ text, onClick }: { text: string, onClick: () => void }) => {
  return (
    <motion.button
      whileHover={{ scale: 1.05, transition: { duration: 0.3 } }}
      whileTap={{ scale: 0.95, transition: { duration: 0.3 } }}
      className="menu-buttons" onClick={onClick}>{text}</motion.button>
  )
}

const RangeSlider = ({ value, setValue }: { value: number, setValue: (value: number) => void }) => {

  const comments = [
    "☆推奨　サイズ減少量は少なめですが、画質の劣化が少なくなります",
    "サイズ減少量は中程度ですが、画質の劣化がやや多くなります",
    "△非推奨　サイズ減少量は大きいですが、画質の劣化が大きくなります"
  ]

  const [comment, setComment] = useState<string>("");
  if (value >= 85) {
    if (comment !== comments[0]) setComment(comments[0])
  } else if (value >= 50) {
    if (comment !== comments[1]) setComment(comments[1])
  } else {
    if (comment !== comments[2]) setComment(comments[2])
  }
  return (
    <div className="checkbox-container">
      <div className="range-value">圧縮率：{value}%</div>
      <input type="range" min="0" max="100" step="1" value={value} onChange={(e) => setValue(Number(e.target.value))} />
      <div className="range-value">{comment}</div>
    </div>
  )
}

const App = (): JSX.Element => {
  const [path, setPath] = useState<string>("");
  const [result, setResult] = useState<string>("");
  const [ratio, setRatio] = useState<number>(85);
  const [isReplace, setIsReplace] = useState<boolean>(false);
  const [isRecursive, setIsRecursive] = useState<boolean>(false);
  const [status, setStatus] = useState<string>("normal");

  useEffect(() => {
    const dragarea = document.getElementById("dragarea");
    if (!dragarea) return;

    const dragover_handler = (e: DragEvent) => {
      e.preventDefault();
    }

    const drop_handler = (e: DragEvent) => {
      e.preventDefault();
    }

    dragarea.addEventListener("dragover", (e) => {
      dragover_handler(e);
    })
    dragarea.addEventListener("drop", (e) => {
      drop_handler(e);
    });

    return () => {
      dragarea.removeEventListener("dragover", dragover_handler);
      dragarea.removeEventListener("drop", drop_handler);
    }
  }, []);

  useEffect(() => {
    const unlisten = listen<string[]>("tauri://file-drop", (event) => {
      if (event.payload.length > 0 && status == "normal") {
        setPath(event.payload[0]);
      }
    })
    return () => { unlisten.then(fn => fn()) }
  }, [])

  return (
    <div className="App">
      <div className="container" style={{ flex: 2 }}>
        <p></p>
        <h1>画像を自動で軽量化</h1>
        <div style={{ display: "none" }}>
          <RangeSlider value={ratio} setValue={setRatio} />
        </div>
        <div className="checkbox-container">
          <Checkbox text="変換後に元の画像を置き換える" value={isReplace} setValue={setIsReplace} />
          <Checkbox text="すべてのフォルダを対象にする" value={isRecursive} setValue={setIsRecursive} />
        </div>
        <p>　{result}　</p>
      </div>
      <div id="dragarea" className="container" style={{ flex: 1, height: "95vh", margin: "10px" }}>
        <p style={{ padding: "10px" }}>{path ? ("選択されたフォルダ: " + path) : "ドロップしてください"}</p>
        <div className="menu-container">
          <MenuButton text="再読み込み" onClick={() => window.location.reload()} />
          <MenuButton text="選択する" onClick={() => select_folder({ setPath })} />
          <MenuButton text="変換実行" onClick={() => run_convertion_tasks({ path, ratio, isReplace, isRecursive, setResult, status, setStatus })} />
        </div>
      </div>
    </div>
  )
}

interface run_convertion_tasks_props {
  path: string,
  ratio: number,
  isReplace: boolean,
  isRecursive: boolean,
  setResult: (result: string) => void,
  status: string,
  setStatus: (value: string) => void
}

const run_convertion_tasks = ({ path, ratio, isReplace, isRecursive, setResult, status, setStatus }: run_convertion_tasks_props) => {
  if (status == "normal") {
    setStatus("converting");
    run_convert({ path, ratio, isReplace, isRecursive, setResult })
      .then(() => setStatus("normal"));
  }
}

export default App